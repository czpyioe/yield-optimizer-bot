mod rpc;

#[tokio::main]
async fn main() {
    let chain_id = match rpc::connect_provider().await {
        Ok(v) => v,
        Err(_) => return
    };
    println!("{:?}", chain_id);
}