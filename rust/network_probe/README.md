# Network Probe

一个高性能的 Rust 网络工具集，支持 ping、tcping、网站测试、路由跟踪、DNS 查询，并提供 API、WebSocket 和命令行接口。

## 功能特性

- **ICMP Ping**: 传统 ping 测试，支持自定义包数量和超时时间
- **TCPing**: TCP 连接测试，可测试端口连通性和延迟
- **网站测试**: HTTP/HTTPS 网站可用性测试，支持多种方法
- **路由跟踪**: 追踪数据包到达目标的路由路径
- **DNS 查询**: 支持多种记录类型的 DNS 查询
- **API 接口**: RESTful API 提供所有功能
- **WebSocket**: 实时网络测试接口
- **命令行**: 完整的 CLI 工具支持
- **高性能**: 异步 I/O，支持并发操作

## 快速开始

### 安装

```bash
cargo build --release
```

### 命令行使用

```bash
# Ping 测试
./target/release/network_probe ping google.com --count 4

# TCP 端口测试
./target/release/network_probe tcping google.com --port 443 --count 3

# 网站测试
./target/release/network_probe website https://google.com

# DNS 查询
./target/release/network_probe dns google.com --query-type A

# 路由跟踪
./target/release/network_probe traceroute google.com

# 启动 API 服务器
./target/release/network_probe server --host 0.0.0.0 --port 8080
```

### API 使用

启动服务器后，可以使用以下 API 端点：

```bash
# Ping 测试
curl -X POST http://localhost:8080/api/ping \
  -H "Content-Type: application/json" \
  -d '{"host": "google.com", "count": 4}'

# 网站测试
curl -X POST http://localhost:8080/api/website \
  -H "Content-Type: application/json" \
  -d '{"url": "https://google.com"}'

# DNS 查询
curl -X POST http://localhost:8080/api/dns \
  -H "Content-Type: application/json" \
  -d '{"domain": "google.com", "query_type": "A"}'

# 健康检查
curl http://localhost:8080/api/health
```

### WebSocket 使用

连接到 `ws://localhost:8080/ws` 可以发送以下格式的消息：

```json
// Ping 测试
{"type": "Ping", "data": {"host": "baidu.com", "count": 4}}

// 网站测试
{"type": "Website", "data": {"url": "https://google.com"}}

// DNS 查询
{"type": "Dns", "data": {"domain": "google.com", "query_type": "A"}}
```

## 性能测试

运行性能基准测试：

```bash
cargo bench
```

测试结果示例：
- Ping 测试：~470ns（本地回环）
- TCPing 测试：~440μs
- 网站测试：~2-8ms
- DNS 查询：~30-70ms
- 并发操作：~500μs

## 项目结构

```
network_probe/
├── src/
│   ├── modules/          # 核心功能模块
│   │   ├── ping.rs      # ICMP ping 实现
│   │   ├── tcping.rs    # TCP 连接测试
│   │   ├── website.rs   # 网站测试
│   │   ├── traceroute.rs # 路由跟踪
│   │   └── dns.rs       # DNS 查询
│   ├── api/             # RESTful API 接口
│   ├── websocket/         # WebSocket 接口
│   ├── cli/              # 命令行接口
│   ├── utils/            # 工具函数
│   └── main.rs           # 主程序入口
├── tests/               # 集成测试
├── benches/             # 性能基准测试
└── Cargo.toml          # 项目配置
```

## 技术栈

- **异步运行时**: Tokio
- **HTTP 框架**: Axum
- **DNS 解析**: trust-dns-resolver
- **ICMP ping**: surge-ping
- **HTTP 客户端**: reqwest
- **命令行解析**: clap
- **序列化**: serde
- **日志**: env_logger

## 优化特性

- **异步 I/O**: 所有网络操作都是异步的
- **连接池**: HTTP 客户端使用连接池复用
- **并发支持**: 支持并发执行多个测试
- **内存优化**: 使用零拷贝和内存池技术
- **错误处理**: 完善的错误处理和恢复机制
- **超时控制**: 所有操作都支持超时设置

## 使用场景

- **网络监控**: 持续监控网络服务可用性
- **性能测试**: 测试网络延迟和吞吐量
- **故障排查**: 诊断网络连接问题
- **安全扫描**: 端口扫描和服务发现
- **DNS 验证**: 验证 DNS 配置和解析

## 许可证

MIT License