use atoms_rpc_types::BlockId;
use base_primitives::{hex::FromHex, B256};
use cli_error::CliError;
use rpc::RpcClient;
use std::sync::Arc;
use tokio::sync::Mutex;
use types::Response;

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

    async fn block_height(&self) -> Result<Response, CliError> {
        match self.client.lock().await.get_block_height().await {
            Ok(height) => Ok(Response::U64(height)),
            Err(e) => Err(e),
        }
    }

    fn get_block_id(&self, arg: &str) -> Result<BlockId, CliError> {
        if arg == "latest" {
            Ok(BlockId::latest())
        } else if arg.len() == 64 {
            let b256 = B256::from_hex(arg).map_err(|_| {
                CliError::InvalidArgument(
                    arg.to_string(),
                    "number, block hash or `latest`".to_string(),
                )
            })?;
            Ok(BlockId::hash(b256))
        } else {
            Ok(BlockId::number(arg.parse::<u64>().map_err(|_| {
                CliError::InvalidArgument(
                    arg.to_string(),
                    "number, block hash or `latest`".to_string(),
                )
            })?))
        }
    }

    async fn block(&self, args: Vec<String>) -> Result<Response, CliError> {
        if args.len() != 1 {
            return Err(CliError::InvalidNumberOfArguments("1".to_string()));
        }
        let block_id = self.get_block_id(&args[0])?;
        let block = self.client().await.lock().await.get_block(block_id).await;
        match block {
            Ok(block) => Ok(Response::Block(block)),
            Err(e) => Err(e),
        }
    }

    async fn get_energy_price(&self) -> Result<Response, CliError> {
        let price = self.client().await.lock().await.get_energy_price().await;
        match price {
            Ok(price) => Ok(Response::U128(price)),
            Err(e) => Err(e),
        }
    }

    async fn get_network_id(&self) -> Result<Response, CliError> {
        let network_id = self.client().await.lock().await.get_network_id().await;
        match network_id {
            Ok(network_id) => Ok(Response::U64(network_id)),
            Err(e) => Err(e),
        }
    }

    async fn get_balance(&self, args: Vec<String>) -> Result<Response, CliError> {
        if args.len() != 2 {
            return Err(CliError::InvalidNumberOfArguments("1".to_string()));
        }
        let address = &args[0];
        let block_id = self.get_block_id(&args[1])?;
        let balance = self
            .client()
            .await
            .lock()
            .await
            .get_balance(address.to_string(), block_id)
            .await;
        match balance {
            Ok(balance) => Ok(Response::U256(balance)),
            Err(e) => Err(e),
        }
    }

    async fn get_tx_count(&self, args: Vec<String>) -> Result<Response, CliError> {
        if args.len() != 2 {
            return Err(CliError::InvalidNumberOfArguments("1".to_string()));
        }
        let address = &args[0];
        let block_id = self.get_block_id(&args[1])?;
        let tx_count: Result<u64, CliError> = self
            .client()
            .await
            .lock()
            .await
            .get_tx_count(address.to_string(), block_id)
            .await;
        match tx_count {
            Ok(tx_count) => Ok(Response::U64(tx_count)),
            Err(e) => Err(e),
        }
    }

    async fn get_code(&self, args: Vec<String>) -> Result<Response, CliError> {
        if args.len() != 2 {
            return Err(CliError::InvalidNumberOfArguments("1".to_string()));
        }
        let address = &args[0];
        let block_id = self.get_block_id(&args[1])?;

        let code = self
            .client()
            .await
            .lock()
            .await
            .get_code(address.to_string(), block_id)
            .await;
        match code {
            Ok(code) => Ok(Response::String(code)),
            Err(e) => Err(e),
        }
    }

    async fn send_raw_transaction(&self, args: Vec<String>) -> Result<Response, CliError> {
        if args.len() != 1 {
            return Err(CliError::InvalidNumberOfArguments("1".to_string()));
        }
        let tx = &args[0];
        let tx_hash = self
            .client()
            .await
            .lock()
            .await
            .send_raw_transaction(tx.to_string())
            .await;
        match tx_hash {
            Ok(tx_hash) => Ok(Response::String(tx_hash)),
            Err(e) => Err(e),
        }
    }

    async fn get_storage_at(&self, args: Vec<String>) -> Result<Response, CliError> {
        if args.len() != 3 {
            return Err(CliError::InvalidNumberOfArguments("1".to_string()));
        }
        let address = &args[0];
        let key = args[1]
            .parse::<u128>()
            .map_err(|_| CliError::InvalidArgument(args[1].clone(), "hex string".to_string()))?;
        let block_id = self.get_block_id(&args[2])?;

        let storage = self
            .client()
            .await
            .lock()
            .await
            .get_storage_at(address.to_string(), key, block_id)
            .await;
        match storage {
            Ok(storage) => Ok(Response::String(storage)),
            Err(e) => Err(e),
        }
    }

    async fn syncing(&self) -> Result<Response, CliError> {
        let syncing = self.client().await.lock().await.syncing().await;
        match syncing {
            Ok(syncing) => Ok(Response::SyncStatus(syncing)),
            Err(e) => Err(e),
        }
    }

    async fn get_tx(&self, args: Vec<String>) -> Result<Response, CliError> {
        if args.len() != 1 {
            return Err(CliError::InvalidNumberOfArguments("1".to_string()));
        }
        let tx_hash = &args[0];
        let tx = self
            .client()
            .await
            .lock()
            .await
            .get_transaction_by_hash(tx_hash.to_string())
            .await;
        match tx {
            Ok(tx) => Ok(Response::Transaction(tx)),
            Err(e) => Err(e),
        }
    }

    async fn get_tx_receipt(&self, args: Vec<String>) -> Result<Response, CliError> {
        if args.len() != 1 {
            return Err(CliError::InvalidNumberOfArguments("1".to_string()));
        }
        let tx_hash = &args[0];
        let receipt = self
            .client()
            .await
            .lock()
            .await
            .get_transaction_receipt(tx_hash.to_string())
            .await;
        match receipt {
            Ok(receipt) => Ok(Response::Receipt(Box::new(receipt))),
            Err(e) => Err(e),
        }
    }

    async fn get_uncle(&self, args: Vec<String>) -> Result<Response, CliError> {
        if args.len() != 2 {
            return Err(CliError::InvalidNumberOfArguments("1".to_string()));
        }
        let block_id = self.get_block_id(&args[0])?;
        let uncle_index = args[1]
            .parse::<u64>()
            .map_err(|_| CliError::InvalidArgument(args[1].clone(), "number".to_string()))?;
        let uncle = self
            .client()
            .await
            .lock()
            .await
            .get_uncle(block_id, uncle_index)
            .await;
        match uncle {
            Ok(uncle) => Ok(Response::Block(uncle)),
            Err(e) => Err(e),
        }
    }
}

#[async_trait::async_trait]
impl Module for XcbModule {
    async fn execute(&mut self, command: String, args: Vec<String>) -> Result<Response, CliError> {
        match command.as_str() {
            "get_block_height" => self.block_height().await,
            "get_energy_price" => self.get_energy_price().await,
            "get_network_id" => self.get_network_id().await,

            "get_block" => self.block(args).await,
            "get_uncle" => self.get_uncle(args).await,

            "get_balance" => self.get_balance(args).await,
            "get_code" => self.get_code(args).await,
            "get_storage_at" => self.get_storage_at(args).await,

            "get_tx_count" => self.get_tx_count(args).await,
            "get_tx" => self.get_tx(args).await,
            "get_tx_receipt" => self.get_tx_receipt(args).await,

            "send_raw_transaction" => self.send_raw_transaction(args).await,

            "syncing" => self.syncing().await,
            _ => Err(CliError::UnknownCommand),
        }
    }
}
