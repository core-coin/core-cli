use core::error;

use atoms_signer_wallet as wallet;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("RPC error: {0}")]
    RpcError(String),
    #[error("Invalid module name: {0}. Please write 'list' to get list of all possible modules and commands")]
    UnknownModule(String),
    #[error("Unknown command. Please write 'list' to get list of all possible commands")]
    UnknownCommand,
    #[error("Unknown client: {0}")]
    UnknownClient(String),
    #[error("Invalid number of arguments: must be {0}")]
    InvalidNumberOfArguments(String),
    #[error("Invalid hex argument: {0}")]
    InvalidHexArgument(String),
    #[error("Invalid argument: {0}. Must be {1}")]
    InvalidArgument(String, String),
    #[error("Wallet error: {0}")]
    WalletError(#[from] wallet::WalletError),
    #[error("Account with address {0} not found")]
    AccountNotFound(String),

    #[error("Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Error: {0}")]
    SerdeError(#[from] serde_json::Error),
}
