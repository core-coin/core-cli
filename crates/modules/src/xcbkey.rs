use atoms_signer::{Signature, Signer};
use atoms_signer_wallet::LocalWallet;
use cli_error::CliError;
use hex::ToHex;
use rand::rngs::OsRng;
use rpassword::read_password;
use rpc::RpcClient;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::{fs, io};
use tokio::sync::Mutex;
use types::account::{Accounts, KeyFile};
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
        let accounts_dir = format!("{}/{}", datadir, ACCOUNT_SUBDIR);

        // Create data directory if it does not exist
        if !PathBuf::from(&accounts_dir).exists() {
            fs::create_dir_all(&accounts_dir).unwrap();
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

    /// Read keyfile from the file
    /// Returns an account with the address and path but without the wallet information
    async fn read_keyfile(&self, path: PathBuf) -> Result<Account, CliError> {
        let keyfile_json = fs::read_to_string(&path).map_err(CliError::IoError)?;
        let keyfile_data: XcbKeystore =
            serde_json::from_str(&keyfile_json).map_err(CliError::SerdeError)?;
        Ok(Account {
            address: keyfile_data.address.encode_hex(),
            wallet: None,
            path,
            unlocked: 0,
        })
    }

    /// Get password from arguments or prompt
    /// If no arguments are provided, prompt for password
    /// If one argument is provided, treat it as password
    /// If filepath is provided, read the file
    fn get_password(&self, args: Vec<String>) -> Result<String, CliError> {
        let password = match args.len() {
            0 => {
                let password = self.prompt_password("Enter password (or a path to the file): ")?;
                let confirm_password = self.prompt_password("Confirm password (or the file): ")?;
                if password != confirm_password {
                    return Err(CliError::InvalidArgument(
                        "".to_string(),
                        "equal passwords".to_string(),
                    ));
                }
                password
            }
            1 => args[0].clone(),
            _ => return Err(CliError::InvalidNumberOfArguments("0 or 1".to_string())),
        };
        self.choose_file_or_string(password, "password")
    }

    /// Get core ID from arguments or prompt
    /// If no arguments are provided, prompt for core ID
    /// If one argument is provided, treat it as core ID
    /// If filepaths are provided, read the files
    fn get_core_id(&self, args: Vec<String>) -> Result<String, CliError> {
        match args.len() {
            1 => Ok(args[0].clone()),
            0 => self.prompt_string("Enter address (or a path to the file): "),
            _ => Err(CliError::InvalidNumberOfArguments("0 or 1".to_string())),
        }
    }

    /// Get private key and password from arguments or prompt
    /// If no arguments are provided, prompt for private key and password
    /// If one argument is provided, treat it as private key and prompt for password
    /// If two arguments are provided, treat them as private key and password
    /// If filepaths are provided, read the files
    fn get_key_and_password(&self, args: Vec<String>) -> Result<(String, String), CliError> {
        let (key, password) = match args.len() {
            0 => {
                let key = self.prompt_string("Enter private key (or a path to the file): ")?;
                let password = self.prompt_password("Enter password (or a path to the file): ")?;
                let confirm_password = self.prompt_password("Confirm password (or the file): ")?;
                if password != confirm_password {
                    return Err(CliError::InvalidArgument(
                        "".to_string(),
                        "equal passwords".to_string(),
                    ));
                }
                (key, password)
            }
            1 => {
                let key = args[0].clone();
                let password = self.prompt_password("Enter password (or a path to the file): ")?;
                let confirm_password = self.prompt_password("Confirm password (or the file): ")?;
                if password != confirm_password {
                    return Err(CliError::InvalidArgument(
                        "".to_string(),
                        "equal passwords".to_string(),
                    ));
                }
                (key, password)
            }
            2 => (args[0].clone(), args[1].clone()),
            _ => return Err(CliError::InvalidNumberOfArguments("0, 1, or 2".to_string())),
        };
        let key = self.choose_file_or_string(key, "private key")?;
        let password = self.choose_file_or_string(password, "password")?;

        Ok((key, password))
    }

    /// Get core ID and password from arguments or prompt
    /// If no arguments are provided, prompt for core ID and password
    /// If one argument is provided, treat it as core ID and prompt for password
    /// If two arguments are provided, treat them as core ID and password
    /// If filepaths are provided, read the files
    fn get_core_id_and_password(&self, args: Vec<String>) -> Result<(String, String), CliError> {
        let (core_id, password) = match args.len() {
            2 => (args[0].clone(), args[1].clone()),
            1 => {
                let core_id = args[0].clone();
                let password = self.prompt_password("Enter password (or a path to the file): ")?;
                (core_id, password)
            }
            0 => {
                let core_id = self.prompt_string("Enter address (or a path to the file): ")?;
                let password = self.prompt_password("Enter password  (or a path to the file): ")?;
                (core_id, password)
            }
            _ => return Err(CliError::InvalidNumberOfArguments("0, 1, or 2".to_string())),
        };
        let core_id = self.choose_file_or_string(core_id, "core ID")?;
        let password = self.choose_file_or_string(password, "password")?;

        Ok((core_id, password))
    }

    /// Get core ID and message from arguments or prompt
    /// If no arguments are provided, prompt for core ID and message
    /// If one argument is provided, treat it as core ID and prompt for message
    /// If two arguments are provided, treat them as core ID and message
    /// If filepaths are provided, read the files
    fn get_core_id_and_message(&self, args: Vec<String>) -> Result<(String, String), CliError> {
        let (core_id, message) = match args.len() {
            2 => (args[0].clone(), args[1].clone()),
            1 => {
                let core_id = args[0].clone();
                let message =
                    self.prompt_string("Enter message to sign (or a path to the file): ")?;
                (core_id, message)
            }
            0 => {
                let core_id = self.prompt_string("Enter address (or a path to the file): ")?;
                let message = self.prompt_string("Enter message to sign: ")?;
                (core_id, message)
            }
            _ => return Err(CliError::InvalidNumberOfArguments("0, 1, or 2".to_string())),
        };
        let core_id = self.choose_file_or_string(core_id, "core ID")?;
        let message = self.choose_file_or_string(message, "message to sign")?;

        Ok((core_id, message))
    }

    /// Get address, signature, and message from arguments or prompt
    /// If no arguments are provided, prompt for address, signature, and message
    /// If one argument is provided, treat it as address and prompt for signature and message
    /// If two arguments are provided, treat them as address and signature and prompt for message
    /// If three arguments are provided, treat them as address, signature, and message
    /// If filepaths are provided, read the files
    fn get_address_signature_and_message(
        &self,
        args: Vec<String>,
    ) -> Result<(String, String, String), CliError> {
        let (address, signature, message) = match args.len() {
            3 => (args[0].clone(), args[1].clone(), args[2].clone()),
            2 => {
                let address = args[0].clone();
                let signature = args[1].clone();
                let message = self.prompt_string("Enter message to verify: ")?;
                (address, signature, message)
            }
            1 => {
                let address = args[0].clone();
                let signature = self.prompt_string("Enter signature to verify: ")?;
                let message = self.prompt_string("Enter message to verify: ")?;
                (address, signature, message)
            }
            0 => {
                let address = self.prompt_string("Enter address (or a path to the file): ")?;
                let signature = self.prompt_string("Enter signature to verify: ")?;
                let message = self.prompt_string("Enter message to verify: ")?;
                (address, signature, message)
            }
            _ => {
                return Err(CliError::InvalidNumberOfArguments(
                    "0, 1, 2, or 3".to_string(),
                ))
            }
        };
        let address = self.choose_file_or_string(address, "address")?;
        let signature = self.choose_file_or_string(signature, "signature")?;
        let message = self.choose_file_or_string(message, "message to verify")?;
        Ok((address, signature, message))
    }

    /// Generate random keyfile with provided password
    /// Returns a keyfile with the wallet information the ID of the account
    async fn generate_keyfile(&self, password: String) -> Result<(LocalWallet, String), CliError> {
        let mut rng = OsRng;
        let keystore = LocalWallet::new_keystore(
            self.accounts_dir.clone(),
            &mut rng,
            password,
            None,
            self.network_id,
        )
        .map_err(CliError::WalletError)?;
        Ok(keystore)
    }

    /// Encrypt private key with provided password
    /// Returns a keyfile with the wallet information and the ID of the account
    async fn encrypt_keyfile(
        &self,
        key: String,
        password: String,
    ) -> Result<(LocalWallet, String), CliError> {
        let mut rng = OsRng;
        let key = hex::decode(&key).map_err(|_| CliError::InvalidHexArgument(key))?;
        let keystore = LocalWallet::encrypt_keystore(
            self.accounts_dir.clone(),
            &mut rng,
            key,
            password,
            None,
            self.network_id,
        )
        .map_err(CliError::WalletError)?;
        Ok(keystore)
    }

    /// Add account to the shared list of accounts
    async fn add_account_to_list(&self, key: &LocalWallet, id: String) {
        let account = Account {
            address: key.address().to_string(),
            wallet: Some(key.clone()),
            path: PathBuf::from(self.accounts_dir.clone() + "/" + &id),
            unlocked: 0,
        };
        self.accounts.add_account(account);
    }

    /// Format keyfile response to user-friendly message
    fn format_keyfile_response(&self, key: &LocalWallet) -> Response {
        let pk_hex = hex::encode(key.signer().to_bytes());
        let public_key = hex::encode(key.signer().verifying_key().as_bytes());
        let core_id = key.address().to_string();
        Response::Keyfile(KeyFile::new(core_id, public_key, pk_hex))
    }

    /// Prompt for password
    fn prompt_password(&self, prompt: &str) -> Result<String, CliError> {
        println!("{}", prompt);
        read_password().map_err(CliError::IoError)
    }

    /// Prompt for string
    fn prompt_string(&self, prompt: &str) -> Result<String, CliError> {
        print!("{}", prompt);
        io::stdout().flush().map_err(|e| CliError::IoError(e))?;
        let mut address = String::new();
        io::stdin()
            .read_line(&mut address)
            .map_err(|e| CliError::IoError(e))?;
        Ok(address.trim().to_string())
    }

    /// Choose between file and string
    /// If the path exists, read the file
    /// Otherwise, return the string
    fn choose_file_or_string(&self, text: String, label: &str) -> Result<String, CliError> {
        let path = PathBuf::from(&text);
        if path.exists() {
            let file_content = fs::read_to_string(&path)
                .map_err(CliError::IoError)?
                .trim()
                .to_string();
            println!(
                "Seems like that \"{}\" is a file\nReading {} from it...",
                text, label
            );
            return Ok(file_content);
        }
        Ok(text)
    }
}

