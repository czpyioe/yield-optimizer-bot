use std::time::{Duration, Instant};
use anyhow::Result;
use std::collections::HashMap;
use alloy::providers::{Provider, ProviderBuilder}; 
use std::sync::Arc;
use futures::stream::{self, StreamExt};

use crate::rpc::loader;
use crate::rpc::health;
use crate::rpc::utils;
use crate::contracts::addresses::Network;

#[derive(Clone)]
pub struct RpcEndpoint {
    pub url: String,
    pub latency: Option<Duration>,
}

#[derive(Clone)]
pub struct ProviderWithScore {
    pub provider: Arc<dyn Provider>,
    pub endpoint: RpcEndpoint,
    pub score: usize,
}

pub struct NetworkProviderPool {
    pools: HashMap<Network, Vec<ProviderWithScore>>,
    last_health_check: Instant,
    health_check_interval: Duration,
}

impl NetworkProviderPool {
    pub fn new(health_check_interval: Duration) -> Self {
        Self {
            pools: HashMap::new(),
            last_health_check: Instant::now(),
            health_check_interval,
        }
    }

    pub async fn initialize(&mut self) -> Result<()> {
        let rpc_urls_by_network = loader::load_rpcs_url().await?;

        let futures = rpc_urls_by_network
            .into_iter()
            .map(|(network, urls)| self.process_network(network, urls));

        let results: Vec<Result<(Network, Vec<ProviderWithScore>)>> = stream::iter(futures)
            .buffer_unordered(3)
            .collect()
            .await;

        for result in results {
            let (network, providers) = result?;
            if !providers.is_empty() {
                self.pools.insert(network, providers);
            }
        }

        self.last_health_check = Instant::now();
        Ok(())
    }
    
    async fn process_network(&self, network: Network, urls: Vec<String>) -> Result<(Network, Vec<ProviderWithScore>)> {
        let endpoints: Vec<RpcEndpoint> = utils::create_endpoints(urls);

        let healthy_endpoints: Vec<RpcEndpoint> = stream::iter(endpoints)
            .map(health::check_endpoint_health)
            .buffer_unordered(50)
            .filter_map(|res| async move {
                match res {
                    Ok(endpoint) if endpoint.latency.is_some() => Some(endpoint),
                    _ => None,
                }
            })
            .collect()
            .await;

        let mut providers: Vec<ProviderWithScore> = stream::iter(healthy_endpoints)
            .map(|endpoint| async move {
                let provider = ProviderBuilder::new()
                    .connect(endpoint.url.as_str())
                    .await
                    .ok()?;

                let score = endpoint.latency
                    .map(|l| l.as_millis() as usize)
                    .unwrap_or(10_000);

                Some(ProviderWithScore {
                    provider: Arc::new(provider),
                    endpoint,
                    score,
                })
            })
            .buffer_unordered(50)
            .filter_map(|x| async move { x })
            .collect()
            .await;

        providers.sort_by_key(|p| p.score);

        Ok((network, providers))
    }



    pub fn get_all_providers(&self, network: &Network) -> Option<&Vec<ProviderWithScore>> {
        self.pools.get(network)
    }

    
    pub async fn refresh_health(&mut self) -> Result<()> {
        // if self.last_health_check.elapsed() < self.health_check_interval {
        //     return Ok(());
        // }

        let inputs: Vec<(Network, Vec<String>)> = self.pools
            .iter()
            .map(|(&network, providers)| {
                let urls = providers
                    .iter()
                    .map(|p| p.endpoint.url.clone())
                    .collect::<Vec<_>>();

                (network, urls)
            })
            .collect();

        let futures = inputs
            .into_iter()
            .map(|(network, urls)| self.process_network(network, urls));

        let results:  Vec<Result<(Network, Vec<ProviderWithScore>)>> = stream::iter(futures)
            .buffer_unordered(3)
            .collect()
            .await;

            for result in results {
            let (network, providers) = result?;
            self.pools.insert(network, providers);
        }

        self.last_health_check = Instant::now();
        Ok(())
    }

    
    pub fn stats(&self, network: &Network) -> Option<NetworkStats> {
        let providers = self.pools.get(network)?;
        
        let total = providers.len();
        let avg_latency = if total > 0 {
            let sum: u128 = providers
                .iter()
                .filter_map(|p| p.endpoint.latency)
                .map(|d| d.as_millis())
                .sum();
            Some((sum / total as u128) as usize)
        } else {
            None
        };

        Some(NetworkStats {
            network: *network,
            total_providers: total,
            avg_latency_ms: avg_latency,
        })
    }
}

pub struct NetworkStats {
    pub network: Network,
    pub total_providers: usize,
    pub avg_latency_ms: Option<usize>,
}