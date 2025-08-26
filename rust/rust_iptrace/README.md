# Rust IP Trace

一个用Rust实现的IP追踪工具，类似于traceroute命令。

## 功能特性

- 🔍 IP地址和主机名追踪
- 🌍 地理位置信息显示
- ⚡ 异步网络操作
- 🎨 彩色输出
- ⚙️ 可配置的超时和最大跳数

## 安装和使用

### 编译

```bash
cargo build --release
```

### 使用方法

```bash
# 基本用法
cargo run -- google.com

# 显示地理位置信息
cargo run -- -g google.com

# 设置最大跳数
cargo run -- -m 20 google.com

# 设置超时时间
cargo run -- -t 3 google.com
```

### 命令行参数

- `target`: 目标IP地址或主机名（必需）
- `-m, --max-hops`: 最大跳数（默认：30）
- `-t, --timeout`: 每跳超时时间（秒，默认：5）
- `-g, --geo`: 显示地理位置信息

## 依赖说明

- `pnet`: 网络数据包处理
- `tokio`: 异步运行时
- `clap`: 命令行参数解析
- `reqwest`: HTTP客户端（用于地理位置查询）
- `colored`: 彩色终端输出

## 注意事项

- 需要管理员权限来发送原始ICMP数据包
- 某些防火墙可能会阻止ICMP数据包
- 地理位置信息依赖于第三方API服务

## 许可证

MIT License