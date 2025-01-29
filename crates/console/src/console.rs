use cli_error::CliError;
use modules::xcb::XcbModule;
use modules::{Module, XcbKeyModule};
use rpc::RpcClient;
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::Editor;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::debug;
use types::account::Accounts;
use types::ResponseView;

use crate::base::{base_functions, BaseFunctions};

pub struct Console<W: Write> {
    modules: HashMap<String, Box<dyn Module>>,
    base_functions: BaseFunctions,
    client: Arc<Mutex<dyn RpcClient + Send>>,
    datadir: PathBuf,
    writer: W,
    editor: Editor<(), FileHistory>,
}

impl<W: Write> Console<W> {
    pub async fn new(
        client: Arc<Mutex<dyn RpcClient + Send>>,
        datadir: PathBuf,
        writer: W,
        editor: Editor<(), FileHistory>,
    ) -> Self {
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
            writer,
            editor,
        }
    }

    pub async fn run(&mut self) {
        // create history file if not exists
        if !std::path::Path::new(&self.history_file()).exists() {
            std::fs::File::create(self.history_file()).unwrap();
        }
        if self.editor.load_history(&self.history_file()).is_err() {
            self.write("No previous history.");
        }
        self.write("Welcome to the Core Blockchain Console");
        self.write(&format!(
            "Working data directory: {}",
            self.datadir.display()
        ));
        self.write(&format!(
            "Current network_id: {}",
            self.client.lock().await.get_network_id().await.unwrap()
        ));
        self.write("Type 'list' to see available modules and functions that can be executed");
        self.write("Type 'exit' or press Ctrl+C to exit the console");

        loop {
            let readline = self.editor.readline(">> ");
            match readline {
                Ok(line) => {
                    if line.is_empty() {
                        continue;
                    }
                    self.editor.add_history_entry(line.as_str()).unwrap();
                    self.editor.save_history(&self.history_file()).unwrap();

                    match self.evaluate(line).await {
                        Ok(result) => self.write(&result.to_string()),
                        Err(err) => self.write(&format!("Error: {}", err)),
                    }
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                    break;
                }
                Err(err) => {
                    self.write(format!("Error: {:?}", err).as_str());
                    break;
                }
            }
        }
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
            if let Ok(view) = ResponseView::from_str(last_arg) {
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

    fn write(&mut self, message: &str) {
        writeln!(self.writer, "{}", message).unwrap();
    }

    fn history_file(&self) -> String {
        self.datadir.display().to_string() + "/history.txt"
    }
}
