use clap::{Parser, Subcommand};
use std::time::Duration;

use crate::modules::{
    ping::{PingConfig, PingService},
    tcping::{TcpingConfig, TcpingService},
    website::{WebsiteTestConfig, WebsiteTestService},
    traceroute::{TracerouteConfig, TracerouteService},
    dns::{DnsConfig, DnsService, DnsQueryType},
};

#[derive(Parser)]
#[command(name = "network-probe")]
#[command(about = "A comprehensive network testing tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, default_value = "info")]
    pub log_level: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Perform ICMP ping test
    Ping {
        /// Target host
        host: String,
        
        /// Number of ping packets to send
        #[arg(short, long, default_value = "4")]
        count: u32,
        
        /// Timeout in seconds
        #[arg(short, long, default_value = "2")]
        timeout: u64,
    },

    /// Perform TCP connection test
    Tcping {
        /// Target host
        host: String,
        
        /// Target port
        #[arg(short, long, default_value = "80")]
        port: u16,
        
        /// Number of connection attempts
        #[arg(short, long, default_value = "4")]
        count: u32,
        
        /// Timeout in seconds
        #[arg(short, long, default_value = "3")]
        timeout: u64,
    },

    /// Test website availability
    Website {
        /// Target URL
        url: String,
        
        /// HTTP method
        #[arg(short, long, default_value = "GET")]
        method: String,
        
        /// Follow redirects
        #[arg(short, long)]
        follow_redirects: bool,
        
        /// Timeout in seconds
        #[arg(short, long, default_value = "30")]
        timeout: u64,
    },

    /// Perform traceroute
    Traceroute {
        /// Target host
        host: String,
        
        /// Maximum number of hops
        #[arg(short, long, default_value = "30")]
        max_hops: u32,
        
        /// Protocol (icmp, udp, tcp)
        #[arg(short, long, default_value = "icmp")]
        protocol: String,
    },

    /// Perform DNS query
    Dns {
        /// Domain name
        domain: String,
        
        /// Query type (A, AAAA, CNAME, MX, TXT, NS, SOA, PTR, ALL)
        #[arg(short, long, default_value = "A")]
        query_type: String,
        
        /// Custom nameserver
        #[arg(short, long)]
        nameserver: Option<String>,
    },

    /// Start API server
    Server {
        /// Server host
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        
        /// Server port
        #[arg(long, default_value = "8080")]
        port: u16,
    },

    /// Scan ports on target host
    PortScan {
        /// Target host
        host: String,
        
        /// Port range (e.g., 1-1000)
        #[arg(short, long, default_value = "1-1000")]
        range: String,
        
        /// Timeout in milliseconds
        #[arg(short, long, default_value = "1000")]
        timeout: u64,
    },

    /// Install as a systemd service
    Install {
        /// Server host
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        
        /// Server port
        #[arg(long, default_value = "8080")]
        port: u16,
        
        /// Service name
        #[arg(long, default_value = "network-probe")]
        service_name: String,
    },

    /// Uninstall systemd service
    Uninstall {
        /// Service name
        #[arg(long, default_value = "network-probe")]
        service_name: String,
    },
}

pub async fn handle_command(cli: Cli) -> anyhow::Result<()> {
    // 日志已经在主函数中初始化，这里不需要重复初始化

    match cli.command {
        Commands::Ping { host, count, timeout } => {
            handle_ping(host, count, Duration::from_secs(timeout)).await
        }
        Commands::Tcping { host, port, count, timeout } => {
            handle_tcping(host, port, count, Duration::from_secs(timeout)).await
        }
        Commands::Website { url, method, follow_redirects, timeout } => {
            handle_website(url, method, follow_redirects, Duration::from_secs(timeout)).await
        }
        Commands::Traceroute { host, max_hops, protocol } => {
            handle_traceroute(host, max_hops, protocol).await
        }
        Commands::Dns { domain, query_type, nameserver } => {
            handle_dns(domain, query_type, nameserver).await
        }
        Commands::Server { host, port } => {
            handle_server(host, port).await
        }
        Commands::PortScan { host, range, timeout } => {
            handle_port_scan(host, range, Duration::from_millis(timeout)).await
        }
        Commands::Install { host, port, service_name } => {
            handle_install(host, port, service_name).await
        }
        Commands::Uninstall { service_name } => {
            handle_uninstall(service_name).await
        }
    }
}

