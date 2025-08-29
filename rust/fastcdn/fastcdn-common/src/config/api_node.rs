use super::{load_default, load_from_file};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// 默认API节点配置文件路径
const CONF_YAML: &str = "configs/api_node.yaml";

/// API节点配置结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiNode {
    #[serde(rename = "rpc.endpoints")]
    pub rpc_endpoints: Vec<String>,
    #[serde(rename = "nodeId")]
    pub node_id: String,
    pub secret: String,
}

// 修改单例实现，直接存储 Arc<Mutex<ApiNode>>
lazy_static! {
    static ref INSTANCE: Arc<Mutex<Option<Arc<Mutex<ApiNode>>>>> = Arc::new(Mutex::new(None));
}

impl ApiNode {
    /// 获取单例实例
    pub fn instance() -> Result<Arc<Mutex<ApiNode>>, Box<dyn std::error::Error>> {
        let mut instance_guard = INSTANCE.lock().unwrap();

        if instance_guard.is_none() {
            let api_node = Self::load_default()?;
            api_node
                .validate()
                .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;

            // 创建真正的单例实例
            let shared_instance = Arc::new(Mutex::new(api_node));
            *instance_guard = Some(shared_instance.clone());
        }

        // 返回共享的实例
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
        if self.rpc_endpoints.is_empty() {
            return Err("RPC端点列表不能为空".to_string());
        }

        if self.node_id.is_empty() {
            return Err("节点ID不能为空".to_string());
        }

        if self.secret.is_empty() {
            return Err("密钥不能为空".to_string());
        }

        // 验证RPC端点格式
        for endpoint in &self.rpc_endpoints {
            if !endpoint.starts_with("http://") && !endpoint.starts_with("https://") {
                return Err(format!("无效的RPC端点格式: {}", endpoint));
            }
        }

        Ok(())
    }

    /// 验证请求的nodeId和secret是否匹配
    pub fn verify_credentials(&self, node_id: &str, secret: &str) -> bool {
        self.node_id == node_id && self.secret == secret
    }

    /// 获取主要RPC端点
    pub fn get_primary_endpoint(&self) -> Option<&str> {
        self.rpc_endpoints.first().map(|s| s.as_str())
    }

    /// 获取所有RPC端点
    pub fn get_all_endpoints(&self) -> Vec<&str> {
        self.rpc_endpoints.iter().map(|s| s.as_str()).collect()
    }
}

/// 配置管理器
pub struct Manager {
    // 不再直接持有api_node实例，而是通过单例获取
}

impl Manager {
    /// 创建新的配置管理器
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // 确保单例已初始化
        ApiNode::instance()?;
        Ok(Manager {})
    }

    /// 获取API节点配置
    pub fn api_node(&self) -> Result<Arc<Mutex<ApiNode>>, Box<dyn std::error::Error>> {
        ApiNode::instance()
    }

    /// 重新加载API节点配置
    pub fn reload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        ApiNode::reload()
    }
}
