use alloy::providers::Provider;
use anyhow::Result;
use crate::network_choice;

use crate::contracts::bindings::aave::AaveProtocolDataProvider;
use crate::contracts::addresses;

pub async fn get_aave_tokens_reserves<P: Provider>(provider: P, network: network_choice) -> Result<()> {
    let mainnet_addresses = addresses::Addresses::get_mainnet_addresses()?;
    let aave_protocol_data_provider_address = match network{
        network_choice::Ethereum=>mainnet_addresses.eth_aave_protocol_data_provider,
        network_choice::Arbitrum=>mainnet_addresses.arb_aave_protocol_data_provider,
        _ => anyhow::bail!("Unsupported network for Aave")
    };
    
    let contract = AaveProtocolDataProvider::new(aave_protocol_data_provider_address, provider);
    
    let all_reserves = contract.getAllReservesTokens().call().await?;

    
    println!("Found {} reserves:", all_reserves.len());
    for i in all_reserves.iter() {
        println!("  {} -> {:?}", i.symbol, i.tokenAddress);
    }
    
    Ok(())
}