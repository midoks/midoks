use pnet::packet::icmp::{IcmpPacket, IcmpTypes};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::{Ipv4Packet, MutableIpv4Packet};
use pnet::packet::Packet;
use pnet::transport::{transport_channel, TransportChannelType, TransportReceiver, TransportSender};
use std::net::{IpAddr, ToSocketAddrs};
use std::time::{Duration, Instant};
use tokio::time::timeout;
use anyhow::{anyhow, Result};
use colored::*;
use crate::geo::GeoLocator;
use crate::utils;

pub struct IpTracer {
    max_hops: u8,
    timeout_duration: Duration,
    show_geo: bool,
    geo_locator: Option<GeoLocator>,
}

#[derive(Debug, Clone)]
pub struct HopResult {
    pub hop: u8,
    pub ip: Option<IpAddr>,
    pub rtt: Option<Duration>,
    pub hostname: Option<String>,
    pub geo_info: Option<String>,
}

impl IpTracer {
    pub fn new(max_hops: u8, timeout_secs: u64, show_geo: bool) -> Result<Self> {
        let geo_locator = if show_geo {
            Some(GeoLocator::new())
        } else {
            None
        };

        Ok(Self {
            max_hops,
            timeout_duration: Duration::from_secs(timeout_secs),
            show_geo,
            geo_locator,
        })
    }

    pub async fn trace(&mut self, target: &str) -> Result<()> {
        let target_ip = self.resolve_hostname(target).await?;
        
        println!("{}", format!("Target IP: {}", target_ip).green());
        println!();

        for ttl in 1..=self.max_hops {
            let result = self.trace_hop(target_ip, ttl).await;
            self.print_hop_result(&result).await;
            
            if let Some(ip) = result.ip {
                if ip == target_ip {
                    println!();
                    println!("{}", "Trace complete!".green().bold());
                    break;
                }
            }
        }

        Ok(())
    }

    async fn resolve_hostname(&self, hostname: &str) -> Result<IpAddr> {
        if let Ok(ip) = hostname.parse::<IpAddr>() {
            return Ok(ip);
        }

        let addrs: Vec<_> = format!("{}:80", hostname)
            .to_socket_addrs()?
            .collect();
        
        addrs.first()
            .map(|addr| addr.ip())
            .ok_or_else(|| anyhow!("Failed to resolve hostname: {}", hostname))
    }

    async fn trace_hop(&self, target_ip: IpAddr, ttl: u8) -> HopResult {
        match self.send_ping(target_ip, ttl).await {
            Ok((ip, rtt)) => {
                let hostname = utils::reverse_dns_lookup(ip).await;
                let geo_info = if let Some(ref locator) = self.geo_locator {
                    locator.get_location(ip).await.ok()
                } else {
                    None
                };

                HopResult {
                    hop: ttl,
                    ip: Some(ip),
                    rtt: Some(rtt),
                    hostname,
                    geo_info,
                }
            }
            Err(_) => HopResult {
                hop: ttl,
                ip: None,
                rtt: None,
                hostname: None,
                geo_info: None,
            },
        }
    }

    async fn send_ping(&self, target_ip: IpAddr, ttl: u8) -> Result<(IpAddr, Duration)> {
        // 使用系统ping命令作为简化实现
        let start_time = Instant::now();
        
        let output = tokio::process::Command::new("ping")
            .arg("-c")
            .arg("1")
            .arg("-t")
            .arg(ttl.to_string())
            .arg("-W")
            .arg((self.timeout_duration.as_millis() as u32).to_string())
            .arg(target_ip.to_string())
            .output()
            .await?;

        let rtt = start_time.elapsed();
        
        if output.status.success() {
            Ok((target_ip, rtt))
        } else {
            // 尝试解析ping输出中的中间路由器IP
            let output_str = String::from_utf8_lossy(&output.stderr);
            if let Some(ip_str) = self.extract_intermediate_ip(&output_str) {
                if let Ok(ip) = ip_str.parse::<IpAddr>() {
                    return Ok((ip, rtt));
                }
            }
            Err(anyhow!("Ping failed or timeout"))
        }
    }

    fn extract_intermediate_ip(&self, output: &str) -> Option<String> {
        // 从ping错误输出中提取中间路由器IP
        for line in output.lines() {
            if line.contains("Time to live exceeded") || line.contains("TTL expired") {
                // 尝试提取IP地址
                let words: Vec<&str> = line.split_whitespace().collect();
                for word in words {
                    if word.parse::<IpAddr>().is_ok() {
                        return Some(word.to_string());
                    }
                }
            }
        }
        None
    }

    async fn print_hop_result(&self, result: &HopResult) {
        print!("{:2} ", result.hop);
        
        match result.ip {
            Some(ip) => {
                let ip_str = format!("{}", ip).yellow();
                print!("{}", ip_str);
                
                if let Some(ref hostname) = result.hostname {
                    print!(" ({})", hostname.blue());
                }
                
                if let Some(rtt) = result.rtt {
                    print!("  {:.2}ms", rtt.as_secs_f64() * 1000.0);
                }
                
                if let Some(ref geo) = result.geo_info {
                    print!("  [{}]", geo.green());
                }
            }
            None => {
                print!("{}", "* * *".red());
            }
        }
        
        println!();
    }
}