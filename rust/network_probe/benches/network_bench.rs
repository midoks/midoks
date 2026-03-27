use criterion::{black_box, criterion_group, criterion_main, Criterion};
use network_probe::modules::{
    ping::{PingConfig, PingService},
    tcping::{TcpingConfig, TcpingService},
    website::{WebsiteTestConfig, WebsiteTestService},
    dns::{DnsConfig, DnsService, DnsQueryType},
};
use std::time::Duration;
use tokio::runtime::Runtime;

fn benchmark_ping(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("ping_localhost", |b| {
        b.iter(|| {
            rt.block_on(async {
                let service = PingService::new();
                let config = PingConfig {
                    host: "127.0.0.1".to_string(),
                    count: 1,
                    ..Default::default()
                };
                let _ = service.ping(black_box(config)).await;
            })
        })
    });
}

fn benchmark_tcping(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("tcping_localhost", |b| {
        b.iter(|| {
            rt.block_on(async {
                let service = TcpingService::new();
                let config = TcpingConfig {
                    host: "127.0.0.1".to_string(),
                    port: 80,
                    count: 1,
                    ..Default::default()
                };
                let _ = service.tcping(black_box(config)).await;
            })
        })
    });
}

fn benchmark_website(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("website_test", |b| {
        b.iter(|| {
            rt.block_on(async {
                let service = WebsiteTestService::new();
                let config = WebsiteTestConfig {
                    url: "https://httpbin.org/status/200".to_string(),
                    ..Default::default()
                };
                let _ = service.test_website(black_box(config)).await;
            })
        })
    });
}

fn benchmark_dns(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("dns_query", |b| {
        b.iter(|| {
            rt.block_on(async {
                let service = DnsService::new().await.unwrap();
                let config = DnsConfig {
                    domain: "google.com".to_string(),
                    query_type: DnsQueryType::A,
                    ..Default::default()
                };
                let _ = service.query(black_box(config)).await;
            })
        })
    });
}

fn benchmark_concurrent_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("concurrent_ping_tcping", |b| {
        b.iter(|| {
            rt.block_on(async {
                let ping_service = PingService::new();
                let tcping_service = TcpingService::new();
                
                let ping_config = PingConfig {
                    host: "127.0.0.1".to_string(),
                    count: 1,
                    ..Default::default()
                };
                
                let tcping_config = TcpingConfig {
                    host: "127.0.0.1".to_string(),
                    port: 80,
                    count: 1,
                    ..Default::default()
                };
                
                let (ping_result, tcping_result) = tokio::join!(
                    ping_service.ping(ping_config),
                    tcping_service.tcping(tcping_config)
                );
                
                let _ = (ping_result, tcping_result);
            })
        })
    });
}

criterion_group!(
    benches,
    benchmark_ping,
    benchmark_tcping,
    benchmark_website,
    benchmark_dns,
    benchmark_concurrent_operations
);

criterion_main!(benches);