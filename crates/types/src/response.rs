use atoms_rpc_types::Block;
use serde::Serialize;

use crate::{account::KeyFile, Account};
use atoms_rpc_types::SyncStatus;
use std::str::FromStr;

/// ResponseView decided if response of call will be returned as a string, json object or human readable format
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ResponseView {
    /// Response will be returned as a string
    #[default]
    String,
    /// Response will be returned as a json object
    Json,
    /// Response will be returned as a human readable format
    Human,
}

impl FromStr for ResponseView {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "string" => Ok(ResponseView::String),
            "json" => Ok(ResponseView::Json),
            "human" => Ok(ResponseView::Human),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, PartialEq, Debug, Clone)]
pub enum Response {
    U64(u64),
    U128(u128),

    Bool(bool),
    String(String),

    Block(Block),
    Struct(serde_json::Value), // Use serde_json::Value for custom structs
    SyncStatus(SyncStatus),

    Accounts(Vec<Account>),
    Keyfile(KeyFile),
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Response::U64(val) => write!(f, "{}", val),
            Response::U128(val) => write!(f, "{}", val),
            Response::Bool(val) => write!(f, "{}", val),
            Response::String(val) => write!(f, "{}", val),
            Response::Block(val) => write!(
                f,
                "{}",
                serde_json::to_string(val)
                    .unwrap_or_else(|_| "Failed to serialize to JSON".to_string())
            ),
            Response::Struct(val) => write!(f, "{}", val),
            Response::Accounts(accounts) => {
                writeln!(f, "Accounts:")?;
                for (num, account) in accounts.iter().enumerate() {
                    writeln!(
                        f,
                        "{}: {} . File - {}. {}",
                        num + 1,
                        account.address,
                        account.path.to_str().unwrap(),
                        account.is_unlocked_str()
                    )?;
                }
                Ok(())
            }
            Response::SyncStatus(sync_info) => {
                if let SyncStatus::Info(sync_info) = sync_info {
                    write!(f, "RPC node is syncing now. Current block: {}, highest block: {}, starting block: {}", sync_info.current_block, sync_info.highest_block, sync_info.starting_block)
                } else {
                    write!(f, "RPC node is synced and data is up to date")
                }
            }

            Response::Keyfile(keyfile) => write!(f, "{}", keyfile),
        }
    }
}

impl Response {
    pub fn format(&self, view: ResponseView) -> String {
        match view {
            ResponseView::String => self.to_string(),
            ResponseView::Json => serde_json::to_string(self)
                .unwrap_or_else(|_| "Failed to serialize to JSON".to_string()),
            ResponseView::Human => self.to_human_readable(),
        }
    }

    fn to_human_readable(&self) -> String {
        match self {
            Response::U64(val) => format!("u64 value: {:#?}", val),
            Response::U128(val) => format!("U128 value: {:#?}", val),
            Response::Bool(val) => format!("Boolean value: {:#?}", val),
            Response::String(val) => format!("String value: {:#?}", val),
            Response::Block(val) => format!("{:#?}", val),
            Response::Struct(val) => format!("Struct value: {:#?}", val),
            Response::Accounts(_) => self.to_string(),
            Response::Keyfile(_) => self.to_string(),
            Response::SyncStatus(_) => self.to_string(),
        }
    }
}
