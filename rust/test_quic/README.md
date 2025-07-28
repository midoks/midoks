# Rust QUIC 协议示例

这个项目演示了如何在Rust中使用QUIC协议替代TCP，展示QUIC协议在减少握手延迟方面的优势。

## QUIC协议优势

### 1. 减少握手延迟
- **TCP + TLS**: 需要3次握手(TCP) + 2次握手(TLS) = 5次往返
- **QUIC**: 只需要1次握手就能建立加密连接

### 2. 连接迁移
- 支持网络切换时保持连接（如WiFi切换到移动网络）
- 基于连接ID而非IP地址

### 3. 多路复用
- 原生支持多流，避免队头阻塞
- 每个流独立，一个流的丢包不影响其他流

### 4. 内置加密
- 默认使用TLS 1.3加密
- 更好的安全性和性能

## 项目结构

```
test_quic/
├── Cargo.toml          # 项目配置和依赖
├── src/
│   ├── server.rs       # QUIC服务器
│   ├── client.rs       # QUIC客户端
│   ├── tcp_server.rs   # TCP服务器（对比用）
│   ├── tcp_client.rs   # TCP客户端（对比用）
│   ├── udp_server.rs   # UDP服务器（对比用）
│   └── udp_client.rs   # UDP客户端（对比用）
├── benchmark.sh        # 性能对比测试脚本
├── full_benchmark.sh   # 完整的自动化测试脚本
└── README.md           # 说明文档
```

## 运行示例

### 1. 启动服务器

```bash
cargo run --bin server
```

服务器将在 `127.0.0.1:4433` 上监听QUIC连接。

### 2. 运行客户端

在另一个终端中运行：

```bash
cargo run --bin client
```

客户端将连接到服务器并发送5条测试消息。

### 3. 运行TCP对比测试

为了对比QUIC和TCP的性能差异，项目还包含了TCP实现：

**启动TCP服务器：**
```bash
cargo run --bin tcp_server
```

**运行TCP客户端：**
```bash
cargo run --bin tcp_client
```

### 5. 运行UDP对比测试

UDP作为无连接协议，提供最基础的网络传输性能基准：

**启动UDP服务器：**
```bash
cargo run --bin udp_server
```

**运行UDP客户端：**
```bash
cargo run --bin udp_client
```

### 6. 自动化性能对比

**基础对比脚本：**
```bash
./benchmark.sh
```

**完整测试脚本（推荐）：**
```bash
./full_benchmark.sh
```

完整测试脚本会自动运行所有三种协议的测试，生成详细的对比报告，并保存测试结果到文件中。

## 性能对比

运行示例时，你会看到：

1. **连接建立时间**: QUIC的0-RTT或1-RTT连接建立
2. **往返时间**: 每条消息的发送和接收时间
3. **连接复用**: 在同一连接上发送多条消息

## 依赖说明

- `quinn`: Rust的QUIC实现库
- `tokio`: 异步运行时
- `rustls`: TLS实现
- `rcgen`: 生成自签名证书
- `tracing`: 日志记录

## 注意事项

1. 本示例使用自签名证书，仅用于测试
2. 生产环境应使用有效的TLS证书
3. 防火墙可能需要允许UDP流量（QUIC基于UDP）

## 扩展功能

你可以基于这个示例扩展：

- 文件传输
- 实时通信
- HTTP/3实现
- 负载均衡
- 连接池管理