use crate::RpcClient;
use async_trait::async_trait;
use atoms_provider::{network::Ethereum, Provider, RootProvider};
use atoms_rpc_client::RpcClient as AtomsRpcClient;
use atoms_rpc_types::{Block, BlockId, SyncStatus, Transaction, TransactionReceipt};
use atoms_transport_http::{Client, Http};
use base_primitives::{hex::FromHex, FixedBytes, IcanAddress, U256};
use cli_error::CliError;

pub struct GoCoreClient {
    provider: RootProvider<Http<Client>>,
}

impl GoCoreClient {
    pub fn new(backend_url: String) -> Self {
        let url = reqwest::Url::parse(&backend_url).expect("Invalid URL for rpc client");
        let client = AtomsRpcClient::new_http(url);
        let provider: RootProvider<Http<Client>> = RootProvider::<_, Ethereum>::new(client);

        GoCoreClient { provider }
    }
}

#[async_trait]
impl RpcClient for GoCoreClient {
    async fn get_block_height(&self) -> Result<u64, CliError> {
        let response = self
            .provider
            .get_block_number()
            .await
            .map_err(|e| CliError::RpcError(e.to_string()))?;
        Ok(response)
    }

    async fn get_block(&self, block: BlockId) -> Result<Block, CliError> {
        let response = self
            .provider
            .get_block(block, true)
            .await
            .map_err(|e| CliError::RpcError(e.to_string()))?;
        match response {
            Some(block) => Ok(block),
            None => Err(CliError::RpcError("Block not found".to_string())),
        }
    }

    async fn get_energy_price(&self) -> Result<u128, CliError> {
        let response = self
            .provider
            .get_energy_price()
            .await
            .map_err(|e| CliError::RpcError(e.to_string()))?;
        Ok(response)
    }

    async fn get_network_id(&self) -> Result<u64, CliError> {
        let response = self
            .provider
            .get_chain_id()
            .await
            .map_err(|e| CliError::RpcError(e.to_string()))?;
        Ok(response)
    }

    async fn syncing(&self) -> Result<SyncStatus, CliError> {
        let response = self
            .provider
            .syncing()
            .await
            .map_err(|e| CliError::RpcError(e.to_string()))?;
        Ok(response)
    }

    async fn get_balance(&self, account: String, block: BlockId) -> Result<U256, CliError> {
        let hex = IcanAddress::from_hex(account)
            .map_err(|e| CliError::InvalidHexArgument(e.to_string()))?;
        let response = self
            .provider
            .get_balance(hex, block)
            .await
            .map_err(|e| CliError::RpcError(e.to_string()))?;
        Ok(response)
    }

    async fn get_tx_count(&self, account: String, block: BlockId) -> Result<u64, CliError> {
        let hex = IcanAddress::from_hex(account)
            .map_err(|e| CliError::InvalidHexArgument(e.to_string()))?;
        let response = self
            .provider
            .get_transaction_count(hex, block)
            .await
            .map_err(|e| CliError::RpcError(e.to_string()))?;
        Ok(response)
    }

    async fn get_code(&self, account: String, block: BlockId) -> Result<String, CliError> {
        let hex = IcanAddress::from_hex(account)
            .map_err(|e| CliError::InvalidHexArgument(e.to_string()))?;
        let response = self
            .provider
            .get_code_at(hex, block)
            .await
            .map_err(|e| CliError::RpcError(e.to_string()))?;
        Ok(response.to_string())
    }

    async fn send_raw_transaction(&self, tx: String) -> Result<String, CliError> {
        let response = self
            .provider
            .send_raw_transaction(tx.as_bytes())
            .await
            .map_err(|e| CliError::RpcError(e.to_string()))?;
        Ok(response.tx_hash().to_string())
    }

    async fn get_storage_at(
        &self,
        address: String,
        key: u128,
        block: BlockId,
    ) -> Result<String, CliError> {
        let hex = IcanAddress::from_hex(address)
            .map_err(|e| CliError::InvalidHexArgument(e.to_string()))?;
        let response = self
            .provider
            .get_storage_at(hex, U256::from(key), block)
            .await
            .map_err(|e| CliError::RpcError(e.to_string()))?;
        Ok(response.to_string())
    }

    async fn get_transaction_count(
        &self,
        account: String,
        block: BlockId,
    ) -> Result<u64, CliError> {
        let hex = IcanAddress::from_hex(account)
            .map_err(|e| CliError::InvalidHexArgument(e.to_string()))?;
        let response = self
            .provider
            .get_transaction_count(hex, block)
            .await
            .map_err(|e| CliError::RpcError(e.to_string()))?;
        Ok(response)
    }

    async fn get_transaction_by_hash(&self, hash: String) -> Result<Transaction, CliError> {
        let fixed_bytes =
            FixedBytes::from_hex(hash).map_err(|e| CliError::InvalidHexArgument(e.to_string()))?;
        let response = self
            .provider
            .get_transaction_by_hash(fixed_bytes)
            .await
            .map_err(|e| CliError::RpcError(e.to_string()))?;
        if let Some(tx) = response {
            Ok(tx)
        } else {
            Err(CliError::RpcError("Transaction not found".to_string()))
        }
    }

    async fn get_transaction_receipt(&self, hash: String) -> Result<TransactionReceipt, CliError> {
        let b256_hash =
            FixedBytes::from_hex(hash).map_err(|e| CliError::InvalidHexArgument(e.to_string()))?;
        let response = self
            .provider
            .get_transaction_receipt(b256_hash)
            .await
            .map_err(|e| CliError::RpcError(e.to_string()))?;
        if let Some(receipt) = response {
            Ok(receipt)
        } else {
            Err(CliError::RpcError(
                "Transaction receipt not found".to_string(),
            ))
        }
    }

    async fn get_uncle(&self, block: BlockId, index: u64) -> Result<Block, CliError> {
        let response = self
            .provider
            .get_uncle(block, index)
            .await
            .map_err(|e| CliError::RpcError(e.to_string()))?;
        match response {
            Some(block) => Ok(block),
            None => Err(CliError::RpcError("Uncle not found".to_string())),
        }
    }
}