#[async_trait::async_trait]
impl Module for XcbKeyModule {
    async fn execute(&mut self, command: String, args: Vec<String>) -> Result<Response, CliError> {
        match command.as_str() {
            "new" => self.create_account(args).await,
            "new_from_key" => self.create_account_from_key(args).await,
            "list" => self.list_accounts().await,
            "inspect" => self.inspect(args).await,
            "unlock" => self.unlock_account(args).await,
            "sign" => self.sign(args).await,
            "verify" => self.verify(args).await,
            _ => Err(CliError::UnknownCommand),
        }
    }
}

impl XcbKeyModule {
    /// Create a new account with a random private key
    async fn create_account(&self, args: Vec<String>) -> Result<Response, CliError> {
        let password = self.get_password(args)?;
        let key = self.generate_keyfile(password).await?;
        self.add_account_to_list(&key.0, key.1).await;
        Ok(self.format_keyfile_response(&key.0))
    }

    /// Create a new account from a provided private key
    async fn create_account_from_key(&self, args: Vec<String>) -> Result<Response, CliError> {
        let (key, password) = self.get_key_and_password(args)?;
        let key = self.encrypt_keyfile(key, password).await?;
        self.add_account_to_list(&key.0, key.1).await;
        Ok(self.format_keyfile_response(&key.0))
    }

