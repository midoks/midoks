use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::time::Duration;
use surge_ping::{Client, Config, PingIdentifier, PingSequence};
use tokio::time::timeout;

use crate::utils::error::{NetworkError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingResult {
    pub host: String,
    pub ip: String,
    pub packet_loss: f64,
    pub min_rtt: f64,
    pub max_rtt: f64,
    pub avg_rtt: f64,
    pub packets_sent: u32,
    pub packets_received: u32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingConfig {
    pub host: String,
    pub count: u32,
    pub timeout: Duration,
    pub packet_size: usize,
}

impl Default for PingConfig {
    fn default() -> Self {
        Self {
            host: String::new(),
            count: 4,
            timeout: Duration::from_secs(2),
            packet_size: 56,
        }
    }
}

pub struct PingService;

impl PingService {
    pub fn new() -> Self {
        Self
    }

    pub async fn ping(&self, config: PingConfig) -> Result<PingResult> {
        let client = match Client::new(&Config::default()) {
            Ok(client) => client,
            Err(e) => {
                if e.to_string().contains("Operation not permitted") {
                    return Err(NetworkError::Permission("Ping requires root privileges. Please run with sudo or as root.".to_string()));
                }
                return Err(NetworkError::Ping(e.to_string()));
            }
        };
        let host = config.host.clone();

        // 解析主机地址
        let host_addr: IpAddr = tokio::net::lookup_host(&host)
            .await?
            .next()
            .ok_or_else(|| NetworkError::Dns(format!("Could not resolve {}", host)))?
            .ip();

        let mut packets_sent = 0;
        let mut packets_received = 0;
        let mut rtt_values = Vec::new();

        for i in 0..config.count {
            packets_sent += 1;

            let start_time = std::time::Instant::now();
            let sequence = PingSequence(i as u16);
            let identifier = PingIdentifier(rand::random());

            let mut pinger = client.pinger(host_addr, identifier).await;
            match timeout(config.timeout, pinger.ping(sequence, &[0; 56])).await {
                Ok(Ok((_packet, _))) => {
                    let rtt = start_time.elapsed().as_secs_f64() * 1000.0;
                    packets_received += 1;
                    rtt_values.push(rtt);
                }
                Ok(Err(e)) => {
                    log::warn!("ping {} failed: {}", host, e);
                }
                Err(_) => {
                    log::warn!("ping {} timeout", host);
                }
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        if packets_received == 0 {
            return Err(NetworkError::Ping(format!("All packets lost for {}", host)));
        }

        let packet_loss = ((packets_sent - packets_received) as f64 / packets_sent as f64) * 100.0;
        let min_rtt = rtt_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_rtt = rtt_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let avg_rtt = rtt_values.iter().sum::<f64>() / rtt_values.len() as f64;

        // 获取IP地址
        let ip = tokio::net::lookup_host(&host)
            .await?
            .next()
            .ok_or_else(|| NetworkError::Dns(format!("Could not resolve {}", host)))?
            .to_string();

        Ok(PingResult {
            host,
            ip,
            packet_loss,
            min_rtt,
            max_rtt,
            avg_rtt,
            packets_sent,
            packets_received,
            timestamp: Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ping_localhost() {
        let service = PingService::new();
        let config = PingConfig {
            host: "127.0.0.1".to_string(),
            count: 2,
            ..Default::default()
        };

        let result = service.ping(config).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.host, "127.0.0.1");
        assert!(result.packets_received > 0);
    }
}
