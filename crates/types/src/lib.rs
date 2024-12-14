pub mod response;
pub use response::{Response, ResponseView};

pub mod account;
pub use account::Account;

pub const DEFAULT_BACKEND: &str = "https://xcbapi-arch-mainnet.coreblockchain.net/";
