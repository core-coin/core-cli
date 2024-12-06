use atoms_signer_wallet::LocalWallet;
use cli_error::CliError;
use hex::ToHex;
use rand::rngs::OsRng;
use rand::{CryptoRng, Rng};
use rand_core::le;
use rpassword::read_password;
use rpc::RpcClient;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::{fs, io};
use tokio::sync::Mutex;
use types::account::Accounts;
use types::response::Response;
use types::Account;
use xcb_keystore::EthKeystore as XcbKeystore;

use crate::Module;

const ACCOUNT_SUBDIR: &str = "keystore";

pub struct XcbKeyModule {
    client: Arc<Mutex<dyn RpcClient + Send>>,
    accounts_dir: String,
    network_id: u64,
    accounts: Accounts,
}

impl XcbKeyModule {
    pub async fn new(
        client: Arc<Mutex<dyn RpcClient + Send>>,
        datadir: String,
        accounts: Accounts,
    ) -> Self {
        let network_id = client.lock().await.get_network_id().await.unwrap();
        let accounts_dir = datadir + "/" + ACCOUNT_SUBDIR;

        // create data if not exists
        if !std::path::Path::new(&accounts_dir).exists() {
            std::fs::create_dir_all(&accounts_dir).unwrap();
        }

        XcbKeyModule {
            client,
            accounts_dir,
            network_id,
            accounts,
        }
    }

    async fn client(&self) -> Arc<Mutex<dyn RpcClient + Send>> {
        self.client.clone()
    }

    async fn create_account(&self, args: Vec<String>) -> Result<Response, CliError> {
        let mut password;
        if args.len() == 0 {
            password = self.prompt_password("Enter password: ")?;
            let confirm_password = self.prompt_password("Confirm password: ")?;
            if password != confirm_password {
                return Err(CliError::InvalidArgument(
                    "passwords do not match".to_string(),
                    "equal".to_string(),
                ));
            }
        } else if args.len() == 1 {
            password = args[0].clone();
        } else {
            return Err(CliError::InvalidNumberOfArguments("0 or 1".to_string()));
        }

        let mut rng = OsRng;
        let key = LocalWallet::new_keystore(
            self.accounts_dir.clone(),
            &mut rng,
            password,
            None,
            self.network_id,
        )?;
        let pk_hex = hex::encode(key.0.signer().to_bytes());
        let public_key = hex::encode(key.0.signer().verifying_key().as_bytes());
        let core_id = key.0.address();

        Ok(types::response::Response::String(
            format!(
                "Keyfile added to directory {}.\nPrivate Key: {}\nPublic Key: {}\nCoreID: {}",
                self.accounts_dir, pk_hex, public_key, core_id
            )
            .to_string(),
        ))
    }

    fn prompt_password(&self, prompt: &str) -> Result<String, CliError> {
        println!("{}", prompt);
        read_password().map_err(|e| CliError::IoError(e))
    }

    fn prompt_address(&self, prompt: &str) -> Result<String, CliError> {
        println!("{}", prompt);
        io::stdout().flush().map_err(|e| CliError::IoError(e))?;
        let mut address = String::new();
        io::stdin()
            .read_line(&mut address)
            .map_err(|e| CliError::IoError(e))?;
        Ok(address.trim().to_string())
    }

    async fn unlock_account(&self, args: Vec<String>) -> Result<Response, CliError> {
        let mut password;
        let mut core_id;
        match args.len() {
            2 => {
                core_id = args[0].clone();
                password = args[1].clone();
            }
            1 => {
                core_id = args[0].clone();
                password = self.prompt_password("Enter password: ")?;
            }
            0 => {
                core_id = self.prompt_address("Enter address: ")?;
                password = self.prompt_password("Enter password: ")?;
            }
            _ => return Err(CliError::InvalidNumberOfArguments("0, 1 or 2".to_string())),
        }

        let account = self.accounts.get_account(&core_id);
        if account.is_none() {
            return Err(CliError::AccountNotFound(core_id));
        }
        let mut account = account.unwrap();

        let wallet = LocalWallet::decrypt_keystore(&account.path, password, self.network_id)
            .map_err(|e| CliError::WalletError(e))?;

        self.accounts.unlock_account(&account.address, wallet);

        Ok(types::response::Response::String(
            format!("Account {} successfully unlocked!", core_id).to_string(),
        ))
    }

    async fn list_accounts(&self) -> Result<Response, CliError> {
        let path = PathBuf::from(&self.accounts_dir);
        for entry in fs::read_dir(&path).map_err(|e| CliError::IoError(e))? {
            let entry = entry.map_err(|e| CliError::IoError(e))?;
            let path = entry.path();
            if path.is_file() {
                let account = self.read_keyfile(path).await;
                match account {
                    Ok(account) => {
                        self.accounts.add_account(account);
                    }
                    Err(_) => continue,
                }
            }
        }

        Ok(Response::Accounts(self.accounts.get_accounts()))
    }

    async fn read_keyfile(&self, path: PathBuf) -> Result<Account, CliError> {
        let keyfile_json = fs::read_to_string(&path).map_err(|e| CliError::IoError(e))?;
        let keyfile_data: XcbKeystore =
            serde_json::from_str(&keyfile_json).map_err(|e| CliError::SerdeError(e))?;
        let account = Account {
            address: keyfile_data.address.encode_hex(),
            wallet: None,
            path: path,
            unlocked: 0,
        };
        Ok(account)
    }
}

#[async_trait::async_trait]
impl Module for XcbKeyModule {
    async fn execute(&mut self, command: String, args: Vec<String>) -> Result<Response, CliError> {
        match command.as_str() {
            "new" => self.create_account(args.clone()).await,
            "list" => self.list_accounts().await,
            "unlock" => self.unlock_account(args.clone()).await,
            _ => Err(CliError::UnknownCommand),
        }
    }
}
