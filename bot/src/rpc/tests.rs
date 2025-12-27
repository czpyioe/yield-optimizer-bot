#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};
    use alloy::providers::{Provider, ProviderBuilder};
    use crate::rpc::manager::RpcManager;

    use crate::rpc::*;

    #[tokio::test]
    async fn test_load_env_rpcs() {
        let result = loader::load_rpcs_url().await;
        assert!(result.is_ok());
        let rpcs = result.unwrap();
        assert!(!rpcs.is_empty(), "Should load at least one RPC from env");
    }

    #[tokio::test]
    async fn test_rpc_manager_initialization() {
        let start = Instant::now();
        let health_check_interval = Duration::from_secs(5*60);
        let rpc_fetch_interval = Duration::from_secs(60*60);
        let mut manager = RpcManager::new(health_check_interval,rpc_fetch_interval);
        let result = manager.initialize().await;
        assert!(result.is_ok());
        let (total_rpc,healthy_rpc, avg_latency) = utils::stats(&manager);
        println!("Total rpc: {}\nTotal healthy rpc: {}",total_rpc,healthy_rpc);
        if avg_latency.is_some(){
            println!("Average latency: {}ms", avg_latency.unwrap());
        }
        println!("Time elapsed: {}s", start.elapsed().as_secs());
    }

    #[tokio::test]
    async fn test_get_provider() {
        let health_check_interval = Duration::from_secs(5*60);
        let rpc_fetch_interval = Duration::from_secs(60*60);
        let mut manager = RpcManager::new(health_check_interval,rpc_fetch_interval);
        manager.initialize().await.unwrap();
        
        let provider = manager.get_provider().await;
        assert!(provider.is_ok(), "Should return a valid provider");
    }

    #[tokio::test]
    async fn test_provider_get_block() {
        let health_check_interval = Duration::from_secs(5*60);
        let rpc_fetch_interval = Duration::from_secs(60*60);
        let mut manager = RpcManager::new(health_check_interval,rpc_fetch_interval);
        manager.initialize().await.unwrap();
        
        let provider = manager.get_provider().await.unwrap();
        let block_number = provider.get_block_number().await;
        
        assert!(block_number.is_ok(), "Should fetch block number");
        println!("Current block: {}", block_number.unwrap());
    }
}