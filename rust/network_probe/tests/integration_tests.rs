use network_probe::modules::{
    ping::{PingConfig, PingService, PingResult},
    tcping::{TcpingConfig, TcpingService, TcpingResult},
    website::{WebsiteTestConfig, WebsiteTestService, WebsiteTestResult},
    traceroute::{TracerouteConfig, TracerouteService, TracerouteResult},
    dns::{DnsConfig, DnsService, DnsQueryType, DnsQueryResult},
};
use std::time::Duration;

#[tokio::test]
async fn test_ping_integration() {
    let service = PingService::new();
    let config = PingConfig {
        host: "127.0.0.1".to_string(),
        count: 2,
        ..Default::default()
    };
    
    let result: Result<PingResult, _> = service.ping(config).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.host, "127.0.0.1");
    assert!(result.packets_received > 0);
}

#[tokio::test]
async fn test_tcping_integration() {
    let service = TcpingService::new();
    let config = TcpingConfig {
        host: "127.0.0.1".to_string(),
        port: 22, // SSH port, might not be open
        count: 2,
        ..Default::default()
    };
    
    let result: Result<TcpingResult, _> = service.tcping(config).await;
    // This might fail if port 22 is not open, so we just check it's a valid result
    if let Ok(result) = result {
        assert_eq!(result.host, "127.0.0.1");
        assert_eq!(result.port, 22);
    }
}

#[tokio::test]
async fn test_website_integration() {
    let service = WebsiteTestService::new();
    let config = WebsiteTestConfig {
        url: "https://httpbin.org/status/200".to_string(),
        ..Default::default()
    };
    
    let result: Result<WebsiteTestResult, _> = service.test_website(config).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.status_code, Some(200));
    assert!(result.success);
}

#[tokio::test]
async fn test_dns_integration() {
    let service: DnsService = DnsService::new().await.unwrap();
    let config = DnsConfig {
        domain: "google.com".to_string(),
        query_type: DnsQueryType::A,
        ..Default::default()
    };
    
    let result: Result<DnsQueryResult, _> = service.query(config).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.domain, "google.com");
    assert!(!result.records.is_empty());
}

#[tokio::test]
async fn test_traceroute_integration() {
    let service = TracerouteService::new();
    let config = TracerouteConfig {
        host: "127.0.0.1".to_string(),
        max_hops: 5,
        ..Default::default()
    };
    
    let result: Result<TracerouteResult, _> = service.traceroute(config).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.host, "127.0.0.1");
    assert!(!result.hops.is_empty());
}

#[tokio::test]
async fn test_multiple_operations() {
    // Test multiple operations in sequence
    let ping_service = PingService::new();
    let tcping_service = TcpingService::new();
    let website_service = WebsiteTestService::new();
    
    // Ping test
    let ping_config = PingConfig {
        host: "127.0.0.1".to_string(),
        count: 1,
        ..Default::default()
    };
    let ping_result: Result<PingResult, _> = ping_service.ping(ping_config).await;
    assert!(ping_result.is_ok());
    
    // TCP test
    let tcping_config = TcpingConfig {
        host: "127.0.0.1".to_string(),
        port: 80,
        count: 1,
        ..Default::default()
    };
    let tcping_result: Result<TcpingResult, _> = tcping_service.tcping(tcping_config).await;
    // Port 80 might not be open, so we don't assert success
    
    // Website test
    let website_config = WebsiteTestConfig {
        url: "https://httpbin.org/status/200".to_string(),
        ..Default::default()
    };
    let website_result: Result<WebsiteTestResult, _> = website_service.test_website(website_config).await;
    assert!(website_result.is_ok());
}