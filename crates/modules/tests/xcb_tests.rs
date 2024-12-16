#[cfg(test)]
mod tests {
    use atoms_rpc_types::Block;
    use cli_error::CliError;
    use modules::{Module, XcbModule};
    use rpc::MockRpcClient;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use types::Response;

    fn get_module() -> XcbModule {
        let mut block = Block::default();
        block.header.number = Some(100);

        let mock = MockRpcClient::new()
            .with_block_by_hash(block.clone())
            .with_block_by_number(block.clone())
            .with_block_latest(block.clone())
            .with_block_height(100)
            .with_energy_price(1000)
            .with_network_id(999);
        let client = Arc::new(Mutex::new(mock));
        XcbModule::new(client)
    }

    #[tokio::test]
    async fn test_execute_get_block_height() {
        let mut module = get_module();

        let response = module
            .execute("get_block_height".to_string(), vec![])
            .await
            .unwrap();
        assert_eq!(response, Response::U64(100));
    }

    #[tokio::test]
    async fn test_execute_get_block_latest() {
        let mut module = get_module();

        let response = module
            .execute("get_block".to_string(), vec!["latest".to_string()])
            .await
            .unwrap();

        // The block number is set to 100 in the mock client
        if let Response::Block(block) = response {
            assert_eq!(block.header.number, Some(100));
        } else {
            panic!("Expected Response::Block");
        }
    }

    #[tokio::test]
    async fn test_execute_get_block_by_hash() {
        let mut module = get_module();

        let response = module
            .execute(
                "get_block".to_string(),
                vec![
                    "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
                ],
            )
            .await
            .unwrap();

        // The block number is set to 100 in the mock client
        if let Response::Block(block) = response {
            assert_eq!(block.header.number, Some(100));
        } else {
            panic!("Expected Response::Block");
        }
    }

    #[tokio::test]
    async fn test_execute_get_block_by_number() {
        let mut module = get_module();

        let response = module
            .execute("get_block".to_string(), vec!["100".to_string()])
            .await
            .unwrap();

        if let Response::Block(block) = response {
            assert_eq!(block.header.number, Some(100));
        } else {
            panic!("Expected Response::Block");
        }
    }

    #[tokio::test]
    async fn test_execute_invalid_block_argument() {
        let mut module = get_module();

        let response = module
            .execute("get_block".to_string(), vec!["invalid".to_string()])
            .await;
        assert!(matches!(response, Err(CliError::InvalidArgument(_, _))));
    }

    #[tokio::test]
    async fn test_execute_get_energy_price() {
        let mut module = get_module();

        let response = module
            .execute("get_energy_price".to_string(), vec![])
            .await
            .unwrap();
        assert_eq!(response, Response::U128(1000));
    }

    #[tokio::test]
    async fn test_execute_get_network_id() {
        let mut module = get_module();

        let response = module
            .execute("get_network_id".to_string(), vec![])
            .await
            .unwrap();
        assert_eq!(response, Response::U64(999));
    }

    #[tokio::test]
    async fn test_execute_unknown_command() {
        let mut module = get_module();

        let response = module.execute("unknown_command".to_string(), vec![]).await;
        assert!(matches!(response, Err(CliError::UnknownCommand)));
    }

    #[tokio::test]
    async fn test_cli_get_block_zero_arguments() {
        let mut module = get_module();

        let response = module.execute("get_block".to_string(), vec![]).await;
        assert!(matches!(
            response,
            Err(CliError::InvalidNumberOfArguments(_))
        ));
    }
}