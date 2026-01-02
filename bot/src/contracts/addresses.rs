use alloy::primitives::Address;
use anyhow::Result;



#[derive(Debug, Clone, Copy)]
pub enum Asset {
    USDC,
    WETH,
}

impl Asset {
    pub fn all()->Vec<Asset> {
        vec![
            Asset::USDC,
            Asset::WETH
        ]
    }

    pub fn name(&self) -> &str{
        match self {
            Asset::USDC=>"USDC",
            Asset::WETH=>"WETH"
        }
    }

    pub fn is_on_aave(&self) -> bool {
        matches!(self,Asset::USDC | Asset::WETH)
    }

    pub fn is_on_compound(&self) -> bool {
        matches!(self,Asset::USDC | Asset::WETH)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Protocol{
    Aave,
    Compound
}

impl Protocol{
    pub fn all() -> Vec<Protocol> {
        vec![
            Protocol::Aave,
            Protocol::Compound
        ]
    }

    pub fn name(&self)-> &str{
        match self {
            Protocol::Aave=>"Aave",
            Protocol::Compound=>"Compound"
        }
    }

    pub fn supported_assets(&self,network: &Network) -> Vec<Asset> {
        match network {
            Network::Ethereum=>{
                match self{
                    Protocol::Aave => vec![Asset::USDC, Asset::WETH],
                    Protocol::Compound => vec![Asset::USDC, Asset::WETH],
                }
            },
            Network::Arbitrum=>{
                match self{
                    Protocol::Aave => vec![Asset::USDC, Asset::WETH],
                    Protocol::Compound => vec![Asset::USDC, Asset::WETH],
                }
            }

        }
    }
    
    pub fn supports_asset(&self, asset: Asset) -> bool {
        match self {
            Protocol::Aave => asset.is_on_aave(),
            Protocol::Compound => asset.is_on_compound(),
        }
    }

    pub fn supported_networks(&self) -> Vec<Network> {
        match self {
            Protocol::Aave => vec![Network::Ethereum, Network::Arbitrum],
            Protocol::Compound => vec![Network::Ethereum, Network::Arbitrum],
        }
    }
    
    pub fn supports_network(&self, network: Network) -> bool {
        match self {
            Protocol::Aave => network.is_on_aave(),
            Protocol::Compound => network.is_on_compound(),
        }
    }

}


#[derive(Debug, Clone, Copy,Eq, Hash, PartialEq)]
pub enum Network{
    Ethereum,
    Arbitrum,
}

impl Network{
    pub fn name(&self)->&str{
        match self{
            Network::Ethereum=>"Ethereum",
            Network::Arbitrum=>"Arbitrum",
        }
    }

    pub fn all() -> Vec<Network> {
        vec![
            Network::Ethereum,
            Network::Arbitrum,
        ]
    }

    pub fn get_chain_id(&self)->usize{
        match self{
            Network::Ethereum=>1,
            Network::Arbitrum=>42161
        }
    }

    pub fn get_network_from_chain_id(chain_id:usize)->Result<Network>{
        let network = match chain_id{
            1=>Network::Ethereum,
            42161=>Network::Arbitrum,
            _=>anyhow::bail!("chain id not found")
        };
        Ok(network)
    }

    pub fn is_on_aave(&self) -> bool {
        matches!(self,Network::Ethereum | Network::Arbitrum)
    }

    pub fn is_on_compound(&self) -> bool {
        matches!(self,Network::Ethereum | Network::Arbitrum)
    }

    // Aave
    pub fn get_aave_contract_address(&self,contract: AaveContract) -> Result<Address> {
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
    pub fn get_asset_address_aave(&self, asset:Asset)->Result<Address>{
        let addr = match (self,asset) {
            // Ethereum mainet
            (Network::Ethereum,Asset::USDC)=>"0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",


            // Arbitrum mainnet
            (Network::Arbitrum,Asset::USDC)=>"0xFF970A61A04b1cA14834A43f5dE4533eBDDB5CC8",

            _=>anyhow::bail!("Asset address not configured yet")
        };
        Ok(addr.parse()?)
    }



    //Compound
    pub fn get_compound_contract_address(&self,contract: CompoundContract, asset:Asset) -> Result<Address> {
        let addr = match (self, contract, asset) {
            // Ethereum mainnet
            (Network::Ethereum, CompoundContract::CTokenv3, Asset::USDC) => "0xc3d688B66703497DAA19211EEdff47f25384cdc3",
            (Network::Ethereum, CompoundContract::CTokenv3, Asset::WETH) => "0xA17581A9E3356d9A858b789D68B4d866e593aE94",

            // Arbitrum mainnet
            (Network::Arbitrum, CompoundContract::CTokenv3, Asset::USDC) => "0x9c4ec768c28520B50860ea7a15bd7213a9fF58bf",
            (Network::Arbitrum, CompoundContract::CTokenv3, Asset::WETH) => "0x6f7D514bbD4aFf3BcD1140B7344b32f063dEe486",
          
            _ => anyhow::bail!("Contract address not configured yet"),
        };
        Ok(addr.parse()?)
    }


}



#[derive(Debug, Clone, Copy)]
pub enum AaveContract{
    AaveProtocolDataProvider,
    Pool
}


#[derive(Debug, Clone, Copy)]
pub enum CompoundContract{
    CTokenv3,
}