async fn handle_ping(host: String, count: u32, timeout: Duration) -> anyhow::Result<()> {
    println!("Pinging {}...", host);
    
    let service = PingService::new();
    let config = PingConfig {
        host: host.clone(),
        count,
        timeout,
        ..Default::default()
    };
    
    match service.ping(config).await {
        Ok(result) => {
            println!("Ping results for {} ({}):", result.host, result.ip);
            println!("  Packets: sent = {}, received = {}, loss = {:.1}%", 
                     result.packets_sent, result.packets_received, result.packet_loss);
            println!("  RTT (ms): min = {:.2}, max = {:.2}, avg = {:.2}",
                     result.min_rtt, result.max_rtt, result.avg_rtt);
        }
        Err(e) => {
            eprintln!("Ping failed: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}

async fn handle_tcping(host: String, port: u16, count: u32, timeout: Duration) -> anyhow::Result<()> {
    println!("TCPing {}:{}...", host, port);
    
    let service = TcpingService::new();
    let config = TcpingConfig {
        host: host.clone(),
        port,
        count,
        timeout,
        ..Default::default()
    };
    
    match service.tcping(config).await {
        Ok(result) => {
            println!("TCPing results for {}:{} ({}):", result.host, result.port, result.ip);
            println!("  Attempts: {}, successful = {}, loss = {:.1}%", 
                     result.attempts, result.successful_attempts, result.packet_loss);
            println!("  RTT (ms): min = {:.2}, max = {:.2}, avg = {:.2}",
                     result.min_rtt, result.max_rtt, result.avg_rtt);
            println!("  Port status: {}", if result.success { "OPEN" } else { "CLOSED" });
        }
        Err(e) => {
            eprintln!("TCPing failed: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}

async fn handle_website(url: String, method: String, follow_redirects: bool, timeout: Duration) -> anyhow::Result<()> {
    println!("Testing website {}...", url);
    
    let service = WebsiteTestService::new();
    let config = WebsiteTestConfig {
        url: url.clone(),
        method,
        timeout,
        follow_redirects,
        ..Default::default()
    };
    
    match service.test_website(config).await {
        Ok(result) => {
            println!("Website test results for {}:", result.url);
            println!("  Status: {}", if result.success { "SUCCESS" } else { "FAILED" });
            if let Some(status_code) = result.status_code {
                println!("  Status Code: {}", status_code);
            }
            println!("  Response Time: {:.2}ms", result.response_time);
            if let Some(content_length) = result.content_length {
                println!("  Content Length: {} bytes", content_length);
            }
            if let Some(error) = result.error_message {
                println!("  Error: {}", error);
            }
        }
        Err(e) => {
            eprintln!("Website test failed: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}

async fn handle_traceroute(host: String, max_hops: u32, protocol: String) -> anyhow::Result<()> {
    println!("Tracerouting to {} using {}...", host, protocol);
    
    let service = TracerouteService::new();
    let config = TracerouteConfig {
        host: host.clone(),
        max_hops,
        ..Default::default()
    };
    
    match service.traceroute(config).await {
        Ok(result) => {
            println!("Traceroute results for {} ({}):", result.host, result.ip);
            println!("  Total time: {:.2}s", result.total_time);
            println!("  Hops:");
            for hop in &result.hops {
                if hop.success {
                    let hostname = hop.hostname.as_deref().unwrap_or("*");
                    let ip = hop.ip.as_deref().unwrap_or("*");
                    let rtt = hop.rtt.map(|r| format!("{:.2}ms", r)).unwrap_or_else(|| "*".to_string());
                    println!("    {:2}: {} ({}) {}", hop.hop_number, hostname, ip, rtt);
                } else {
                    println!("    {:2}: * * *", hop.hop_number);
                }
            }
        }
        Err(e) => {
            eprintln!("Traceroute failed: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}

async fn handle_dns(domain: String, query_type: String, nameserver: Option<String>) -> anyhow::Result<()> {
    println!("DNS query for {} (type: {})...", domain, query_type);
    
    let dns_service = if let Some(ns) = nameserver.as_ref() {
        DnsService::new_with_nameserver(ns).await?
    } else {
        DnsService::new().await?
    };
    
    let query_type = match query_type.to_uppercase().as_str() {
        "A" => DnsQueryType::A,
        "AAAA" => DnsQueryType::AAAA,
        "CNAME" => DnsQueryType::CNAME,
        "MX" => DnsQueryType::MX,
        "TXT" => DnsQueryType::TXT,
        "NS" => DnsQueryType::NS,
        "SOA" => DnsQueryType::SOA,
        "PTR" => DnsQueryType::PTR,
        "ALL" => DnsQueryType::ALL,
        _ => {
            eprintln!("Invalid query type: {}", query_type);
            std::process::exit(1);
        }
    };
    
    let config = DnsConfig {
        domain: domain.clone(),
        query_type,
        nameserver: nameserver.clone(),
        ..Default::default()
    };
    
    match dns_service.query(config).await {
        Ok(result) => {
            println!("DNS query results for {} (type: {}):", result.domain, result.query_type);
            println!("  Response time: {:.2}ms", result.response_time);
            println!("  Records:");
            for record in &result.records {
                println!("    {}: {} (TTL: {})", record.record_type, record.value, record.ttl);
            }
        }
        Err(e) => {
            eprintln!("DNS query failed: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}

async fn handle_server(host: String, port: u16) -> anyhow::Result<()> {
    println!("Starting API server on {}:{}...", host, port);
    
    use crate::api::create_api_router;
    use crate::websocket::create_websocket_router;
    use axum::{serve, routing::get};
    use tower_http::cors::CorsLayer;
    
    let api_router = create_api_router().await;
    let ws_router = create_websocket_router().await;
    
    let app = api_router
        .merge(ws_router)
        .layer(CorsLayer::permissive())
        .route("/", get(|| async { "Network Probe API Server" }));
    
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    println!("Server listening on http://{}", addr);
    println!("API endpoints:");
    println!("  POST /api/ping - ICMP ping test");
    println!("  POST /api/tcping - TCP connection test");
    println!("  POST /api/website - Website test");
    println!("  POST /api/traceroute - Traceroute");
    println!("  POST /api/dns - DNS query");
    println!("  GET  /api/health - Health check");
    println!("  GET  /api/status - Service status");
    println!("  GET  /ws - WebSocket endpoint");
    
    serve(listener, app).await?;
    
    Ok(())
}

async fn handle_port_scan(host: String, range: String, timeout: Duration) -> anyhow::Result<()> {
    println!("Scanning ports on {} (range: {})...", host, range);
    
    // 解析端口范围
    let ports = parse_port_range(&range)?;
    
    let service = TcpingService::new();
    let results = service.scan_ports(&host, ports, timeout).await?;
    
    println!("Port scan results for {}:", host);
    let open_ports: Vec<_> = results.iter()
        .filter(|(_, is_open)| *is_open)
        .map(|(port, _)| port)
        .collect();
    
    let closed_ports: Vec<_> = results.iter()
        .filter(|(_, is_open)| !*is_open)
        .map(|(port, _)| port)
        .collect();
    
    println!("  Open ports: {:?}", open_ports);
    println!("  Closed ports: {} total", closed_ports.len());
    
    Ok(())
}

fn parse_port_range(range: &str) -> anyhow::Result<Vec<u16>> {
    if range.contains('-') {
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid port range format"));
        }
        
        let start: u16 = parts[0].parse()?;
        let end: u16 = parts[1].parse()?;
        
        if start > end {
            return Err(anyhow::anyhow!("Invalid port range: start > end"));
        }
        
        Ok((start..=end).collect())
    } else {
        let port: u16 = range.parse()?;
        Ok(vec![port])
    }
}

async fn handle_install(host: String, port: u16, service_name: String) -> anyhow::Result<()> {
    println!("Installing systemd service '{}'...", service_name);
    
    let service_file_path = format!("/etc/systemd/system/{}.service", service_name);
    
    let current_exe = std::env::current_exe()
        .map_err(|e| anyhow::anyhow!("Failed to get current executable path: {}", e))?;
    let exe_path = current_exe.to_string_lossy().to_string();
    
    let service_content = format!(
        r#"[Unit]
Description=Network Probe Service
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory={}
ExecStart={} server --host {} --port {}
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
"#,
        std::env::current_dir()
            .map(|d| d.to_string_lossy().to_string())
            .unwrap_or_else(|_| "/".to_string()),
        exe_path,
        host,
        port
    );
    
    let service_dir = "/etc/systemd/system";
    if !std::path::Path::new(service_dir).exists() {
        return Err(anyhow::anyhow!("Systemd service directory {} does not exist. This command must be run on Linux with systemd.", service_dir));
    }
    
    std::fs::write(&service_file_path, service_content)
        .map_err(|e| anyhow::anyhow!("Failed to write service file: {}", e))?;
    
    println!("Service file created at: {}", service_file_path);
    
    let reload_status = std::process::Command::new("systemctl")
        .args(["daemon-reload"])
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to run systemctl daemon-reload: {}", e))?;
    
    if !reload_status.success() {
        return Err(anyhow::anyhow!("systemctl daemon-reload failed"));
    }
    
    println!("Systemd daemon reloaded");
    
    let enable_status = std::process::Command::new("systemctl")
        .args(["enable", &service_name])
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to enable service: {}", e))?;
    
    if !enable_status.success() {
        return Err(anyhow::anyhow!("Failed to enable service"));
    }
    
    println!("Service '{}' enabled", service_name);
    
    let start_status = std::process::Command::new("systemctl")
        .args(["start", &service_name])
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to start service: {}", e))?;
    
    if !start_status.success() {
        return Err(anyhow::anyhow!("Failed to start service"));
    }
    
    println!("Service '{}' started successfully", service_name);
    println!("\nTo check service status: systemctl status {}", service_name);
    println!("To view logs: journalctl -u {} -f", service_name);
    println!("To stop service: systemctl stop {}", service_name);
    println!("To restart service: systemctl restart {}", service_name);
    
    Ok(())
}

async fn handle_uninstall(service_name: String) -> anyhow::Result<()> {
    println!("Uninstalling systemd service '{}'...", service_name);
    
    let service_file_path = format!("/etc/systemd/system/{}.service", service_name);
    
    let stop_status = std::process::Command::new("systemctl")
        .args(["stop", &service_name])
        .status();
    
    if let Ok(status) = stop_status {
        if status.success() {
            println!("Service '{}' stopped", service_name);
        }
    }
    
    let disable_status = std::process::Command::new("systemctl")
        .args(["disable", &service_name])
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to disable service: {}", e))?;
    
    if !disable_status.success() {
        return Err(anyhow::anyhow!("Failed to disable service"));
    }
    
    println!("Service '{}' disabled", service_name);
    
    if std::path::Path::new(&service_file_path).exists() {
        std::fs::remove_file(&service_file_path)
            .map_err(|e| anyhow::anyhow!("Failed to remove service file: {}", e))?;
        println!("Service file removed: {}", service_file_path);
    } else {
        println!("Service file not found: {}", service_file_path);
    }
    
    let reload_status = std::process::Command::new("systemctl")
        .args(["daemon-reload"])
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to run systemctl daemon-reload: {}", e))?;
    
    if !reload_status.success() {
        return Err(anyhow::anyhow!("systemctl daemon-reload failed"));
    }
    
    println!("Systemd daemon reloaded");
    println!("Service '{}' uninstalled successfully", service_name);
    
    Ok(())
}