use super::{load_default, load_from_file};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};

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
        new_db
            .validate()
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;

        let mut instance_guard = INSTANCE.lock().unwrap();
        *instance_guard = Some(new_db);
        Ok(())
    }

    /// 从YAML文件加载配置
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        load_from_file(path)
    }

    /// 从默认路径加载配置
    pub fn load_default() -> Result<Self, Box<dyn std::error::Error>> {
        let exec_path = std::env::current_exe()?;
        let root_path = exec_path.parent().and_then(|p| p.parent());

        let db_file = match root_path {
            Some(path) => path.join(CONF_YAML).to_string_lossy().to_string(),
            None => CONF_YAML.to_string(),
        };

        // println!("db_file:{:?}", db_file);
        load_default(&db_file)
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

    /// 将当前配置写入/覆盖到本地YAML文件
    pub fn write(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.write_to_file(CONF_YAML)
    }

    /// 将当前配置写入/覆盖到本地YAML文件[api]配置写入
    pub fn write_api(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.write_to_file(format!("fastcdn-api/{}", CONF_YAML))
    }

    /// 将当前配置写入/覆盖到指定路径的YAML文件
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let yaml_content = serde_yaml::to_string(self)?;
        std::fs::write(path, yaml_content)?;
        Ok(())
    }
}
