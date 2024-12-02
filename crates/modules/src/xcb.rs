use cli_error::CliError;
use rpc::RpcClient;
use std::sync::Arc;
use tokio::sync::Mutex;
use types::response::Response;

use crate::Module;

pub struct XcbModule {
    client: Arc<Mutex<dyn RpcClient + Send>>,
}

impl XcbModule {
    pub fn new(client: Arc<Mutex<dyn RpcClient + Send>>) -> Self {
        XcbModule { client }
    }

    async fn client(&self) -> Arc<Mutex<dyn RpcClient + Send>> {
        self.client.clone()
    }

    pub async fn block_height(&self) -> Result<Response, CliError> {
        match self.client.lock().await.get_block_height().await {
            Ok(height) => Ok(Response::U64(height)),
            Err(e) => Err(e),
        }
    }

    pub async fn block(&self, args: Vec<String>) -> Result<Response, CliError> {
        if args.len() != 1 {
            return Err(CliError::InvalidNumberOfArguments(1, "".to_string()));
        }
        let arg = &args[0];
        if arg == "latest" {
            match self.client().await.lock().await.get_block_latest().await {
                Ok(block) => Ok(Response::Block(block)),
                Err(e) => Err(e),
            }
        } else if arg.len() == 64 {
            // Handle 64-character string
            // Assuming it's a block hash
            match self
                .client()
                .await
                .lock()
                .await
                .get_block_by_hash(arg.to_string())
                .await
            {
                Ok(block) => Ok(Response::Block(block)),
                Err(e) => Err(e),
            }
        } else if let Ok(block_number) = arg.parse::<u64>() {
            // Handle string that can be converted to u64
            match self
                .client()
                .await
                .lock()
                .await
                .get_block_by_number(block_number)
                .await
            {
                Ok(block) => Ok(Response::Block(block)),
                Err(e) => Err(e),
            }
        } else {
            Err(CliError::InvalidArgument(
                arg.clone(),
                "number, block hash or `latest`".to_string(),
            ))
        }
    }

    pub async fn get_energy_price(&self) -> Result<Response, CliError> {
        let price = self.client().await.lock().await.get_energy_price().await;
        match price {
            Ok(price) => Ok(Response::U128(price)),
            Err(e) => Err(e),
        }
    }
}

#[async_trait::async_trait]
impl Module for XcbModule {
    async fn execute(&mut self, command: String, args: Vec<String>) -> Result<Response, CliError> {
        match command.as_str() {
            "get_block_height" => self.block_height().await,
            "get_block" => self.block(args).await,
            "get_energy_price" => self.get_energy_price().await,
            _ => Err(CliError::UnknownCommand),
        }
    }
}
