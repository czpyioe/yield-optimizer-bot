use alloy::contract;
use alloy::providers::Provider;
use alloy::primitives::Address;
use anyhow::Result;

use crate::contracts::addresses::{CompoundContract,Asset,Network,Protocol};

use crate::contracts::bindings::compound::cTokenv3;
use crate::contracts::addresses;

use crate::db::models::ApySnapshot;


pub fn get_compound_contract<P: Provider>(
    provider: P,
    network: Network,
    compound_contract:CompoundContract,
    asset:Asset   
) -> Result<cTokenv3::cTokenv3Instance<P>> {
    let address = network.get_compound_contract_address(compound_contract,asset)?;
    Ok(cTokenv3::new(address, provider))
}

pub async fn get_apy_snapshot<P:Provider>(provider:P, network: Network,asset:Asset)->Result<ApySnapshot>{
    let contract = get_compound_contract(provider, network,CompoundContract::CTokenv3,asset)?;
    let utilization = contract.getUtilization().call().await?;

    let supply_rate = contract.getSupplyRate(utilization).call().await?;

    let apy = supply_rate_to_apy(supply_rate);
    
    Ok(ApySnapshot { 
        protocol: Protocol::Compound.name().to_string(),
        network: network.name().to_string(), 
        asset: asset.name().to_string(), 
        apy: Some(apy) 
    })
}

// helper
fn supply_rate_to_apy(rate: u64) -> f64 {
    const SECONDS_PER_YEAR: f64 = 31_536_000.0;
    let rate_per_second =(rate as f64) / 1e18;

    let apy =((1.0 + rate_per_second).powf(SECONDS_PER_YEAR) - 1.0)*100.0;
    apy
}

