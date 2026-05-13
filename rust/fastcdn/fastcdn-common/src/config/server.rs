use super::{load_default, load_from_file};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// 默认服务器配置文件路径
const CONF_YAML: &str = "configs/server.yaml";

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
    #[serde(default)]
    pub open_swagger_doc: bool,
    pub http: Http,
    pub https: Https,
}

// 使用 lazy_static 实现线程安全的单例
lazy_static! {
    static ref INSTANCE: Arc<Mutex<Option<Server>>> = Arc::new(Mutex::new(None));
}

impl Server {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Server {
            env: "development".to_string(),
            open_swagger_doc: false,
            http: Http {
                on: true,
                listen: vec!["0.0.0.0:8080".to_string()],
            },
            https: Https {
                on: false,
                listen: vec!["0.0.0.0:8443".to_string()],
                cert: "".to_string(),
                key: "".to_string(),
            },
        })
    }

    /// 获取单例实例
    pub fn instance() -> Result<Arc<Mutex<Server>>, Box<dyn std::error::Error>> {
        let mut instance_guard = INSTANCE.lock().unwrap();

        if instance_guard.is_none() {
            // 优先尝试从默认配置文件加载，失败时回退到内置默认配置
            let server = match Self::load_default() {
                Ok(s) => s,
                Err(_e) => Self::new()?,
            };
            server
                .validate()
                .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;
            *instance_guard = Some(server);
        }

        // 创建一个新的 Arc<Mutex<Db>> 包装实际的 Db 实例
        let server = instance_guard.as_ref().unwrap().clone();
        Ok(Arc::new(Mutex::new(server)))
    }

    /// 从YAML文件加载配置
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        load_from_file(path)
    }

    /// 从默认路径加载配置
    pub fn load_default() -> Result<Self, Box<dyn std::error::Error>> {
        let exec_path = std::env::current_exe()?;
        let root_path = exec_path.parent().and_then(|p| p.parent());

        let server_file = match root_path {
            Some(path) => path.join(CONF_YAML).to_string_lossy().to_string(),
            None => CONF_YAML.to_string(),
        };
        load_default(&server_file)
    }

    /// 验证配置是否有效
    pub fn validate(&self) -> Result<(), String> {
        if self.env.is_empty() {
            return Err("env is not empty".to_string());
        }

        if self.http.on && self.http.listen.is_empty() {
            return Err(
                "http service is enabled but no listening address has been configured".to_string(),
            );
        }

        if self.https.on && self.https.listen.is_empty() {
            return Err(
                "https service has been enabled but no listening address has been configured"
                    .to_string(),
            );
        }

        if self.https.on && (self.https.cert.is_empty() || self.https.key.is_empty()) {
            return Err(
                "https service has been enabled but no certificate file has been configured"
                    .to_string(),
            );
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
