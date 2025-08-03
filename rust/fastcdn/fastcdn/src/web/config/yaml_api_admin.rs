use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

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

impl ApiAdmin {
    /// 从YAML文件加载API管理员配置
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let yaml_content = fs::read_to_string(path)?;
        let config: ApiAdmin = serde_yaml::from_str(&yaml_content)?;
        Ok(config)
    }

    /// 从默认路径加载API管理员配置
    pub fn load_default() -> Result<Self, Box<dyn std::error::Error>> {
        Self::load_from_file("configs/api_admin.yaml")
    }

    /// 验证API管理员配置是否有效
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
}
