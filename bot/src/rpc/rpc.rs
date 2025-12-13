use alloy::providers::{Provider, ProviderBuilder};


pub async fn connect_provider() -> Result<u64, Box<dyn std::error::Error>>{
    let rpc_url = "https://eth.llamarpc.com";
    let provider = ProviderBuilder::new().connect(rpc_url).await?;

    let chain_id = provider.get_chain_id().await?;
    Ok(chain_id)
}
