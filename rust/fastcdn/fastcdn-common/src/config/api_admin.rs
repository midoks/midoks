use super::{load_default, load_from_file};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// 默认服务器配置文件路径
const CONF_YAML: &str = "configs/api_admin.yaml";

/// API管理员配置结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiAdmin {
    #[serde(rename = "rpc.endpoints")]
    pub rpc_endpoints: Vec<String>,
    #[serde(rename = "rpc.disableUpdate")]
    pub rpc_disable_update: bool,
    #[serde(rename = "nodeId")]
    pub node_id: String,
    pub secret: String,
}

// 使用 lazy_static 实现线程安全的单例
lazy_static! {
    static ref INSTANCE: Arc<Mutex<Option<ApiAdmin>>> = Arc::new(Mutex::new(None));
}

impl ApiAdmin {
    /// 获取单例实例
    pub fn instance() -> Result<Arc<Mutex<ApiAdmin>>, Box<dyn std::error::Error>> {
        let mut instance_guard = INSTANCE.lock().unwrap();

        if instance_guard.is_none() {
            let api_admin = Self::load_default()?;

            println!("{:?}", api_admin);
            api_admin
                .validate()
                .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;
            *instance_guard = Some(api_admin);
        }

        // 创建一个新的 Arc<Mutex<ApiAdmin>> 包装实际的 ApiAdmin 实例
        let api_admin = instance_guard.as_ref().unwrap().clone();
        Ok(Arc::new(Mutex::new(api_admin)))
    }

    /// 从YAML文件加载API管理员配置
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        load_from_file(path)
    }

    /// 从默认路径加载API管理员配置
    pub fn load_default() -> Result<Self, Box<dyn std::error::Error>> {
        let exec_path = std::env::current_exe()?;
        let root_path = exec_path.parent().and_then(|p| p.parent());

        let api_admin_file = match root_path {
            Some(path) => path.join(CONF_YAML).to_string_lossy().to_string(),
            None => CONF_YAML.to_string(),
        };
        load_default(&api_admin_file)
    }

    /// 验证API管理员配置是否有效
    pub fn validate(&self) -> Result<(), String> {
        if self.rpc_endpoints.is_empty() {
            return Err("rpc endpoint list cannot be empty!".to_string());
        }

        if self.node_id.is_empty() {
            return Err("node_id cannot be empty!".to_string());
        }

        if self.secret.is_empty() {
            return Err("secret cannot be empty!".to_string());
        }

        for endpoint in &self.rpc_endpoints {
            if !endpoint.starts_with("http://") && !endpoint.starts_with("https://") {
                return Err(format!("invalid rpc endpoint format: {}", endpoint));
            }
        }

        Ok(())
    }

    /// 获取主要RPC端点
    pub fn get_primary_endpoint(&self) -> Option<&str> {
        self.rpc_endpoints.first().map(|s| s.as_str())
    }

    /// 获取所有RPC端点
    pub fn get_all_endpoints(&self) -> Vec<&str> {
        self.rpc_endpoints.iter().map(|s| s.as_str()).collect()
    }

    /// 检查是否禁用更新
    pub fn is_update_disabled(&self) -> bool {
        self.rpc_disable_update
    }

    /// 获取备用端点
    pub fn get_backup_endpoints(&self) -> Vec<&str> {
        if self.rpc_endpoints.len() > 1 {
            self.rpc_endpoints[1..].iter().map(|s| s.as_str()).collect()
        } else {
            vec![]
        }
    }

    /// 将当前配置写入/覆盖到本地YAML文件
    pub fn write(&self) -> Result<(), Box<dyn std::error::Error>> {
        let exec_path = std::env::current_exe()?;
        let root_path = exec_path.parent().and_then(|p| p.parent());

        let api_admin_file = match root_path {
            Some(path) => path.join(CONF_YAML).to_string_lossy().to_string(),
            None => CONF_YAML.to_string(),
        };
        self.write_to_file(&api_admin_file)
    }

    /// 将当前配置写入/覆盖到指定路径的YAML文件
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let yaml_content = serde_yaml::to_string(self)?;
        std::fs::write(path, yaml_content)?;
        Ok(())
    }

    /// 验证请求的nodeId和secret是否匹配
    pub fn verify_credentials(&self, node_id: &str, secret: &str) -> bool {
        self.node_id == node_id && self.secret == secret
    }
}
