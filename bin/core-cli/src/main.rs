use cli::Cli;
use cli_error::CliError;
use console::Console;
use rpc::{go_core::GoCoreClient, RpcClient};
use rustyline::DefaultEditor;
use std::sync::Arc;
use structopt::StructOpt;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), CliError> {
    tracing_subscriber::fmt::init();

    let args = Cli::from_args();
    let client: Arc<Mutex<dyn RpcClient + Send>> = match args.client.as_str() {
        "go-core" => Arc::new(Mutex::new(GoCoreClient::new(args.backend.to_string()))),
        _ => return Err(CliError::UnknownClient(args.client)),
    };

    // create datadir if not exists
    if !std::path::Path::new(&args.get_datadir()).exists() {
        std::fs::create_dir_all(args.get_datadir())?;
    }
    let stdout = std::io::stdout();
    let editor = DefaultEditor::new().unwrap();

    let mut console = Console::new(client, args.get_datadir(), stdout, editor).await;
    console.run().await;

    Ok(())
}
