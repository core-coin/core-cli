#[cfg(test)]
mod tests {
    use atoms_signer_wallet::LocalWallet;
    use chrono::Utc;
    use std::path::PathBuf;
    use types::{
        account::{Accounts, KeyFile},
        Account,
    };

    #[test]
    fn test_account_new() {
        let account = Account::new(
            "0x123".to_string(),
            None,
            PathBuf::from("/path/to/keyfile"),
            0,
        );
        assert_eq!(account.address, "0x123");
        assert!(account.wallet.is_none());
        assert_eq!(account.path, PathBuf::from("/path/to/keyfile"));
        assert_eq!(account.unlocked, 0);
    }

    #[test]
    fn test_account_is_unlocked() {
        let account = Account::new(
            "0x123".to_string(),
            None,
            PathBuf::from("/path/to/keyfile"),
            Utc::now().timestamp() + 100,
        );
        assert!(account.is_unlocked());
    }

    #[test]
    fn test_account_is_locked() {
        let account = Account::new(
            "0x123".to_string(),
            None,
            PathBuf::from("/path/to/keyfile"),
            Utc::now().timestamp() - 100,
        );
        assert!(!account.is_unlocked());
    }

    #[test]
    fn test_account_is_unlocked_str() {
        let account = Account::new(
            "0x123".to_string(),
            None,
            PathBuf::from("/path/to/keyfile"),
            Utc::now().timestamp() + 100,
        );
        assert_eq!(account.is_unlocked_str(), "🔓 Unlocked");
    }

    #[test]
    fn test_account_is_locked_str() {
        let account = Account::new(
            "0x123".to_string(),
            None,
            PathBuf::from("/path/to/keyfile"),
            Utc::now().timestamp() - 100,
        );
        assert_eq!(account.is_unlocked_str(), "🔒 Locked");
    }

    #[test]
    fn test_accounts_add_account() {
        let accounts = Accounts::new(vec![]);
        let account = Account::new(
            "0x123".to_string(),
            None,
            PathBuf::from("/path/to/keyfile"),
            0,
        );
        accounts.add_account(account.clone());
        assert_eq!(accounts.get_accounts().len(), 1);
        assert_eq!(accounts.get_accounts()[0], account);
    }

    #[test]
    fn test_accounts_remove_account() {
        let accounts = Accounts::new(vec![]);
        let account = Account::new(
            "0x123".to_string(),
            None,
            PathBuf::from("/path/to/keyfile"),
            0,
        );
        accounts.add_account(account.clone());
        accounts.remove_account("0x123");
        assert!(accounts.get_accounts().is_empty());
    }

    #[test]
    fn test_accounts_get_account() {
        let accounts = Accounts::new(vec![]);
        let account = Account::new(
            "0x123".to_string(),
            None,
            PathBuf::from("/path/to/keyfile"),
            0,
        );
        accounts.add_account(account.clone());
        let retrieved_account = accounts.get_account("0x123").unwrap();
        assert_eq!(retrieved_account, account);
    }

    #[test]
    fn test_accounts_unlock_account() {
        let accounts = Accounts::new(vec![]);
        let account = Account::new(
            "0x123".to_string(),
            None,
            PathBuf::from("/path/to/keyfile"),
            0,
        );
        accounts.add_account(account.clone());
        let wallet = LocalWallet::random(1);
        accounts.unlock_account("0x123", wallet.clone());
        let unlocked_account = accounts.get_account("0x123").unwrap();
        assert!(unlocked_account.is_unlocked());
        assert_eq!(unlocked_account.wallet.unwrap(), wallet);
    }

    #[test]
    fn test_accounts_check_outdated() {
        let accounts = Accounts::new(vec![]);
        let account = Account::new(
            "0x123".to_string(),
            Some(LocalWallet::random(1)),
            PathBuf::from("/path/to/keyfile"),
            Utc::now().timestamp() - 100,
        );
        accounts.add_account(account.clone());
        let outdated: Account = accounts.get_account("0x123").unwrap();

        assert_eq!(outdated.wallet, None)
    }

    #[test]
    fn test_keyfile_new() {
        let keyfile = KeyFile::new(
            "0x123".to_string(),
            "public_key".to_string(),
            "private_key".to_string(),
        );
        assert_eq!(keyfile.address, "0x123");
        assert_eq!(keyfile.public_key, "public_key");
        assert_eq!(keyfile.private_key, "private_key");
    }

    #[test]
    fn test_keyfile_from_wallet() {
        let wallet = LocalWallet::random(1);
        let keyfile = KeyFile::from_wallet(&wallet);
        assert_eq!(keyfile.address, wallet.address().to_string());
        assert_eq!(
            keyfile.public_key,
            hex::encode(wallet.signer().verifying_key().as_bytes())
        );
        assert_eq!(keyfile.private_key, hex::encode(wallet.signer().to_bytes()));
    }

    #[test]
    fn test_keyfile_to_string() {
        let keyfile = KeyFile::new(
            "0x123".to_string(),
            "public_key".to_string(),
            "private_key".to_string(),
        );
        assert_eq!(
            keyfile.to_string(),
            "Address: 0x123\nPublic key: public_key\nPrivate key: private_key"
        );
    }
}
