use alloy::primitives::Address;
use anyhow::Result;

pub struct Addresses{
    // ethereum mainnet
    pub eth_aave_protocol_data_provider:Address,


    // arbitrum mainnet
    pub arb_aave_protocol_data_provider:Address,
}


impl Addresses {
    pub fn get_mainnet_addresses()->Result<Self>{
        Ok(Self { 
            eth_aave_protocol_data_provider:"0x0a16f2FCC0D44FaE41cc54e079281D84A363bECD".parse()?,
            arb_aave_protocol_data_provider:"0x243Aa95cAC2a25651eda86e80bEe66114413c43b".parse()?,
        })
    }
}