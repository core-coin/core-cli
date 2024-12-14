use crate::{CliError, RpcClient};
use async_trait::async_trait;
use atoms_rpc_types::Block;

pub struct MockRpcClient {
    pub block_height: u64,
    pub block_by_hash: Block,
    pub block_by_number: Block,
    pub block_latest: Block,
    pub energy_price: u128,
    pub network_id: u64,
}

impl MockRpcClient {
    pub fn new() -> Self {
        MockRpcClient {
            block_height: 0,
            block_by_hash: Block::default(),
            block_by_number: Block::default(),
            block_latest: Block::default(),
            energy_price: 0,
            network_id: 0,
        }
    }

    pub fn with_block_height(mut self, block_height: u64) -> Self {
        self.block_height = block_height;
        self
    }

    pub fn with_block_by_hash(mut self, block_by_hash: Block) -> Self {
        self.block_by_hash = block_by_hash;
        self
    }

    pub fn with_block_by_number(mut self, block_by_number: Block) -> Self {
        self.block_by_number = block_by_number;
        self
    }

    pub fn with_block_latest(mut self, block_latest: Block) -> Self {
        self.block_latest = block_latest;
        self
    }

    pub fn with_energy_price(mut self, energy_price: u128) -> Self {
        self.energy_price = energy_price;
        self
    }

    pub fn with_network_id(mut self, network_id: u64) -> Self {
        self.network_id = network_id;
        self
    }
}

impl Default for MockRpcClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RpcClient for MockRpcClient {
    async fn get_block_height(&self) -> Result<u64, CliError> {
        Ok(self.block_height)
    }

    async fn get_block_by_hash(&self, _hash: String) -> Result<Block, CliError> {
        Ok(self.block_by_hash.clone())
    }

    async fn get_block_by_number(&self, _number: u64) -> Result<Block, CliError> {
        Ok(self.block_by_number.clone())
    }

    async fn get_block_latest(&self) -> Result<Block, CliError> {
        Ok(self.block_latest.clone())
    }

    async fn get_energy_price(&self) -> Result<u128, CliError> {
        Ok(self.energy_price)
    }

    async fn get_network_id(&self) -> Result<u64, CliError> {
        Ok(self.network_id)
    }
}
