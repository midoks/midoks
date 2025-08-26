use pnet::packet::icmp::{IcmpPacket, IcmpTypes};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::{Ipv4Packet, MutableIpv4Packet};
use pnet::packet::{MutablePacket, Packet};
use pnet::transport::{transport_channel, TransportChannelType, TransportProtocol, TransportReceiver, TransportSender};
use std::net::{IpAddr, Ipv4Addr, ToSocketAddrs};
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
        match self.send_icmp_packet(target_ip, ttl).await {
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

    async fn send_icmp_packet(&self, target_ip: IpAddr, ttl: u8) -> Result<(IpAddr, Duration)> {
        let protocol = TransportChannelType::Layer3(IpNextHeaderProtocols::Icmp);
        let (mut tx, mut rx) = transport_channel(4096, protocol)?;

        let start_time = Instant::now();
        
        // 发送ICMP包
        self.send_icmp_echo(&mut tx, target_ip, ttl)?;
        
        // 等待响应
        let result = timeout(self.timeout_duration, self.receive_icmp_reply(&mut rx)).await;
        
        match result {
            Ok(Ok(ip)) => {
                let rtt = start_time.elapsed();
                Ok((ip, rtt))
            }
            _ => Err(anyhow!("Timeout or error receiving ICMP reply")),
        }
    }

    fn send_icmp_echo(&self, tx: &mut TransportSender, target_ip: IpAddr, ttl: u8) -> Result<()> {
        if let IpAddr::V4(target_ipv4) = target_ip {
            let mut buffer = vec![0u8; 28]; // IP header (20) + ICMP header (8)
            let mut ip_packet = MutableIpv4Packet::new(&mut buffer).unwrap();
            
            ip_packet.set_version(4);
            ip_packet.set_header_length(5);
            ip_packet.set_total_length(28);
            ip_packet.set_identification(rand::random());
            ip_packet.set_ttl(ttl);
            ip_packet.set_next_level_protocol(IpNextHeaderProtocols::Icmp);
            ip_packet.set_destination(target_ipv4);
            
            // 设置ICMP Echo Request
            let icmp_payload = &mut buffer[20..];
            icmp_payload[0] = 8; // ICMP Type: Echo Request
            icmp_payload[1] = 0; // ICMP Code
            icmp_payload[4] = rand::random::<u8>(); // Identifier
            icmp_payload[5] = rand::random::<u8>();
            icmp_payload[6] = 0; // Sequence Number
            icmp_payload[7] = 1;
            
            tx.send_to(ip_packet, IpAddr::V4(target_ipv4))?;
        }
        
        Ok(())
    }

    async fn receive_icmp_reply(&self, rx: &mut TransportReceiver) -> Result<IpAddr> {
        loop {
            let (packet, addr) = rx.next()?;
            
            if let Some(ipv4_packet) = Ipv4Packet::new(packet) {
                if ipv4_packet.get_next_level_protocol() == IpNextHeaderProtocols::Icmp {
                    if let Some(icmp_packet) = IcmpPacket::new(ipv4_packet.payload()) {
                        // 检查是否是Time Exceeded或Echo Reply
                        if icmp_packet.get_icmp_type() == IcmpTypes::TimeExceeded ||
                           icmp_packet.get_icmp_type() == IcmpTypes::EchoReply {
                            return Ok(IpAddr::V4(ipv4_packet.get_source()));
                        }
                    }
                }
            }
        }
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