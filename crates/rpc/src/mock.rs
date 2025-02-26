use std::str::FromStr;

use crate::{CliError, RpcClient};
use async_trait::async_trait;
use atoms_rpc_types::{Block, BlockId};
use base_primitives::{IcanAddress, B256, U256};

pub struct MockRpcClient {
    pub block_height: u64,
    pub block_by_hash: Block,
    pub block_by_number: Block,
    pub block_latest: Block,
    pub energy_price: u128,
    pub network_id: u64,
    pub syncing: atoms_rpc_types::SyncStatus,
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
            syncing: atoms_rpc_types::SyncStatus::None,
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

    pub fn with_syncing(mut self, syncing: atoms_rpc_types::SyncStatus) -> Self {
        self.syncing = syncing;
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

    async fn get_block(&self, _block: BlockId) -> Result<Block, CliError> {
        Ok(self.block_latest.clone())
    }

    async fn get_energy_price(&self) -> Result<u128, CliError> {
        Ok(self.energy_price)
    }

    async fn get_network_id(&self) -> Result<u64, CliError> {
        Ok(self.network_id)
    }

    async fn syncing(&self) -> Result<atoms_rpc_types::SyncStatus, CliError> {
        Ok(self.syncing)
    }

    async fn get_balance(&self, _account: String, _block: BlockId) -> Result<U256, CliError> {
        Ok(U256::from(0))
    }

    async fn get_tx_count(&self, _account: String, _block: BlockId) -> Result<u64, CliError> {
        Ok(0)
    }

    async fn get_code(&self, _account: String, _block: BlockId) -> Result<String, CliError> {
        Ok("".to_string())
    }

    async fn send_raw_transaction(&self, _tx: String) -> Result<String, CliError> {
        Ok("".to_string())
    }

    async fn get_storage_at(
        &self,
        _account: String,
        _key: u128,
        _block: BlockId,
    ) -> Result<String, CliError> {
        Ok("".to_string())
    }

    async fn get_transaction_count(
        &self,
        _account: String,
        _block: BlockId,
    ) -> Result<u64, CliError> {
        Ok(0)
    }

    async fn get_transaction_by_hash(
        &self,
        _hash: String,
    ) -> Result<atoms_rpc_types::Transaction, CliError> {
        Ok(atoms_rpc_types::Transaction::default())
    }

    async fn get_transaction_receipt(
        &self,
        hash: String,
    ) -> Result<atoms_rpc_types::TransactionReceipt, CliError> {
        Ok(atoms_rpc_types::TransactionReceipt {
            transaction_hash: B256::from_str(&hash).unwrap(),
            transaction_index: Some(0),
            block_hash: Some(B256::from_str("0x0").unwrap()),
            block_number: Some(0),
            energy_used: 0,
            contract_address: None,
            inner: Default::default(),
            blob_gas_price: Some(0),
            blob_gas_used: Some(0),
            from: IcanAddress::default(),
            to: None,
            state_root: None,
        })
    }

    async fn get_uncle(&self, _block: BlockId, _index: u64) -> Result<Block, CliError> {
        Ok(Block::default())
    }
}
