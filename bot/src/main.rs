mod rpc;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // let tmp = rpc::check_rpcs_health().await;
    // for (a,b,c) in tmp{
    //     println!("{a}, {b}, {c}");
    // }
    let urls = rpc::request_lama_rpcs().await?;
    // for url in urls{
    //     println!("{}",url);
    // }
    println!("{}",urls.len());
    Ok(())
}

