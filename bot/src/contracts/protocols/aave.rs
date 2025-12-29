use alloy::contract;
use alloy::providers::Provider;
use alloy::primitives::Address;
use anyhow::Result;

use crate::contracts::addresses::{Asset, AaveContract, Network,Protocol};

use crate::contracts::bindings::aave::AaveProtocolDataProvider;
use crate::contracts::addresses;

use crate::db::models::ApySnapshot;


pub fn get_aave_contract<P: Provider>(
    provider: P,
    network: Network,
    aave_contract:AaveContract
) -> Result<AaveProtocolDataProvider::AaveProtocolDataProviderInstance<P>> {
    let address = network.get_aave_contract_address(aave_contract)?;
    Ok(AaveProtocolDataProvider::new(address, provider))
}

pub async fn get_apy_snapshot<P:Provider>(provider:P, network: Network,asset:Asset)->Result<ApySnapshot>{
    let contract = get_aave_contract(provider, network,AaveContract::AaveProtocolDataProvider)?;
    let asset_address = network.get_asset_address_aave(asset)?;
    let reserve_data = contract.getReserveData(asset_address).call().await?;

    let apy = ray_to_apy(reserve_data.liquidityRate.to::<u128>());
    
    Ok(ApySnapshot { 
        protocol: Protocol::Aave.name().to_string(), 
        network: network.name().to_string(), 
        asset: asset.name().to_string(), 
        apy: Some(apy) 
    })
}

// helper
fn ray_to_apy(rate: u128) -> f64 {
    const RAY: f64 = 1e27;
    (rate as f64 / RAY) * 100.0
}

