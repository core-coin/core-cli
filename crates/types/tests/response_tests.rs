// FILE: response_tests.rs

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use atoms_rpc_types::Block;
    use serde_json::json;
    use types::{
        account::{Account, KeyFile},
        Response, ResponseView,
    };

    #[test]
    fn test_response_format_string() {
        let response = Response::String("test".to_string());
        assert_eq!(response.format(ResponseView::String), "test");
    }

    #[test]
    fn test_response_format_json() {
        let response = Response::String("test".to_string());
        assert_eq!(response.format(ResponseView::Json), "{\"String\":\"test\"}");
    }

    #[test]
    fn test_response_format_human() {
        let response = Response::String("test".to_string());
        assert_eq!(
            response.format(ResponseView::Human),
            "String value: \"test\""
        );
    }

    #[test]
    fn test_response_u64() {
        let response = Response::U64(100);
        assert_eq!(response.format(ResponseView::Human), "u64 value: 100");
    }

    #[test]
    fn test_response_u128() {
        let response = Response::U128(1000);
        assert_eq!(response.format(ResponseView::Json), "{\"U128\":1000}");
    }

    #[test]
    fn test_response_bool() {
        let response = Response::Bool(true);
        assert_eq!(response.format(ResponseView::Json), "{\"Bool\":true}");
    }

    #[test]
    fn test_response_block() {
        let block = Block::default();
        let response = Response::Block(block.clone());
        assert_eq!(
            response.format(ResponseView::Human),
            "Block {\n    header: Header {\n        hash: None,\n        parent_hash: 0x0000000000000000000000000000000000000000000000000000000000000000,\n        uncles_hash: 0x0000000000000000000000000000000000000000000000000000000000000000,\n        miner: 0x00000000000000000000000000000000000000000000,\n        state_root: 0x0000000000000000000000000000000000000000000000000000000000000000,\n        transactions_root: 0x0000000000000000000000000000000000000000000000000000000000000000,\n        receipts_root: 0x0000000000000000000000000000000000000000000000000000000000000000,\n        logs_bloom: 0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,\n        difficulty: 0x0_U256,\n        number: None,\n        energy_limit: 0,\n        energy_used: 0,\n        timestamp: 0,\n        total_difficulty: None,\n        extra_data: 0x,\n        mix_hash: None,\n        nonce: None,\n        base_fee_per_gas: None,\n        withdrawals_root: None,\n        blob_gas_used: None,\n        excess_blob_gas: None,\n        parent_beacon_block_root: None,\n        requests_root: None,\n    },\n    uncles: [],\n    transactions: Hashes(\n        [],\n    ),\n    size: None,\n    withdrawals: None,\n    other: OtherFields {},\n}"
        );
    }

    #[test]
    fn test_response_struct() {
        let value = json!({"key": "value"});
        let response = Response::Struct(value.clone());
        assert_eq!(
            response.format(ResponseView::Json),
            "{\"Struct\":{\"key\":\"value\"}}"
        );
    }

    #[test]
    fn test_response_accounts() {
        let account = Account::new(
            "0x123".to_string(),
            None,
            PathBuf::from("/path/to/keyfile"),
            0,
        );
        let response = Response::Accounts(vec![account.clone()]);
        assert_eq!(
            response.format(ResponseView::Human),
            "Accounts:\n1: 0x123 . File - /path/to/keyfile. 🔒 Locked\n"
        );
    }

    #[test]
    fn test_response_keyfile() {
        let keyfile = KeyFile::new(
            "0x123".to_string(),
            "public_key".to_string(),
            "private_key".to_string(),
        );
        let response = Response::Keyfile(keyfile.clone());
        assert_eq!(
            response.format(ResponseView::Human),
            "Address: 0x123\nPublic key: public_key\nPrivate key: private_key"
        );
    }

    #[test]
    fn test_response_view_from_str() {
        assert_eq!(ResponseView::from_str("string"), Ok(ResponseView::String));
        assert_eq!(ResponseView::from_str("json"), Ok(ResponseView::Json));
        assert_eq!(ResponseView::from_str("human"), Ok(ResponseView::Human));
        assert_eq!(ResponseView::from_str("invalid"), Err(()));
    }
}
