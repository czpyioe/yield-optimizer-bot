use alloy::primitives::Address;
use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub enum AaveContract{
    AaveProtocolDataProvider,
    Pool
}

#[derive(Debug, Clone, Copy)]
pub enum Network{
    Ethereum,
    Arbitrum,
    Base,
}

impl Network{
    pub fn get_aave_contract_address(self:&Network,contract: AaveContract) -> Result<Address> {
        let addr = match (self, contract) {
            // Ethereum mainnet
            (Network::Ethereum, AaveContract::AaveProtocolDataProvider) => "0x0a16f2FCC0D44FaE41cc54e079281D84A363bECD",
            (Network::Ethereum, AaveContract::Pool) => "0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2",
            
            // Arbitrum mainnet
            (Network::Arbitrum, AaveContract::AaveProtocolDataProvider) => "0x243Aa95cAC2a25651eda86e80bEe66114413c43b",
            (Network::Arbitrum, AaveContract::Pool) => "0x794a61358D6845594F94dc1DB02A252b5b4814aD",
            
            _ => anyhow::bail!("Contract address not configured yet"),
        };

        Ok(addr.parse()?)
    }

    pub fn name(self:&Network)->Result<String>{
        let name = match self{
            Network::Ethereum=>"Ethereum",
            Network::Arbitrum=>"Arbitrum",
            Network::Base=>"Base",
        };
        Ok(name.to_string())
    }
}