    /// List all accounts in the keystore directory
    async fn list_accounts(&self) -> Result<Response, CliError> {
        let path = PathBuf::from(&self.accounts_dir);
        for entry in fs::read_dir(&path).map_err(CliError::IoError)? {
            let entry = entry.map_err(CliError::IoError)?;
            let path = entry.path();
            if path.is_file() {
                if let Ok(account) = self.read_keyfile(path).await {
                    self.accounts.add_account(account);
                }
            }
        }
        Ok(Response::Accounts(self.accounts.get_accounts()))
    }

    /// Unlock account with provided core ID and password
    /// If the account is found, decrypt the wallet and unlock it
    async fn unlock_account(&self, args: Vec<String>) -> Result<Response, CliError> {
        let (core_id, password) = self.get_core_id_and_password(args)?;
        let account = self
            .accounts
            .get_account(&core_id)
            .ok_or(CliError::AccountNotFound(core_id.clone()))?;
        let wallet = LocalWallet::decrypt_keystore(&account.path, password, self.network_id)
            .map_err(CliError::WalletError)?;
        self.accounts.unlock_account(&account.address, wallet);
        Ok(Response::String(format!(
            "Account {} successfully unlocked!",
            core_id
        )))
    }

    /// Sign a message with the private key of the account
    /// If the account is found - returns an error that it is not found
    /// If the account is locked - returns an error that it is locked
    async fn sign(&self, args: Vec<String>) -> Result<Response, CliError> {
        let (core_id, message) = self.get_core_id_and_message(args)?;
        let account = self
            .accounts
            .get_account(&core_id)
            .ok_or(CliError::AccountNotFound(core_id.clone()))?;
        if !account.is_unlocked() {
            return Err(CliError::AccountNotUnlocked(core_id.clone()));
        }
        let signature = account
            .wallet
            .unwrap()
            .sign_message(message.as_bytes())
            .await?;
        Ok(Response::String(signature.sig().encode_hex()))
    }

    /// Verify that message was signed with the private key of the account
    async fn verify(&self, args: Vec<String>) -> Result<Response, CliError> {
        let (address, signature, message) = self.get_address_signature_and_message(args)?;
        let signature =
            Signature::from_str(&signature).map_err(|_| CliError::InvalidHexArgument(signature))?;
        let verified = signature
            .recover_address_from_msg(message, self.network_id)
            .map_err(|e| CliError::InvalidHexArgument(e.to_string()))?;
        if verified.to_string() == address {
            Ok(Response::String("Signature is valid".to_string()))
        } else {
            Ok(Response::String("Signature is invalid".to_string()))
        }
    }

    /// Inspect the account with the provided core ID
    /// If the account is not unlocked - returns an error
    /// Otherwise, returns the core ID, public key, and private key
    async fn inspect(&self, args: Vec<String>) -> Result<Response, CliError> {
        let core_id = self.get_core_id(args)?;
        let account = self
            .accounts
            .get_account(&core_id)
            .ok_or(CliError::AccountNotFound(core_id.clone()))?;
        if !account.is_unlocked() {
            return Ok(Response::String("Account is locked".to_string()));
        }
        Ok(Response::Keyfile(KeyFile::from_wallet(
            account.wallet.as_ref().unwrap(),
        )))
    }
}
