#[derive(Clone,Debug)]
pub struct ApySnapshot{
    pub protocol:String,
    pub network:String,
    pub asset:String,
    pub apy:Option<f64>
}