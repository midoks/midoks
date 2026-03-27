use std::time::Duration;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::utils::error::{NetworkError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteTestResult {
    pub url: String,
    pub status_code: Option<u16>,
    pub response_time: f64,
    pub content_length: Option<usize>,
    pub success: bool,
    pub error_message: Option<String>,
    pub headers: std::collections::HashMap<String, String>,
    pub ssl_info: Option<SslInfo>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SslInfo {
    pub issuer: String,
    pub subject: String,
    pub valid_from: DateTime<Utc>,
    pub valid_to: DateTime<Utc>,
    pub days_until_expiry: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteTestConfig {
    pub url: String,
    pub method: String,
    pub timeout: Duration,
    pub follow_redirects: bool,
    pub verify_ssl: bool,
    pub headers: std::collections::HashMap<String, String>,
}

impl Default for WebsiteTestConfig {
    fn default() -> Self {
        let mut headers = std::collections::HashMap::new();
        headers.insert("User-Agent".to_string(), "NetworkProbe/1.0".to_string());
        
        Self {
            url: String::new(),
            method: "GET".to_string(),
            timeout: Duration::from_secs(30),
            follow_redirects: true,
            verify_ssl: true,
            headers,
        }
    }
}

pub struct WebsiteTestService {
    client: Client,
}

impl WebsiteTestService {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();
        
        Self { client }
    }

    pub async fn test_website(&self, config: WebsiteTestConfig) -> Result<WebsiteTestResult> {
        let start_time = std::time::Instant::now();
        
        let client = Client::builder()
            .timeout(config.timeout)
            .danger_accept_invalid_certs(!config.verify_ssl)
            .redirect(if config.follow_redirects {
                reqwest::redirect::Policy::default()
            } else {
                reqwest::redirect::Policy::none()
            })
            .build()
            .map_err(|e| NetworkError::Http(format!("Failed to build HTTP client: {}", e)))?;

        let mut request = match config.method.to_uppercase().as_str() {
            "GET" => client.get(&config.url),
            "POST" => client.post(&config.url),
            "PUT" => client.put(&config.url),
            "DELETE" => client.delete(&config.url),
            "HEAD" => client.head(&config.url),
            _ => return Err(NetworkError::InvalidInput(format!("Unsupported HTTP method: {}", config.method))),
        };

        // 添加自定义头部
        for (key, value) in &config.headers {
            request = request.header(key, value);
        }

        match request.send().await {
            Ok(response) => {
                let response_time = start_time.elapsed().as_secs_f64() * 1000.0;
                let status_code = response.status().as_u16();
                let success = response.status().is_success();
                
                let content_length = match response.content_length() {
                    Some(len) => Some(len as usize),
                    None => None,
                };

                // 获取响应头部
                let mut headers = std::collections::HashMap::new();
                for (key, value) in response.headers() {
                    if let Ok(value_str) = value.to_str() {
                        headers.insert(key.to_string(), value_str.to_string());
                    }
                }

                // 获取SSL信息（如果是HTTPS）
                let ssl_info = if config.url.starts_with("https://") {
                    self.get_ssl_info(&config.url).await.ok()
                } else {
                    None
                };

                Ok(WebsiteTestResult {
                    url: config.url,
                    status_code: Some(status_code),
                    response_time,
                    content_length,
                    success,
                    error_message: None,
                    headers,
                    ssl_info,
                    timestamp: Utc::now(),
                })
            }
            Err(e) => {
                let response_time = start_time.elapsed().as_secs_f64() * 1000.0;
                Ok(WebsiteTestResult {
                    url: config.url,
                    status_code: None,
                    response_time,
                    content_length: None,
                    success: false,
                    error_message: Some(e.to_string()),
                    headers: std::collections::HashMap::new(),
                    ssl_info: None,
                    timestamp: Utc::now(),
                })
            }
        }
    }

    async fn get_ssl_info(&self, _url: &str) -> Result<SslInfo> {
        // 这里简化处理，实际需要更复杂的SSL证书解析
        // 在实际实现中，可以使用 rustls 或其他SSL库来获取证书信息
        
        // 临时返回模拟数据
        Ok(SslInfo {
            issuer: "Example CA".to_string(),
            subject: "example.com".to_string(),
            valid_from: Utc::now(),
            valid_to: Utc::now() + chrono::Duration::days(365),
            days_until_expiry: 365,
        })
    }

    pub async fn test_multiple_urls(&self, urls: Vec<String>) -> Result<Vec<WebsiteTestResult>> {
        let mut results = Vec::new();
        
        for url in urls {
            let config = WebsiteTestConfig {
                url,
                ..Default::default()
            };
            
            match self.test_website(config).await {
                Ok(result) => results.push(result),
                Err(e) => {
                    log::error!("Failed to test website: {}", e);
                    // 创建失败结果
                    results.push(WebsiteTestResult {
                        url: e.to_string(),
                        status_code: None,
                        response_time: 0.0,
                        content_length: None,
                        success: false,
                        error_message: Some(e.to_string()),
                        headers: std::collections::HashMap::new(),
                        ssl_info: None,
                        timestamp: Utc::now(),
                    });
                }
            }
        }
        
        Ok(results)
    }

    pub async fn health_check(&self, url: &str) -> Result<bool> {
        let config = WebsiteTestConfig {
            url: url.to_string(),
            ..Default::default()
        };
        
        let result = self.test_website(config).await?;
        Ok(result.success)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_website_test() {
        let service = WebsiteTestService::new();
        let config = WebsiteTestConfig {
            url: "https://httpbin.org/status/200".to_string(),
            ..Default::default()
        };
        
        let result = service.test_website(config).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.status_code, Some(200));
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_health_check() {
        let service = WebsiteTestService::new();
        let result = service.health_check("https://httpbin.org/status/200").await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}