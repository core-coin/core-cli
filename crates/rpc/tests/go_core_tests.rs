// FILE: go_core_tests.rs

#[cfg(test)]
mod tests {
    use atoms_rpc_types::{BlockId, SyncStatus};
    use base_primitives::{hex::FromHex, B256};
    use cli_error::CliError;
    use rpc::{GoCoreClient, RpcClient};
    use types::DEFAULT_BACKEND;

    async fn gocore_client() -> GoCoreClient {
        GoCoreClient::new(DEFAULT_BACKEND.to_string())
    }

    #[tokio::test]
    async fn test_get_block_height() {
        let go_core_client = gocore_client().await;

        let response = go_core_client.get_block_height().await.unwrap();
        assert!(response > 10000000);
    }

    #[tokio::test]
    async fn test_get_block_by_hash() {
        let go_core_client = gocore_client().await;

        let response = go_core_client
            .get_block(BlockId::hash(
                B256::from_hex(
                    "0x5e466ba194248a4ed816837cbe9eae56140b20dd64166da5aa932ccf6afe3440",
                )
                .unwrap(),
            ))
            .await
            .unwrap();
        assert_eq!(response.header.number, Some(11416658));

        let response = go_core_client
            .get_block(BlockId::hash(
                B256::from_hex(
                    "0x5e466ba194248a4ed816837cbe9eae56140b20dd64166da5aa932ccf6afe3440",
                )
                .unwrap(),
            ))
            .await
            .unwrap();
        assert_eq!(response.header.number, Some(11416658));
    }

    #[tokio::test]
    async fn test_get_block_by_number() {
        let go_core_client = gocore_client().await;

        let response = go_core_client
            .get_block(BlockId::number(100))
            .await
            .unwrap();
        assert_eq!(response.header.number, Some(100));
    }

    #[tokio::test]
    async fn test_get_block_latest() {
        let go_core_client = gocore_client().await;

        let response = go_core_client
            .get_block(atoms_rpc_types::BlockId::latest())
            .await
            .unwrap();

        assert!(response.header.number > Some(10000000))
    }

    #[tokio::test]
    async fn test_get_energy_price() {
        let go_core_client = gocore_client().await;

        let response = go_core_client.get_energy_price().await.unwrap();
        assert!(response >= 1000000000);
    }

    #[tokio::test]
    async fn test_get_network_id() {
        let go_core_client = gocore_client().await;

        let response = go_core_client.get_network_id().await.unwrap();
        assert_eq!(response, 1);
    }

    #[tokio::test]
    async fn test_get_block_not_found() {
        let go_core_client = gocore_client().await;

        let response = go_core_client.get_block(BlockId::number(999999999)).await;
        assert!(matches!(response, Err(CliError::RpcError(_))));
    }

    #[tokio::test]
    async fn test_syncing() {
        let go_core_client = gocore_client().await;

        let response = go_core_client.syncing().await.unwrap();
        assert_eq!(response, SyncStatus::None);
    }
}
