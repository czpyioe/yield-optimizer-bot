pub struct Position{
    pub protocol:String,
    pub network:String,
    pub asset_address:String,
    pub amount:Option<f64>,
    pub apy:Option<f64>
}



pub struct Apy_snapshot{
    pub protocol:String,
    pub network:String,
    pub asset_address:String,
    pub apy:Option<f64>
}