use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// 默认服务器配置文件路径
const DEFAULT_SERVER_CONFIG_PATH: &str = "configs/server.yaml";

/// HTTP配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Http {
    #[serde(rename = "on")]
    pub on: bool,
    pub listen: Vec<String>,
}

/// HTTPS配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Https {
    #[serde(rename = "on")]
    pub on: bool,
    pub listen: Vec<String>,
    pub cert: String,
    pub key: String,
}

/// 服务器配置结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Server {
    pub env: String,
    pub http: Http,
    pub https: Https,
}

impl Server {
    /// 从YAML文件加载配置
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let yaml_content = fs::read_to_string(path)?;
        let config: Server = serde_yaml::from_str(&yaml_content)?;
        Ok(config)
    }

    /// 从默认路径加载配置
    pub fn load_default() -> Result<Self, Box<dyn std::error::Error>> {
        Self::load_from_file(DEFAULT_SERVER_CONFIG_PATH)
    }

    /// 验证配置是否有效
    pub fn validate(&self) -> Result<(), String> {
        if self.env.is_empty() {
            return Err("环境配置不能为空".to_string());
        }

        if self.http.on && self.http.listen.is_empty() {
            return Err("HTTP服务已启用但未配置监听地址".to_string());
        }

        if self.https.on && self.https.listen.is_empty() {
            return Err("HTTPS服务已启用但未配置监听地址".to_string());
        }

        if self.https.on && (self.https.cert.is_empty() || self.https.key.is_empty()) {
            return Err("HTTPS服务已启用但未配置证书文件".to_string());
        }

        Ok(())
    }

    /// 获取HTTP监听地址
    pub fn get_http_addresses(&self) -> Vec<&str> {
        if self.http.on {
            self.http.listen.iter().map(|s| s.as_str()).collect()
        } else {
            vec![]
        }
    }

    /// 获取HTTPS监听地址
    pub fn get_https_addresses(&self) -> Vec<&str> {
        if self.https.on {
            self.https.listen.iter().map(|s| s.as_str()).collect()
        } else {
            vec![]
        }
    }

    /// 检查是否为生产环境
    pub fn is_production(&self) -> bool {
        self.env == "prod" || self.env == "production"
    }

    /// 检查是否为开发环境
    pub fn is_development(&self) -> bool {
        self.env == "dev" || self.env == "development"
    }
}

/// 配置管理器
pub struct Manager {
    server: Server,
}

impl Manager {
    /// 创建新的配置管理器
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let server = Server::load_default()?;
        server
            .validate()
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;

        Ok(Manager { server })
    }

    /// 创建包含API管理员配置的配置管理器
    pub fn new_with_api_admin() -> Result<Self, Box<dyn std::error::Error>> {
        let server = Server::load_default()?;
        server
            .validate()
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;
        Ok(Manager { server })
    }

    /// 获取服务器配置
    pub fn server(&self) -> &Server {
        &self.server
    }

    /// 重新加载服务器配置
    pub fn reload_server(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let new_server = Server::load_default()?;
        new_server
            .validate()
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;
        self.server = new_server;
        Ok(())
    }

    /// 重新加载所有配置
    pub fn reload_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.reload_server()?;
        Ok(())
    }
}
