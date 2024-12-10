use chrono::prelude::*;
use hex::ToHex;
use serde::Serialize;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

const ACCOUNT_UNLOCK_DURATION: i64 = 300;

/// A struct representing an account.
/// Used to store the account's address, wallet, path, and unlocked state.
#[derive(Serialize, Debug, Clone)]
pub struct Account {
    /// The address of the account.
    pub address: String,
    /// Internal wallet struct from atoms_signer_wallet crate
    /// None if the account is locked.
    /// Some(wallet) if the account is unlocked.
    #[serde(skip_serializing)]
    pub wallet: Option<atoms_signer_wallet::LocalWallet>,
    /// The path to the account's keyfile.
    pub path: PathBuf,
    /// The account's unlocked state. If zero - locked, if non-zero - unlocked at the specified timestamp.
    pub unlocked: i64,
}

impl Account {
    /// Create a new account.
    pub fn new(
        address: String,
        wallet: Option<atoms_signer_wallet::LocalWallet>,
        path: PathBuf,
        unlocked: i64,
    ) -> Self {
        Account {
            address,
            wallet,
            path,
            unlocked,
        }
    }

    /// self.unloocked is a timestamp till which the account is unlocked.
    pub fn is_unlocked(&self) -> bool {
        self.unlocked > Utc::now().timestamp()
    }

    /// self.unloocked is a timestamp till which the account is unlocked.
    pub fn is_unlocked_str(&self) -> String {
        if self.is_unlocked() {
            "🔓 Unlocked".to_string()
        } else {
            "🔒 Locked".to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Accounts {
    accounts: Arc<Mutex<Vec<Account>>>,
}

impl Accounts {
    pub fn new(accounts: Vec<Account>) -> Self {
        Accounts {
            accounts: Arc::new(Mutex::new(accounts)),
        }
    }

    /// Get the list of accounts.
    /// If the account is no longer unlocked, it's wallet will be removed.
    pub fn get_accounts(&self) -> Vec<Account> {
        self.check_outdated_unlocks();

        self.accounts.lock().unwrap().clone()
    }

    /// Add the account to the list of accounts.
    /// If the account with the same address already exists, it will not be added.
    pub fn add_account(&self, account: Account) {
        self.check_outdated_unlocks();

        if self
            .get_accounts()
            .iter()
            .any(|a| a.address == account.address)
        {
            return;
        }
        self.accounts.lock().unwrap().push(account);
    }

    /// Remove the account with the specified address.
    pub fn remove_account(&self, address: &str) {
        self.check_outdated_unlocks();

        let mut accounts = self.accounts.lock().unwrap();
        let index = accounts.iter().position(|a| a.address == address);
        if let Some(index) = index {
            accounts.remove(index);
        }
    }

    /// Get the account with the specified address.
    pub fn get_account(&self, address: &str) -> Option<Account> {
        self.check_outdated_unlocks();

        let mut accounts = self.accounts.lock().unwrap();
        accounts.iter().find(|a| a.address == address).cloned()
    }

    /// Remove the wallet from all accounts that are no longer unlocked.
    fn check_outdated_unlocks(&self) {
        let mut accounts = self.accounts.lock().unwrap();
        for account in accounts.iter_mut() {
            if !account.is_unlocked() {
                account.wallet = None
            }
        }
    }

    /// Unlock the account with the specified address.
    /// The account will be unlocked for ACCOUNT_UNLOCK_DURATION seconds.
    /// The account's wallet will be set to the specified wallet.
    pub fn unlock_account(&self, address: &str, wallet: atoms_signer_wallet::LocalWallet) {
        self.check_outdated_unlocks();

        let mut accounts = self.accounts.lock().unwrap();
        if let Some(account) = accounts.iter_mut().find(|a| a.address == address) {
            account.wallet = Some(wallet);
            account.unlocked = Utc::now().timestamp() + ACCOUNT_UNLOCK_DURATION;
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct KeyFile {
    pub address: String,
    pub public_key: String,
    pub private_key: String,
}

impl KeyFile {
    pub fn new(address: String, public_key: String, private_key: String) -> Self {
        KeyFile {
            address,
            public_key,
            private_key,
        }
    }

    pub fn from_wallet(wallet: &atoms_signer_wallet::LocalWallet) -> Self {
        KeyFile {
            address: wallet.address().to_string(),
            public_key: hex::encode(wallet.signer().verifying_key().as_bytes()),
            private_key: hex::encode(wallet.signer().to_bytes()),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "Address: {}\nPublic key: {}\nPrivate key: {}",
            self.address, self.public_key, self.private_key
        )
    }
}
