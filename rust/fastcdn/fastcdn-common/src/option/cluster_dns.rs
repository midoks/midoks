use serde::{Deserialize, Serialize};

/// API管理员配置结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClusterDnsConfig {
    pub cname_records: Vec<String>, // 自动加入的CNAME
    pub ttl: u32,                   // 默认TTL，各个DNS服务商对记录的TTL的限制各有不同
    pub cname_as_domain: bool,      // 是否可以像域名一样直接访问CNAME
    pub including_ln_nodes: bool,   // 是否包含Ln节点

    pub nodes_auto_sync: bool,   // 是否自动同步节点状态
    pub servers_auto_sync: bool, // 是否自动同步服务状态
}

pub fn default() -> ClusterDnsConfig {
    ClusterDnsConfig {
        cname_records: Vec::new(),
        ttl: 0,
        cname_as_domain: true,
        including_ln_nodes: true,
        nodes_auto_sync: true,
        servers_auto_sync: true,
    }
}
