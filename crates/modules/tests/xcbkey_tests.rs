#[cfg(test)]
mod tests {
    use cli_error::CliError;
    use modules::{Module, XcbKeyModule};
    use rpc::MockRpcClient;
    use std::sync::Arc;
    use std::{env, path::PathBuf};
    use tokio::sync::Mutex;
    use types::account::Accounts;
    use types::response::Response;
    use utils::utils::{create_tmp_dir, remove_tmp_dir};

    struct TestContext {
        datadir: PathBuf,
        pub module: XcbKeyModule,
    }

    impl TestContext {
        async fn new() -> Self {
            let datadir = create_tmp_dir(None).to_path_buf();
            let mock = MockRpcClient::new();
            // create a tmp directory for the keystore
            let accounts = Accounts::new(vec![]);
            let client = Arc::new(Mutex::new(mock));
            let module = XcbKeyModule::new(client, datadir.display().to_string(), accounts).await;

            TestContext {
                datadir: datadir,
                module: module,
            }
        }
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            let current = env::current_dir().unwrap();
            remove_tmp_dir(current.join(" ").join(self.datadir.clone())).unwrap();
        }
    }

    /// Test the XcbKeyModule workflow
    /// 1. Create a new account
    /// 2. Check if the account is added to the list
    /// 3. Check if the account is locked and cannot be inspected or signed
    /// 4. Unlock the account
    /// 5. Check if the account is unlocked
    /// 6. Check if the account can be inspected
    /// 7. Check if the account can sign a message
    /// 8. Check if the signature is valid
    /// 9. Check if invalid signature is not verified
    /// 10. Add another account with the predefined private key
    /// 11. Check if the account is added to the list
    /// 12. Verify the account's address, private key and public key
    #[tokio::test]
    async fn test_xcbkey_workflow() {
        let mut context = TestContext::new().await;

        let list = context
            .module
            .execute("list".to_string(), vec![])
            .await
            .unwrap();
        // check that initially there are no accounts
        assert_eq!(list, Response::Accounts(vec![]));
        let initial: Response = context
            .module
            .execute("new".to_string(), vec!["password".to_string()])
            .await
            .unwrap();

        if let Response::Keyfile(keyfile) = initial.clone() {
            assert_eq!(keyfile.address.len(), 44); // Check if core_id is a valid address
            assert_eq!(keyfile.private_key.len(), 114); // Check if private key is valid
            assert_eq!(keyfile.public_key.len(), 114); // Check if public key is valid
        } else {
            panic!("Expected Response::Keyfile");
        }

        let list = context
            .module
            .execute("list".to_string(), vec![])
            .await
            .unwrap();
        // check that the account was added
        if let Response::Accounts(accounts) = list {
            assert_eq!(accounts.len(), 1);
            if let Response::Keyfile(keyfile) = initial.clone() {
                assert_eq!(accounts[0].address, keyfile.address); // check if the address is the same
                assert_eq!(accounts[0].wallet, None); // check if the wallet is None (account is locked)
                assert_eq!(accounts[0].is_unlocked(), false); // check if the account is locked
            }

            // check that we cannot inspect, sign or unlock the account with wrong password

            let inspect = context
                .module
                .execute("inspect".to_string(), vec![accounts[0].address.clone()])
                .await
                .unwrap();

            if let Response::String(result) = inspect {
                assert_eq!(result, "Account is locked");
            } else {
                panic!("Expected Response::String");
            }

            let sign = context
                .module
                .execute(
                    "sign".to_string(),
                    vec![accounts[0].address.clone(), "message".to_string()],
                )
                .await;

            if let Err(CliError::AccountNotUnlocked(account)) = sign {
                assert_eq!(account, accounts[0].address); // check if the account is locked
            } else {
                panic!("Expected CliError::AccountNotUnlocked");
            }

            let unlock = context
                .module
                .execute(
                    "unlock".to_string(),
                    vec![accounts[0].address.clone(), "wrong_password".to_string()],
                )
                .await;

            if let Err(CliError::InvalidPassword) = unlock {
                // check if the password is invalid
            } else {
                panic!("Expected CliError::InvalidPassword");
            }

            // unlock the account and do the same checks
            let unlock = context
                .module
                .execute(
                    "unlock".to_string(),
                    vec![accounts[0].address.clone(), "password".to_string()],
                )
                .await
                .unwrap();

            if let Response::String(result) = unlock {
                assert_eq!(
                    result,
                    format!("Account {} successfully unlocked!", accounts[0].address)
                );
            } else {
                panic!("Expected Response::String");
            }

            let unlocked_list = context
                .module
                .execute("list".to_string(), vec![])
                .await
                .unwrap();
            if let Response::Accounts(accounts) = unlocked_list {
                assert_eq!(accounts.len(), 1);
                assert_eq!(accounts[0].is_unlocked(), true); // check if the account is unlocked
                assert_ne!(accounts[0].wallet, None); // check if the wallet is Some (account is unlocked)
            }

            let inspect = context
                .module
                .execute("inspect".to_string(), vec![accounts[0].address.clone()])
                .await
                .unwrap();

            if let Response::Keyfile(keyfile) = inspect {
                if let Response::Keyfile(initial) = initial {
                    assert_eq!(keyfile, initial); // check if the keyflie is the same
                }
            } else {
                panic!("Expected Response::String");
            }

            let sign = context
                .module
                .execute(
                    "sign".to_string(),
                    vec![accounts[0].address.clone(), "message".to_string()],
                )
                .await
                .unwrap();

            if let Response::String(result) = sign {
                assert_eq!(result.len(), 342); // check if the signature is valid

                let verify = context
                    .module
                    .execute(
                        "verify".to_string(),
                        vec![
                            accounts[0].address.clone(),
                            result.clone(),
                            "message".to_string(),
                        ],
                    )
                    .await
                    .unwrap();

                if let Response::String(result) = verify {
                    assert_eq!(result, "Signature is valid"); // check if the signature is valid
                } else {
                    panic!("Expected Response::String");
                }

                let verify = context
                    .module
                    .execute(
                        "verify".to_string(),
                        vec![
                            accounts[0].address.clone(),
                            "0x11".to_string(),
                            "message".to_string(),
                        ],
                    )
                    .await;

                if let Err(CliError::InvalidSignature) = verify {
                    // check if the signature is invalid
                } else {
                    panic!("Expected CliError::InvalidSignature");
                }
            } else {
                panic!("Expected Response::String");
            }
        } else {
            panic!("Expected Response::Accounts");
        }

        // add another account for existing private key and check if it's added

        let new = context.module
            .execute("new_from_key".to_string(), vec!["000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string(), "password".to_string()])
            .await
            .unwrap();

        if let Response::Keyfile(keyfile) = new.clone() {
            assert_eq!(
                keyfile.address,
                "ce6759015272cf0154d91651a637773ae68daa02dbbf"
            ); // Check if core_id is a valid address
            assert_eq!(keyfile.private_key, "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"); // Check if private key is valid
            assert_eq!(keyfile.public_key, "5b3afe03878a49b28232d4f1a442aebde109f807acef7dfd9a7f65b962fe52d6547312cacecff04337508f9d2529a8f1669169b21c32c48000");
        // Check if public key is valid
        } else {
            panic!("Expected Response::Keyfile");
        }

        let list = context
            .module
            .execute("list".to_string(), vec![])
            .await
            .unwrap();

        if let Response::Accounts(accounts) = list {
            assert_eq!(accounts.len(), 2);
        } else {
            panic!("Expected Response::Accounts");
        }
    }

    #[tokio::test]
    async fn test_create_two_new() {
        let mut context = TestContext::new().await;

        let response = context
            .module
            .execute("new".to_string(), vec!["password".to_string()])
            .await
            .unwrap();
        if let Response::Keyfile(keyfile) = response {
            assert_eq!(keyfile.address.len(), 44); // Check if core_id is a valid address
            assert_eq!(keyfile.private_key.len(), 114); // Check if private key is valid
            assert_eq!(keyfile.public_key.len(), 114); // Check if public key is valid
        } else {
            panic!("Expected Response::Keyfile");
        }

        let response = context
            .module
            .execute("new".to_string(), vec!["password".to_string()])
            .await
            .unwrap();

        if let Response::Keyfile(keyfile) = response {
            assert_eq!(keyfile.address.len(), 44); // Check if core_id is a valid address
            assert_eq!(keyfile.private_key.len(), 114); // Check if private key is valid
            assert_eq!(keyfile.public_key.len(), 114); // Check if public key is valid
        } else {
            panic!("Expected Response::Keyfile");
        }

        let list = context
            .module
            .execute("list".to_string(), vec![])
            .await
            .unwrap();
        if let Response::Accounts(accounts) = list {
            assert_eq!(accounts.len(), 2);
        } else {
            panic!("Expected Response::Accounts");
        }
    }

    #[tokio::test]
    async fn test_create_new_bad_arg_list() {
        let mut context = TestContext::new().await;

        let response = context
            .module
            .execute(
                "new".to_string(),
                vec!["pass".to_string(), "bad_arg".to_string()],
            )
            .await;
        assert!(response.is_err()); // Check if creating an account with no password returns an error
    }

    #[tokio::test]
    async fn test_create_account_from_key() {
        let mut context = TestContext::new().await;

        let response = context.module
            .execute("new_from_key".to_string(), vec!["000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string(), "password".to_string()])
            .await
            .unwrap();
        if let Response::Keyfile(keyfile) = response {
            assert_eq!(
                keyfile.address,
                "ce6759015272cf0154d91651a637773ae68daa02dbbf"
            ); // Check if core_id is a valid address
            assert_eq!(keyfile.private_key, "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"); // Check if private key is valid
            assert_eq!(keyfile.public_key, "5b3afe03878a49b28232d4f1a442aebde109f807acef7dfd9a7f65b962fe52d6547312cacecff04337508f9d2529a8f1669169b21c32c48000");
        } else {
            panic!("Expected Response::Keyfile");
        }
    }

    #[tokio::test]
    async fn test_create_account_from_bad_key() {
        let mut context = TestContext::new().await;

        let response = context
            .module
            .execute(
                "new_from_key".to_string(),
                vec!["bad_key".to_string(), "password".to_string()],
            )
            .await;

        if let Err(CliError::InvalidPrivateKey) = response {
            assert!(true)
        } else {
            panic!("Expected CliError::InvalidPrivateKey");
        }
    }

    #[tokio::test]
    async fn test_create_account_from_key_bad_arg_list() {
        let mut context = TestContext::new().await;

        let response = context
            .module
            .execute(
                "new_from_key".to_string(),
                vec![
                    "0x00".to_string(),
                    "password".to_string(),
                    "bad_arg".to_string(),
                ],
            )
            .await;
        assert!(response.is_err()); // Check if creating an account with an invalid argument list returns an error
    }

    #[tokio::test]
    async fn test_list_accounts() {
        let mut context = TestContext::new().await;

        let response = context
            .module
            .execute("list".to_string(), vec![])
            .await
            .unwrap();
        if let Response::Accounts(accounts) = response {
            assert!(accounts.is_empty()); // Check if accounts list is empty
        } else {
            panic!("Expected Response::Accounts");
        }

        context
            .module
            .execute("new".to_string(), vec!["password".to_string()])
            .await
            .unwrap();

        let response = context
            .module
            .execute("list".to_string(), vec![])
            .await
            .unwrap();
        if let Response::Accounts(accounts) = response {
            assert_eq!(accounts.len(), 1); // Check if accounts list has one account
        } else {
            panic!("Expected Response::Accounts");
        }
    }

    #[tokio::test]
    async fn test_inspect_account() {
        let mut context = TestContext::new().await;

        let response = context
            .module
            .execute("inspect".to_string(), vec!["core_id".to_string()])
            .await;
        assert!(response.is_err()); // Check if inspecting a non-existing account returns an error

        let response = context
            .module
            .execute("new".to_string(), vec!["password".to_string()])
            .await
            .unwrap();
        if let Response::Keyfile(keyfile) = response {
            let response = context
                .module
                .execute("inspect".to_string(), vec![keyfile.address.clone()])
                .await
                .unwrap();
            if let Response::String(result) = response {
                assert_eq!(result, "Account is locked"); // Check if inspecting a locked account returns the correct message
            } else {
                panic!("Expected Response::String");
            }

            let unlock = context
                .module
                .execute(
                    "unlock".to_string(),
                    vec![keyfile.address.clone(), "password".to_string()],
                )
                .await
                .unwrap();

            if let Response::String(result) = unlock {
                assert_eq!(
                    result,
                    format!("Account {} successfully unlocked!", keyfile.address)
                );
            } else {
                panic!("Expected Response::String");
            }

            let response = context
                .module
                .execute("inspect".to_string(), vec![keyfile.address.clone()])
                .await
                .unwrap();
            if let Response::Keyfile(inspected) = response {
                assert_eq!(inspected, keyfile); // Check if inspecting an account returns the correct keyfile
            } else {
                panic!("Expected Response::Keyfile");
            }
        }
    }

    #[tokio::test]
    async fn test_inspect_account_bad_arg_list() {
        let mut context = TestContext::new().await;

        let response = context
            .module
            .execute(
                "inspect".to_string(),
                vec!["core_id".to_string(), "bad_arg".to_string()],
            )
            .await;
        assert!(response.is_err()); // Check if inspecting an account with an invalid argument list returns an error
    }

    #[tokio::test]
    async fn test_sign_verify() {
        let mut context = TestContext::new().await;

        let response = context
            .module
            .execute("new".to_string(), vec!["password".to_string()])
            .await
            .unwrap();

        if let Response::Keyfile(keyfile) = response {
            let response = context
                .module
                .execute(
                    "sign".to_string(),
                    vec![keyfile.address.clone(), "message".to_string()],
                )
                .await;

            // sign must return an error if the account is locked
            if let Err(CliError::AccountNotUnlocked(account)) = response {
                assert_eq!(account, keyfile.address); // check if the account is locked
            } else {
                panic!("Expected CliError::AccountNotUnlocked");
            }

            context
                .module
                .execute(
                    "unlock".to_string(),
                    vec![keyfile.address.clone(), "password".to_string()],
                )
                .await
                .unwrap();

            let response = context
                .module
                .execute(
                    "sign".to_string(),
                    vec![keyfile.address.clone(), "message".to_string()],
                )
                .await;

            if let Response::String(result) = response.unwrap() {
                assert_eq!(result.len(), 342); // check if the signature is valid

                let response = context
                    .module
                    .execute(
                        "verify".to_string(),
                        vec![
                            keyfile.address.clone(),
                            result.clone(),
                            "message".to_string(),
                        ],
                    )
                    .await
                    .unwrap();

                if let Response::String(result) = response {
                    assert_eq!(result, "Signature is valid"); // check if the signature is valid
                } else {
                    panic!("Expected Response::String");
                }

                let response = context
                    .module
                    .execute(
                        "verify".to_string(),
                        vec![
                            keyfile.address.clone(),
                            "0x11".to_string(),
                            "message".to_string(),
                        ],
                    )
                    .await;

                if let Err(CliError::InvalidSignature) = response {
                    // check if the signature is invalid
                } else {
                    panic!("Expected CliError::InvalidSignature");
                }
            } else {
                panic!("Expected Response::String");
            }
        } else {
            panic!("Expected Response::Keyfile");
        }
    }
}
