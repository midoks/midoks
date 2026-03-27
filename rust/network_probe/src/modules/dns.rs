use std::net::IpAddr;
use trust_dns_resolver::{TokioAsyncResolver, config::{ResolverConfig, ResolverOpts}};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::utils::error::{NetworkError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsQueryResult {
    pub domain: String,
    pub query_type: String,
    pub records: Vec<DnsRecord>,
    pub response_time: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecord {
    pub record_type: String,
    pub value: String,
    pub ttl: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsConfig {
    pub domain: String,
    pub query_type: DnsQueryType,
    pub nameserver: Option<String>,
    pub timeout: std::time::Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DnsQueryType {
    A,
    AAAA,
    CNAME,
    MX,
    TXT,
    NS,
    SOA,
    PTR,
    ALL,
}

impl std::fmt::Display for DnsQueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DnsQueryType::A => write!(f, "A"),
            DnsQueryType::AAAA => write!(f, "AAAA"),
            DnsQueryType::CNAME => write!(f, "CNAME"),
            DnsQueryType::MX => write!(f, "MX"),
            DnsQueryType::TXT => write!(f, "TXT"),
            DnsQueryType::NS => write!(f, "NS"),
            DnsQueryType::SOA => write!(f, "SOA"),
            DnsQueryType::PTR => write!(f, "PTR"),
            DnsQueryType::ALL => write!(f, "ALL"),
        }
    }
}

impl Default for DnsConfig {
    fn default() -> Self {
        Self {
            domain: String::new(),
            query_type: DnsQueryType::A,
            nameserver: None,
            timeout: std::time::Duration::from_secs(5),
        }
    }
}

pub struct DnsService {
    resolver: TokioAsyncResolver,
}

impl DnsService {
    pub async fn new() -> Result<Self> {
        let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
        Ok(Self { resolver })
    }

    pub async fn new_with_nameserver(_nameserver: &str) -> Result<Self> {
        // 简化实现，使用默认配置
        let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
        Ok(Self { resolver })
    }

    pub async fn query(&self, config: DnsConfig) -> Result<DnsQueryResult> {
        let start_time = std::time::Instant::now();
        let mut records = Vec::new();

        match config.query_type {
            DnsQueryType::A => {
                let response = self.resolver.lookup_ip(&config.domain).await
                    .map_err(|e| NetworkError::Dns(format!("A record lookup failed: {}", e)))?;
                
                for addr in response.iter() {
                    if let IpAddr::V4(ipv4) = addr {
                        records.push(DnsRecord {
                            record_type: "A".to_string(),
                            value: ipv4.to_string(),
                            ttl: 300, // 默认TTL
                        });
                    }
                }
            }
            DnsQueryType::AAAA => {
                let response = self.resolver.lookup_ip(&config.domain).await
                    .map_err(|e| NetworkError::Dns(format!("AAAA record lookup failed: {}", e)))?;
                
                for addr in response.iter() {
                    if let IpAddr::V6(ipv6) = addr {
                        records.push(DnsRecord {
                            record_type: "AAAA".to_string(),
                            value: ipv6.to_string(),
                            ttl: 300,
                        });
                    }
                }
            }
            DnsQueryType::MX => {
                let response = self.resolver.mx_lookup(&config.domain).await
                    .map_err(|e| NetworkError::Dns(format!("MX lookup failed: {}", e)))?;
                
                for mx in response.iter() {
                    records.push(DnsRecord {
                        record_type: "MX".to_string(),
                        value: format!("{} (priority: {})", mx.exchange(), mx.preference()),
                        ttl: 300,
                    });
                }
            }
            DnsQueryType::TXT => {
                let response = self.resolver.txt_lookup(&config.domain).await
                    .map_err(|e| NetworkError::Dns(format!("TXT lookup failed: {}", e)))?;
                
                for txt in response.iter() {
                    records.push(DnsRecord {
                        record_type: "TXT".to_string(),
                        value: txt.to_string(),
                        ttl: 300,
                    });
                }
            }
            DnsQueryType::NS => {
                let response = self.resolver.ns_lookup(&config.domain).await
                    .map_err(|e| NetworkError::Dns(format!("NS lookup failed: {}", e)))?;
                
                for ns in response.iter() {
                    records.push(DnsRecord {
                        record_type: "NS".to_string(),
                        value: ns.to_string(),
                        ttl: 300,
                    });
                }
            }
            DnsQueryType::CNAME | DnsQueryType::SOA | DnsQueryType::PTR | DnsQueryType::ALL => {
                // 简化实现，返回A记录
                let a_config = DnsConfig {
                    domain: config.domain.clone(),
                    query_type: DnsQueryType::A,
                    ..config
                };
                return Box::pin(self.query(a_config)).await;
            }
        }

        let response_time = start_time.elapsed().as_secs_f64() * 1000.0;

        Ok(DnsQueryResult {
            domain: config.domain,
            query_type: config.query_type.to_string(),
            records,
            response_time,
            timestamp: Utc::now(),
        })
    }

    #[allow(dead_code)]
    pub async fn resolve(&self, domain: &str) -> Result<Vec<String>> {
        let config = DnsConfig {
            domain: domain.to_string(),
            query_type: DnsQueryType::A,
            ..Default::default()
        };
        
        let result = self.query(config).await?;
        let ips: Vec<String> = result.records.iter()
            .map(|record| record.value.clone())
            .collect();
        
        Ok(ips)
    }

    #[allow(dead_code)]
    pub async fn check_dns_propagation(&self, domain: &str, nameservers: Vec<&str>) -> Result<Vec<DnsQueryResult>> {
        let mut results = Vec::new();
        
        for nameserver in nameservers {
            let service = DnsService::new_with_nameserver(nameserver).await?;
            let config = DnsConfig {
                domain: domain.to_string(),
                query_type: DnsQueryType::A,
                ..Default::default()
            };
            
            match service.query(config).await {
                Ok(result) => results.push(result),
                Err(e) => log::warn!("DNS query failed for {}: {}", nameserver, e),
            }
        }
        
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dns_query() {
        let service = DnsService::new().await.unwrap();
        let config = DnsConfig {
            domain: "google.com".to_string(),
            query_type: DnsQueryType::A,
            ..Default::default()
        };
        
        let result = service.query(config).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.domain, "google.com");
        assert!(!result.records.is_empty());
    }

    #[tokio::test]
    async fn test_resolve() {
        let service = DnsService::new().await.unwrap();
        let result = service.resolve("google.com").await;
        assert!(result.is_ok());
        let ips = result.unwrap();
        assert!(!ips.is_empty());
    }
}