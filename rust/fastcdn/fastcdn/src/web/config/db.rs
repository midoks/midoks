use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

/// 默认服务器配置文件路径
const CONF_YAML: &str = "configs/db.yaml";

/// 数据库配置结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Db {
    pub user: String,
    pub password: String,
    pub database: String,
    pub host: String,
}

// 使用 lazy_static 实现线程安全的单例
lazy_static! {
    static ref INSTANCE: Arc<Mutex<Option<Db>>> = Arc::new(Mutex::new(None));
}

impl Db {
    /// 获取单例实例
    pub fn instance() -> Result<Arc<Mutex<Db>>, Box<dyn std::error::Error>> {
        let mut instance_guard = INSTANCE.lock().unwrap();
        
        if instance_guard.is_none() {
            let db = Self::load_default()?;
            db.validate()
                .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;
            *instance_guard = Some(db);
        }
        
        // 创建一个新的 Arc<Mutex<Db>> 包装实际的 Db 实例
        let db = instance_guard.as_ref().unwrap().clone();
        Ok(Arc::new(Mutex::new(db)))
    }

    /// 重新加载配置（更新单例实例）
    pub fn reload() -> Result<(), Box<dyn std::error::Error>> {
        let new_db = Self::load_default()?;
        new_db.validate()
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;
        
        let mut instance_guard = INSTANCE.lock().unwrap();
        *instance_guard = Some(new_db);
        Ok(())
    }

    /// 从YAML文件加载配置
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let yaml_content = fs::read_to_string(path)?;
        let db: Db = serde_yaml::from_str(&yaml_content)?;
        Ok(db)
    }

    /// 从默认路径加载配置
    pub fn load_default() -> Result<Self, Box<dyn std::error::Error>> {
        Self::load_from_file(CONF_YAML)
    }

    /// 验证配置是否有效
    pub fn validate(&self) -> Result<(), String> {
        if self.user.is_empty() {
            return Err("user cannot be empty".to_string());
        }

        if self.password.is_empty() {
            return Err("password cannot be empty".to_string());
        }

        if self.database.is_empty() {
            return Err("database cannot be empty".to_string());
        }

        if self.host.is_empty() {
            return Err("host cannot be empty".to_string());
        }

        Ok(())
    }
}

/// 配置管理器
pub struct Manager {
    // 不再直接持有db实例，而是通过单例获取
}

impl Manager {
    /// 创建新的配置管理器
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // 确保单例已初始化
        Db::instance()?;
        Ok(Manager {})
    }

    /// 创建包含API管理员配置的配置管理器
    pub fn new_db() -> Result<Self, Box<dyn std::error::Error>> {
        // 确保单例已初始化
        Db::instance()?;
        Ok(Manager {})
    }

    /// 获取服务器配置
    pub fn db(&self) -> Result<Arc<Mutex<Db>>, Box<dyn std::error::Error>> {
        Db::instance()
    }

    /// 重新加载服务器配置
    pub fn reload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Db::reload()
    }
}
