use core::error;

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
    #[error("Invalid number of arguments: must be {0}. {1}")]
    InvalidNumberOfArguments(usize, String),
    #[error("Invalid hex argument: {0}")]
    InvalidHexArgument(String),
    #[error("Invalid argument: {0}. Must be {1}")]
    InvalidArgument(String, String),
}
