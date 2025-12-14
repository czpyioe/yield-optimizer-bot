mod rpc;

#[tokio::main]
async fn main() {
    let tmp = rpc::check_rpc_health().await;
    for (a,b,c) in tmp{
        println!("{a}, {b}, {c}");
    }
}