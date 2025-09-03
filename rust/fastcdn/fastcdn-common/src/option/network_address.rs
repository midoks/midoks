use serde::{Deserialize, Serialize};

/// API管理员配置结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkAddressConfig {
    pub protocal: String,   // 协议，http、tcp、tcp4、tcp6、unix、udp等
    pub host: String,       // 主机地址或主机名，支持变量
    pub port_range: String, // 端口范围，支持 8080、8080-8090、8080:8090

    pub min_port: u32,
    pub max_port: u32,

    pub host_has_variables: bool,
}
