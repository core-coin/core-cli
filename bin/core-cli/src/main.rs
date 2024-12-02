use cli::Cli;
use cli_error::CliError;
use console::Console;
use rpc::{go_core::GoCoreClient, RpcClient};
use std::sync::Arc;
use structopt::StructOpt;
use tokio::sync::Mutex;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), CliError> {
    tracing_subscriber::fmt::init();

    let args = Cli::from_args();
    let client: Arc<Mutex<dyn RpcClient + Send>> = match args.client.as_str() {
        "go-core" => Arc::new(Mutex::new(GoCoreClient::new(args.backend))),
        _ => return Err(CliError::UnknownClient(args.client)),
    };

    let mut console = Console::new(client);
    console.run().await;

    Ok(())
}
