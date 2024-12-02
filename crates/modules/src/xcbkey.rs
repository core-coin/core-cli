use cli_error::CliError;
use rpc::RpcClient;
use std::sync::Arc;
use tokio::sync::Mutex;
use types::response::Response;

use crate::Module;

pub struct XcbKeyModule {
    client: Arc<Mutex<dyn RpcClient + Send>>,
}

impl XcbKeyModule {
    pub fn new(client: Arc<Mutex<dyn RpcClient + Send>>) -> Self {
        XcbKeyModule { client }
    }

    async fn client(&self) -> Arc<Mutex<dyn RpcClient + Send>> {
        self.client.clone()
    }
}

#[async_trait::async_trait]
impl Module for XcbKeyModule {
    async fn execute(&mut self, command: String, args: Vec<String>) -> Result<Response, CliError> {
        if args.is_empty() {
            return Ok(types::response::Response::String(
                "No command provided for module xcbkey".to_string(),
            ));
        }

        match command.as_str() {
            _ => Err(CliError::UnknownCommand),
        }
    }
}
