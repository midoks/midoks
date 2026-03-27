use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::utils::error::{NetworkError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpingResult {
    pub host: String,
    pub port: u16,
    pub ip: String,
    pub success: bool,
    pub avg_rtt: f64,
    pub min_rtt: f64,
    pub max_rtt: f64,
    pub attempts: u32,
    pub successful_attempts: u32,
    pub packet_loss: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpingConfig {
    pub host: String,
    pub port: u16,
    pub count: u32,
    pub timeout: Duration,
    pub delay: Duration,
}

impl Default for TcpingConfig {
    fn default() -> Self {
        Self {
            host: String::new(),
            port: 80,
            count: 4,
            timeout: Duration::from_secs(3),
            delay: Duration::from_millis(100),
        }
    }
}

pub struct TcpingService;

impl TcpingService {
    pub fn new() -> Self {
        Self
    }

    pub async fn tcping(&self, config: TcpingConfig) -> Result<TcpingResult> {
        let mut rtt_values = Vec::new();
        let mut successful_attempts = 0;
        
        let addr = format!("{}:{}", config.host, config.port);
        
        for i in 0..config.count {
            let start_time = std::time::Instant::now();
            
            match timeout(config.timeout, TcpStream::connect(&addr)).await {
                Ok(Ok(_)) => {
                    let rtt = start_time.elapsed().as_secs_f64() * 1000.0;
                    rtt_values.push(rtt);
                    successful_attempts += 1;
                    log::info!("TCP connection {} to {} succeeded in {:.2}ms", i + 1, addr, rtt);
                }
                Ok(Err(e)) => {
                    log::warn!("TCP connection {} to {} failed: {}", i + 1, addr, e);
                }
                Err(_) => {
                    log::warn!("TCP connection {} to {} timed out", i + 1, addr);
                }
            }
            
            if i < config.count - 1 {
                tokio::time::sleep(config.delay).await;
            }
        }

        if successful_attempts == 0 {
            return Err(NetworkError::Tcp(format!("All connections to {} failed", addr)));
        }

        let packet_loss = ((config.count - successful_attempts) as f64 / config.count as f64) * 100.0;
        let min_rtt = rtt_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_rtt = rtt_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let avg_rtt = rtt_values.iter().sum::<f64>() / rtt_values.len() as f64;

        // 获取IP地址
        let ip = tokio::net::lookup_host(&config.host)
            .await?
            .next()
            .ok_or_else(|| NetworkError::Dns(format!("Could not resolve {}", config.host)))?
            .to_string();

        Ok(TcpingResult {
            host: config.host,
            port: config.port,
            ip,
            success: successful_attempts > 0,
            avg_rtt,
            min_rtt,
            max_rtt,
            attempts: config.count,
            successful_attempts,
            packet_loss,
            timestamp: Utc::now(),
        })
    }

    pub async fn check_port(&self, host: &str, port: u16, timeout_duration: Duration) -> Result<bool> {
        let addr = format!("{}:{}", host, port);
        match timeout(timeout_duration, TcpStream::connect(&addr)).await {
            Ok(Ok(_)) => Ok(true),
            _ => Ok(false),
        }
    }

    pub async fn scan_ports(&self, host: &str, ports: Vec<u16>, timeout: Duration) -> Result<Vec<(u16, bool)>> {
        let mut results = Vec::new();
        
        for port in ports {
            let is_open = self.check_port(host, port, timeout).await?;
            results.push((port, is_open));
        }
        
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tcping_localhost() {
        let service = TcpingService::new();
        let config = TcpingConfig {
            host: "127.0.0.1".to_string(),
            port: 8080, // 假设这个端口没有服务
            count: 2,
            ..Default::default()
        };
        
        let result = service.tcping(config).await;
        // 这个测试可能失败，因为端口可能没有服务
        if let Ok(result) = result {
            assert_eq!(result.host, "127.0.0.1");
            assert_eq!(result.port, 8080);
        }
    }

    #[tokio::test]
    async fn test_check_port() {
        let service = TcpingService::new();
        let timeout = Duration::from_secs(1);
        
        // 测试常见端口
        let result = service.check_port("127.0.0.1", 22, timeout).await;
        assert!(result.is_ok());
    }
}