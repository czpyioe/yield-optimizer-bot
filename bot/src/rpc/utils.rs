use crate::rpc::manager::RpcEndpoint;

pub fn create_endpoints(urls: Vec<String>) -> Vec<RpcEndpoint> {
    urls.into_iter()
        .map(|url| RpcEndpoint {
            url,
            latency: None,
        })
        .collect()
}

pub fn is_valid_http_rpc_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}