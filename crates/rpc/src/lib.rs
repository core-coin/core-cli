use async_trait::async_trait;
use atoms_rpc_types::{Block, SyncStatus};
use base_primitives::U256;
use cli_error::CliError;

pub mod go_core;
pub use go_core::GoCoreClient;

pub mod mock;
pub use mock::MockRpcClient;

#[async_trait]
pub trait RpcClient {
    async fn get_block_height(&self) -> Result<u64, CliError>;
    async fn get_block_by_hash(&self, hash: String) -> Result<Block, CliError>;
    async fn get_block_by_number(&self, number: u64) -> Result<Block, CliError>;
    async fn get_block_latest(&self) -> Result<Block, CliError>;

    async fn get_energy_price(&self) -> Result<u128, CliError>;
    async fn get_network_id(&self) -> Result<u64, CliError>;

    async fn syncing(&self) -> Result<SyncStatus, CliError>;

    async fn get_balance(&self, account: String, block: Option<u64>) -> Result<U256, CliError>;
    async fn get_tx_count(&self, account: String, block: Option<u64>) -> Result<u64, CliError>;
    async fn get_code(&self, account: String, block: Option<u64>) -> Result<String, CliError>;

    async fn get_storage_at(
        &self,
        account: String,
        key: u128,
        block: Option<u64>,
    ) -> Result<String, CliError>;

    async fn send_raw_transaction(&self, tx: String) -> Result<String, CliError>;
}
