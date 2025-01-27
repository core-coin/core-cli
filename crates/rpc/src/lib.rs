use async_trait::async_trait;
use atoms_rpc_types::{Block, BlockId, SyncStatus, Transaction, TransactionReceipt};
use base_primitives::U256;
use cli_error::CliError;

pub mod go_core;
pub use go_core::GoCoreClient;

pub mod mock;
pub use mock::MockRpcClient;

#[async_trait]
pub trait RpcClient {
    async fn get_block_height(&self) -> Result<u64, CliError>;
    async fn get_block(&self, block: BlockId) -> Result<Block, CliError>;
    async fn get_uncle(&self, block: BlockId, index: u64) -> Result<Block, CliError>;

    async fn get_transaction_count(&self, account: String, block: BlockId)
        -> Result<u64, CliError>;
    async fn get_transaction_by_hash(&self, hash: String) -> Result<Transaction, CliError>;
    async fn get_transaction_receipt(&self, hash: String) -> Result<TransactionReceipt, CliError>;

    async fn get_energy_price(&self) -> Result<u128, CliError>;
    async fn get_network_id(&self) -> Result<u64, CliError>;

    async fn syncing(&self) -> Result<SyncStatus, CliError>;

    async fn get_balance(&self, account: String, block: BlockId) -> Result<U256, CliError>;
    async fn get_tx_count(&self, account: String, block: BlockId) -> Result<u64, CliError>;
    async fn get_code(&self, account: String, block: BlockId) -> Result<String, CliError>;
    async fn get_storage_at(
        &self,
        account: String,
        key: u128,
        block: BlockId,
    ) -> Result<String, CliError>;

    async fn send_raw_transaction(&self, tx: String) -> Result<String, CliError>;
}
