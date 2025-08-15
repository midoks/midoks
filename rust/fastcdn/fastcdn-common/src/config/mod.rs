pub mod api_admin;
pub mod db;
pub mod server;

pub use api_admin as ConfigApiAdmin;
pub use db as ConfigDb;
pub use server as ConfigServer;

use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;

/// 通用的YAML配置文件加载函数
pub fn load_from_file<T, P>(path: P) -> Result<T, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let yaml_content = fs::read_to_string(path)?;
    let config: T = serde_yaml::from_str(&yaml_content)?;
    Ok(config)
}

/// 通用的默认配置加载函数
pub fn load_default<T>(default_path: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    load_from_file(default_path)
}
