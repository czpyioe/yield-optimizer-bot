use alloy::contract;
use alloy::providers::Provider;
use alloy::primitives::Address;
use anyhow::Result;

use crate::contracts::addresses::{AaveContract,Network};

use crate::contracts::bindings::aave::AaveProtocolDataProvider;
use crate::contracts::addresses;

use crate::db::models::Apy_snapshot;


pub fn get_aave_contract<P: Provider>(
    provider: P,
    network: Network,
    aave_contract:AaveContract
) -> Result<AaveProtocolDataProvider::AaveProtocolDataProviderInstance<P>> {
    let address = network.get_aave_contract_address(aave_contract)?;
    Ok(AaveProtocolDataProvider::new(address, provider))
}

pub async fn get_apy_snapshot<P:Provider>(provider:P, network: Network,asset_address:Address)->Result<Apy_snapshot>{
    let contract = get_aave_contract(provider, network,AaveContract::AaveProtocolDataProvider)?;
    let reserve_data = contract.getReserveData(asset_address).call().await?;

    let apy = ray_to_apy(reserve_data.liquidityRate.to::<u128>());
    
    Ok(Apy_snapshot { 
        protocol: "aave".to_string(), 
        network: network.name()?, 
        asset_address: format!("{:?}", asset_address), 
        apy: Some(apy) 
    })
}

// helper
fn ray_to_apy(rate: u128) -> f64 {
    const RAY: f64 = 1e27;
    const SECONDS_PER_YEAR: f64 = 31536000.0;
    
    let rate_per_second = rate as f64 / RAY;
    ((1.0 + rate_per_second).powf(SECONDS_PER_YEAR) - 1.0) * 100.0
}

