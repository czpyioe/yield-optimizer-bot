pub struct Position{
    pub protocol:String,
    pub network:String,
    pub asset_address:String,
    pub amount:Option<f64>,
    pub apy:Option<f64>
}


#[derive(Clone)]
pub struct ApySnapshot{
    pub protocol:String,
    pub network:String,
    pub asset:String,
    pub apy:Option<f64>
}