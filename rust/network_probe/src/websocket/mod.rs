use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    routing::get,
    Router,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::broadcast;

use crate::modules::{
    ping::{PingConfig, PingService},
    tcping::{TcpingConfig, TcpingService},
    website::{WebsiteTestConfig, WebsiteTestService},
    traceroute::{TracerouteConfig, TracerouteService},
    dns::{DnsConfig, DnsService, DnsQueryType},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WebSocketMessage {
    Ping { host: String, count: Option<u32> },
    Tcping { host: String, port: u16, count: Option<u32> },
    Website { url: String, method: Option<String> },
    Traceroute { host: String, max_hops: Option<u32> },
    Dns { domain: String, query_type: Option<String>, nameserver: Option<String> },
    Subscribe { event: String },
    Unsubscribe { event: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketResponse {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct WebSocketHandler {
    ping_service: Arc<PingService>,
    tcping_service: Arc<TcpingService>,
    website_service: Arc<WebsiteTestService>,
    traceroute_service: Arc<TracerouteService>,
    dns_service: Arc<tokio::sync::RwLock<DnsService>>,
    tx: broadcast::Sender<String>,
}

impl WebSocketHandler {
    pub async fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        
        Self {
            ping_service: Arc::new(PingService::new()),
            tcping_service: Arc::new(TcpingService::new()),
            website_service: Arc::new(WebsiteTestService::new()),
            traceroute_service: Arc::new(TracerouteService::new()),
            dns_service: Arc::new(tokio::sync::RwLock::new(
                DnsService::new().await.unwrap()
            )),
            tx,
        }
    }

    pub async fn create_router(&self) -> Router {
        let handler = Arc::new(self.clone());
        
        Router::new()
            .route("/ws", get(move |ws: WebSocketUpgrade| {
                let handler = handler.clone();
                async move {
                    ws.on_upgrade(move |socket| handle_websocket(socket, handler))
                }
            }))
    }

    async fn handle_message(&self, msg: WebSocketMessage) -> WebSocketResponse {
        let timestamp = chrono::Utc::now();
        
        match msg {
            WebSocketMessage::Ping { host, count } => {
                let config = PingConfig {
                    host,
                    count: count.unwrap_or(4),
                    ..Default::default()
                };
                
                match self.ping_service.ping(config).await {
                    Ok(result) => WebSocketResponse {
                        success: true,
                        data: Some(json!(result)),
                        error: None,
                        timestamp,
                    },
                    Err(e) => WebSocketResponse {
                        success: false,
                        data: None,
                        error: Some(e.to_string()),
                        timestamp,
                    },
                }
            }
            
            WebSocketMessage::Tcping { host, port, count } => {
                let config = TcpingConfig {
                    host,
                    port,
                    count: count.unwrap_or(4),
                    ..Default::default()
                };
                
                match self.tcping_service.tcping(config).await {
                    Ok(result) => WebSocketResponse {
                        success: true,
                        data: Some(json!(result)),
                        error: None,
                        timestamp,
                    },
                    Err(e) => WebSocketResponse {
                        success: false,
                        data: None,
                        error: Some(e.to_string()),
                        timestamp,
                    },
                }
            }
            
            WebSocketMessage::Website { url, method } => {
                let config = WebsiteTestConfig {
                    url,
                    method: method.unwrap_or_else(|| "GET".to_string()),
                    ..Default::default()
                };
                
                match self.website_service.test_website(config).await {
                    Ok(result) => WebSocketResponse {
                        success: true,
                        data: Some(json!(result)),
                        error: None,
                        timestamp,
                    },
                    Err(e) => WebSocketResponse {
                        success: false,
                        data: None,
                        error: Some(e.to_string()),
                        timestamp,
                    },
                }
            }
            
            WebSocketMessage::Traceroute { host, max_hops } => {
                let config = TracerouteConfig {
                    host,
                    max_hops: max_hops.unwrap_or(30),
                    ..Default::default()
                };
                
                match self.traceroute_service.traceroute(config).await {
                    Ok(result) => WebSocketResponse {
                        success: true,
                        data: Some(json!(result)),
                        error: None,
                        timestamp,
                    },
                    Err(e) => WebSocketResponse {
                        success: false,
                        data: None,
                        error: Some(e.to_string()),
                        timestamp,
                    },
                }
            }
            
            WebSocketMessage::Dns { domain, query_type, nameserver } => {
                let query_type = match query_type.as_deref() {
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
                    domain,
                    query_type,
                    nameserver,
                    ..Default::default()
                };
                
                let dns_service = self.dns_service.read().await;
                match dns_service.query(config).await {
                    Ok(result) => WebSocketResponse {
                        success: true,
                        data: Some(json!(result)),
                        error: None,
                        timestamp,
                    },
                    Err(e) => WebSocketResponse {
                        success: false,
                        data: None,
                        error: Some(e.to_string()),
                        timestamp,
                    },
                }
            }
            
            WebSocketMessage::Subscribe { event } => {
                // 处理订阅事件
                WebSocketResponse {
                    success: true,
                    data: Some(json!({"subscribed_to": event})),
                    error: None,
                    timestamp,
                }
            }
            
            WebSocketMessage::Unsubscribe { event } => {
                // 处理取消订阅事件
                WebSocketResponse {
                    success: true,
                    data: Some(json!({"unsubscribed_from": event})),
                    error: None,
                    timestamp,
                }
            }
        }
    }
}

impl Clone for WebSocketHandler {
    fn clone(&self) -> Self {
        Self {
            ping_service: self.ping_service.clone(),
            tcping_service: self.tcping_service.clone(),
            website_service: self.website_service.clone(),
            traceroute_service: self.traceroute_service.clone(),
            dns_service: self.dns_service.clone(),
            tx: self.tx.clone(),
        }
    }
}

async fn handle_websocket(socket: WebSocket, handler: Arc<WebSocketHandler>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = handler.tx.subscribe();
    
    // 创建消息处理任务
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });
    
    // 创建消息接收任务
    let mut recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                match msg {
                    Message::Text(text) => {
                        match serde_json::from_str::<WebSocketMessage>(&text) {
                            Ok(ws_msg) => {
                                let response = handler.handle_message(ws_msg).await;
                                let response_text = serde_json::to_string(&response).unwrap();
                                
                                // 发送响应
                                if let Err(e) = handler.tx.send(response_text) {
                                    log::error!("Failed to send WebSocket response: {}", e);
                                }
                            }
                            Err(e) => {
                                let error_response = WebSocketResponse {
                                    success: false,
                                    data: None,
                                    error: Some(format!("Invalid message format: {}", e)),
                                    timestamp: chrono::Utc::now(),
                                };
                                
                                let error_text = serde_json::to_string(&error_response).unwrap();
                                if let Err(e) = handler.tx.send(error_text) {
                                    log::error!("Failed to send error response: {}", e);
                                }
                            }
                        }
                    }
                    Message::Close(_) => break,
                    _ => {}
                }
            } else {
                break;
            }
        }
    });
    
    // 等待任一任务完成
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        }
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }
}

pub async fn create_websocket_router() -> Router {
    let handler = WebSocketHandler::new().await;
    handler.create_router().await
}