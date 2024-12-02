use async_trait::async_trait;
use cli_error::CliError;
use types::response::Response;

pub mod xcb;
pub mod xcbkey;

pub use xcb::XcbModule;
pub use xcbkey::XcbKeyModule;

#[async_trait]
pub trait Module {
    async fn execute(&mut self, command: String, args: Vec<String>) -> Result<Response, CliError>;
}
