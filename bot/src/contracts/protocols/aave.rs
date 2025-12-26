use alloy::providers::Provider;
use alloy::primitives::Address;
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


pub async fn get_aave_reserve_data<P: Provider>(provider: P, network: network_choice, asset_address:Address) -> Result<()> {
    let mainnet_addresses = addresses::Addresses::get_mainnet_addresses()?;
    let aave_protocol_data_provider_address = match network{
        network_choice::Ethereum=>mainnet_addresses.eth_aave_protocol_data_provider,
        network_choice::Arbitrum=>mainnet_addresses.arb_aave_protocol_data_provider,
        _ => anyhow::bail!("Unsupported network for Aave")
    };
    
    let contract = AaveProtocolDataProvider::new(aave_protocol_data_provider_address, provider);
    
    let reserve_data = contract.getReserveData(asset_address).call().await?;

    println!("{} - {} - {} - {} - {} - {}",reserve_data.unbacked,reserve_data.accruedToTreasuryScaled,
        reserve_data.totalAToken,reserve_data.totalStableDebt,
        reserve_data.totalVariableDebt,reserve_data.liquidityRate);
    Ok(())
}


pub async fn get_aave_user_reserve_data<P: Provider>(provider: P, network: network_choice, asset_address:Address, user_address:Address) -> Result<()> {
    let mainnet_addresses = addresses::Addresses::get_mainnet_addresses()?;
    let aave_protocol_data_provider_address = match network{
        network_choice::Ethereum=>mainnet_addresses.eth_aave_protocol_data_provider,
        network_choice::Arbitrum=>mainnet_addresses.arb_aave_protocol_data_provider,
        _ => anyhow::bail!("Unsupported network for Aave")
    };
    
    let contract = AaveProtocolDataProvider::new(aave_protocol_data_provider_address, provider);
    
    let reserve_data = contract.getUserReserveData(asset_address,user_address).call().await?;

    println!("{} - {}",reserve_data.currentATokenBalance,reserve_data.liquidityRate);
    Ok(())
}


pub async fn get_aave_reserve_configuration_data<P: Provider>(provider: P, network: network_choice, asset_address:Address) -> Result<()> {
    let mainnet_addresses = addresses::Addresses::get_mainnet_addresses()?;
    let aave_protocol_data_provider_address = match network{
        network_choice::Ethereum=>mainnet_addresses.eth_aave_protocol_data_provider,
        network_choice::Arbitrum=>mainnet_addresses.arb_aave_protocol_data_provider,
        _ => anyhow::bail!("Unsupported network for Aave")
    };
    
    let contract = AaveProtocolDataProvider::new(aave_protocol_data_provider_address, provider);
    
    let reserve_data = contract.getReserveConfigurationData(asset_address).call().await?;

    println!("{} - {}",reserve_data.borrowingEnabled,reserve_data.isActive);
    Ok(())
}