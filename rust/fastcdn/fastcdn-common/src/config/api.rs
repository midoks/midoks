use super::{load_default, load_from_file};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// 默认API节点配置文件路径
const CONF_YAML: &str = "configs/api.yaml";

/// API节点配置结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Api {
    #[serde(rename = "nodeId")]
    pub node_id: String,
    pub secret: String,
}

lazy_static! {
    static ref INSTANCE: Arc<Mutex<Option<Arc<Mutex<Api>>>>> = Arc::new(Mutex::new(None));
}

impl Api {
    /// 获取单例实例
    pub fn instance() -> Result<Arc<Mutex<Api>>, Box<dyn std::error::Error>> {
        let mut instance_guard = INSTANCE.lock().unwrap();
        if instance_guard.is_none() {
            let api = Self::load_default()?;
            api.validate()
                .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;
            let shared_instance = Arc::new(Mutex::new(api));
            *instance_guard = Some(shared_instance.clone());
        }
        Ok(instance_guard.as_ref().unwrap().clone())
    }

    /// 重新加载配置（更新单例实例）
    pub fn reload() -> Result<(), Box<dyn std::error::Error>> {
        let new_api_node = Self::load_default()?;
        new_api_node
            .validate()
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;

        let mut instance_guard = INSTANCE.lock().unwrap();
        if let Some(ref shared_instance) = *instance_guard {
            // 更新现有实例的内容
            let mut config = shared_instance.lock().unwrap();
            *config = new_api_node;
        } else {
            // 如果实例不存在，创建新的
            let shared_instance = Arc::new(Mutex::new(new_api_node));
            *instance_guard = Some(shared_instance);
        }
        Ok(())
    }

    /// 从YAML文件加载API节点配置
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        load_from_file(path)
    }

    /// 从默认路径加载API节点配置
    pub fn load_default() -> Result<Self, Box<dyn std::error::Error>> {
        load_default(CONF_YAML)
    }

    /// 验证API节点配置是否有效
    pub fn validate(&self) -> Result<(), String> {
        if self.node_id.is_empty() {
            return Err("node_id cannot be empty!".to_string());
        }

        if self.secret.is_empty() {
            return Err("secret cannot be empty!".to_string());
        }
        Ok(())
    }

    /// 验证请求的nodeId和secret是否匹配
    pub fn verify_credentials(&self, node_id: &str, secret: &str) -> bool {
        self.node_id == node_id && self.secret == secret
    }

    /// 将当前配置写入/覆盖到本地YAML文件
    pub fn write(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.write_to_file(CONF_YAML)
    }

    /// 将当前配置写入/覆盖到指定路径的YAML文件
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let yaml_content = serde_yaml::to_string(self)?;
        std::fs::write(path, yaml_content)?;
        Ok(())
    }
}
