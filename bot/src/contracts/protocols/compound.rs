use alloy::contract;
use alloy::providers::Provider;
use alloy::primitives::Address;
use anyhow::Result;

use crate::contracts::addresses::{CompoundContract,CompoundAsset,Network};

use crate::contracts::bindings::compound::cTokenv3;
use crate::contracts::addresses;

use crate::db::models::ApySnapshot;


pub fn get_compound_contract<P: Provider>(
    provider: P,
    network: Network,
    compound_contract:CompoundContract,
    compound_asset:CompoundAsset
) -> Result<cTokenv3::cTokenv3Instance<P>> {
    let address = network.get_compound_contract_address(compound_contract,compound_asset)?;
    Ok(cTokenv3::new(address, provider))
}

pub async fn get_apy_snapshot<P:Provider>(provider:P, network: Network,compound_asset:CompoundAsset)->Result<ApySnapshot>{
    let contract = get_compound_contract(provider, network,CompoundContract::cTokenv3,compound_asset)?;
    let utilization = contract.getUtilization().call().await?;

    let supply_rate = contract.getSupplyRate(utilization).call().await?;

    let apy = supply_rate_to_apy(supply_rate);
    
    Ok(ApySnapshot { 
        protocol: "compound".to_string(),
        network: network.name()?, 
        asset: compound_asset.name()?, 
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

