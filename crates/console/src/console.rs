use cli_error::CliError;
use modules::xcb::XcbModule;
use modules::{Module, XcbKeyModule};
use rpc::RpcClient;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Editor};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};
use types::account::Accounts;
use types::{Account, Response, ResponseView};

use crate::base::{base_functions, BaseFunctions};

pub struct Console {
    modules: HashMap<String, Box<dyn Module>>,
    base_functions: BaseFunctions,
    client: Arc<Mutex<dyn RpcClient + Send>>,
    datadir: String,
    accounts: Accounts,
}

impl Console {
    pub async fn new(client: Arc<Mutex<dyn RpcClient + Send>>, datadir: String) -> Self {
        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
        let accounts = Accounts::new(vec![]);

        modules.insert("xcb".to_string(), Box::new(XcbModule::new(client.clone())));
        modules.insert(
            "xcbkey".to_string(),
            Box::new(XcbKeyModule::new(client.clone(), datadir.clone(), accounts.clone()).await),
        );

        Console {
            modules,
            client,
            base_functions: base_functions(),
            datadir,
            accounts,
        }
    }

    pub async fn run(&mut self) {
        let mut rl = DefaultEditor::new().unwrap();
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }

        info!("Welcome to the Core Blockchain Console");
        info!("Working data directory: {}", self.datadir);
        info!(
            "Current network_id: {}",
            self.client.lock().await.get_network_id().await.unwrap()
        );
        info!("Type 'list' to see available modules and functions that can be executed");
        info!("Type 'exit' or press Ctrl+C to exit the console");

        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str()).unwrap();

                    match self.evaluate(line).await {
                        Ok(result) => println!("{}", result),
                        Err(err) => eprintln!("Error: {}", err),
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }

        rl.save_history("history.txt").unwrap();
    }

    // Default command format: module.function(arg1,arg2,...argN)
    // Example: xcb.block_height()
    // Example: xcbkey.get_block(1)
    // At first, we split the line into module and function.
    // Then we split the function into function name and arguments.
    async fn evaluate(&mut self, line: String) -> Result<String, CliError> {
        if let Some(predefined) = self.base_functions.get(&line) {
            predefined();
            return Ok("".to_string());
        }

        let (module_name, function) = line.split_once(".").ok_or(CliError::UnknownCommand)?;

        let (function_name, mut args) = match function.split_once("(") {
            Some((name, args)) => (
                name,
                args.trim_end_matches(")")
                    .split(",")
                    .map(str::trim)
                    .map(|s| s.replace("\"", ""))
                    .map(|s| s.replace("\'", ""))
                    .filter(|s| !s.is_empty())
                    .collect(),
            ),
            None => (function, Vec::new()),
        };

        debug!(
            "Module: {}, Function: {}, Args: {:?}",
            module_name, function_name, args
        );

        let response_view = if let Some(last_arg) = args.last() {
            if let Some(view) = ResponseView::from_str(last_arg) {
                args.pop(); // Remove the last argument if it's a valid ResponseView
                view
            } else {
                ResponseView::default()
            }
        } else {
            ResponseView::default()
        };

        if let Some(module) = self.modules.get_mut(module_name) {
            let response = module.execute(function_name.to_string(), args).await?;
            // Apply the response view to the response
            Ok(response.format(response_view))
        } else {
            Err(CliError::UnknownModule(module_name.to_string()))
        }
    }
}
