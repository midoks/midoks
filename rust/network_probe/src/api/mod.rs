use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::modules::{
    ping::{PingConfig, PingService},
    tcping::{TcpingConfig, TcpingService},
    website::{WebsiteTestConfig, WebsiteTestService},
    traceroute::{TracerouteConfig, TracerouteService},
    dns::{DnsConfig, DnsService, DnsQueryType},
};

#[derive(Clone)]
pub struct AppState {
    pub ping_service: Arc<PingService>,
    pub tcping_service: Arc<TcpingService>,
    pub website_service: Arc<WebsiteTestService>,
    pub traceroute_service: Arc<TracerouteService>,
    pub dns_service: Arc<RwLock<DnsService>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PingRequest {
    pub host: String,
    pub count: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct TcpingRequest {
    pub host: String,
    pub port: u16,
    pub count: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct WebsiteRequest {
    pub url: String,
    pub method: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TracerouteRequest {
    pub host: String,
    pub max_hops: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct DnsRequest {
    pub domain: String,
    pub query_type: Option<String>,
    pub nameserver: Option<String>,
}

pub async fn create_api_router() -> Router {
    let state = AppState {
        ping_service: Arc::new(PingService::new()),
        tcping_service: Arc::new(TcpingService::new()),
        website_service: Arc::new(WebsiteTestService::new()),
        traceroute_service: Arc::new(TracerouteService::new()),
        dns_service: Arc::new(RwLock::new(DnsService::new().await.unwrap())),
    };

    Router::new()
        .route("/api/ping", post(handle_ping))
        .route("/api/tcping", post(handle_tcping))
        .route("/api/website", post(handle_website))
        .route("/api/traceroute", post(handle_traceroute))
        .route("/api/dns", post(handle_dns))
        .route("/api/health", get(handle_health))
        .route("/api/status", get(handle_status))
        .with_state(state)
}

async fn handle_ping(
    State(state): State<AppState>,
    Json(request): Json<PingRequest>,
) -> impl IntoResponse {
    let config = PingConfig {
        host: request.host,
        count: request.count.unwrap_or(4),
        ..Default::default()
    };

    match state.ping_service.ping(config).await {
        Ok(result) => (StatusCode::OK, Json(ApiResponse::success(result))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ApiResponse::error(e.to_string()))),
    }
}

async fn handle_tcping(
    State(state): State<AppState>,
    Json(request): Json<TcpingRequest>,
) -> impl IntoResponse {
    let config = TcpingConfig {
        host: request.host,
        port: request.port,
        count: request.count.unwrap_or(4),
        ..Default::default()
    };

    match state.tcping_service.tcping(config).await {
        Ok(result) => (StatusCode::OK, Json(ApiResponse::success(result))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ApiResponse::error(e.to_string()))),
    }
}

async fn handle_website(
    State(state): State<AppState>,
    Json(request): Json<WebsiteRequest>,
) -> impl IntoResponse {
    let config = WebsiteTestConfig {
        url: request.url,
        method: request.method.unwrap_or_else(|| "GET".to_string()),
        ..Default::default()
    };

    match state.website_service.test_website(config).await {
        Ok(result) => (StatusCode::OK, Json(ApiResponse::success(result))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ApiResponse::error(e.to_string()))),
    }
}

async fn handle_traceroute(
    State(state): State<AppState>,
    Json(request): Json<TracerouteRequest>,
) -> impl IntoResponse {
    let config = TracerouteConfig {
        host: request.host,
        max_hops: request.max_hops.unwrap_or(30),
        ..Default::default()
    };

    match state.traceroute_service.traceroute(config).await {
        Ok(result) => (StatusCode::OK, Json(ApiResponse::success(result))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ApiResponse::error(e.to_string()))),
    }
}

async fn handle_dns(
    State(state): State<AppState>,
    Json(request): Json<DnsRequest>,
) -> impl IntoResponse {
    let query_type = match request.query_type.as_deref() {
        Some("A") => DnsQueryType::A,
        Some("AAAA") => DnsQueryType::AAAA,
        Some("CNAME") => DnsQueryType::CNAME,
        Some("MX") => DnsQueryType::MX,
        Some("TXT") => DnsQueryType::TXT,
        Some("NS") => DnsQueryType::NS,
        Some("SOA") => DnsQueryType::SOA,
        Some("PTR") => DnsQueryType::PTR,
        Some("ALL") => DnsQueryType::ALL,
        _ => DnsQueryType::A,
    };

    let config = DnsConfig {
        domain: request.domain,
        query_type,
        nameserver: request.nameserver,
        ..Default::default()
    };

    let dns_service = state.dns_service.read().await;
    match dns_service.query(config).await {
        Ok(result) => (StatusCode::OK, Json(ApiResponse::success(result))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ApiResponse::error(e.to_string()))),
    }
}

async fn handle_health() -> impl IntoResponse {
    Json(ApiResponse::success(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now()
    })))
}

async fn handle_status(_state: State<AppState>) -> impl IntoResponse {
    Json(ApiResponse::success(serde_json::json!({
        "services": {
            "ping": "available",
            "tcping": "available",
            "website": "available",
            "traceroute": "available",
            "dns": "available",
        },
        "timestamp": chrono::Utc::now()
    })))
}