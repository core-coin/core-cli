use crate::RpcClient;
use async_trait::async_trait;
use atoms_provider::{network::Ethereum, Provider, RootProvider};
use atoms_rpc_client::RpcClient as AtomsRpcClient;
use atoms_rpc_types::{Block, BlockId, BlockNumberOrTag, RpcBlockHash};
use atoms_transport_http::{Client, Http};
use base_primitives::{hex::FromHex, FixedBytes};
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

    async fn get_block(&self, id: BlockId) -> Result<Option<Block>, CliError> {
        let res = self.provider.get_block(id, true).await;
        match res {
            Ok(block) => Ok(block),
            Err(e) => Err(CliError::RpcError(e.to_string())),
        }
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

    async fn get_block_by_hash(&self, hash: String) -> Result<Block, CliError> {
        let fixed_bytes =
            FixedBytes::from_hex(hash).map_err(|e| CliError::InvalidHexArgument(e.to_string()))?;
        let block_id = BlockId::Hash(RpcBlockHash::from_hash(fixed_bytes, Some(true)));
        let response = self
            .provider
            .get_block(block_id, true)
            .await
            .map_err(|e| CliError::RpcError(e.to_string()))?;
        match response {
            Some(block) => Ok(block),
            None => Err(CliError::RpcError("Block not found".to_string())),
        }
    }

    async fn get_block_by_number(&self, number: u64) -> Result<Block, CliError> {
        let block_id = BlockId::Number(BlockNumberOrTag::Number(number));
        let response = self
            .provider
            .get_block(block_id, true)
            .await
            .map_err(|e| CliError::RpcError(e.to_string()))?;
        match response {
            Some(block) => Ok(block),
            None => Err(CliError::RpcError("Block not found".to_string())),
        }
    }

    async fn get_block_latest(&self) -> Result<Block, CliError> {
        let block_id = BlockId::Number(BlockNumberOrTag::Latest);
        let response = self
            .provider
            .get_block(block_id, true)
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
}
