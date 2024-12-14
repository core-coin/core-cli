use async_trait::async_trait;
use atoms_rpc_types::Block;
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
}
