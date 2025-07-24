# Pingora 代理测试示例

这个项目是基于 Cloudflare 的 [Pingora](https://github.com/cloudflare/pingora) 框架实现的代理服务器示例。Pingora 是一个用 Rust 编写的高性能 HTTP 代理框架，本项目展示了如何使用它构建不同类型的代理服务器。

## 功能特性

本项目实现了三种不同类型的代理服务器：

1. **简单代理服务器**：将所有请求转发到指定的上游服务器
2. **负载均衡代理服务器**：使用轮询算法将请求分发到多个上游服务器
3. **缓存代理服务器**：缓存上游服务器的响应，减少重复请求

## 安装与使用

### 前提条件

- Rust 和 Cargo (推荐使用 [rustup](https://rustup.rs/) 安装)

### 构建项目

```bash
cargo build --release
```

### 运行不同类型的代理服务器

#### 简单代理服务器 (默认，监听端口 8080)

```bash
cargo run --release
# 或者
cargo run --release -- simple
```

#### 负载均衡代理服务器 (监听端口 8081)

```bash
cargo run --release -- load-balancer
```

#### 缓存代理服务器 (监听端口 8082)

```bash
cargo run --release -- cache
```

## 测试代理服务器

可以使用 curl 命令测试代理服务器：

```bash
# 测试简单代理服务器
curl -v http://localhost:8080/

# 测试负载均衡代理服务器
curl -v http://localhost:8081/

# 测试缓存代理服务器
curl -v http://localhost:8082/
curl -v -H "Host: example.com" http://127.0.0.1:8082/ 
```

## 项目结构

- `src/main.rs`: 主程序入口，包含命令行参数解析和简单代理服务器实现
- `src/load_balancer.rs`: 负载均衡代理服务器实现
- `src/cache_proxy.rs`: 缓存代理服务器实现
- `src/lib.rs`: 导出模块和函数

## 自定义配置

可以修改源代码中的上游服务器地址、缓存时间等参数来自定义代理服务器的行为。

## 学习资源

- [Pingora 官方文档](https://github.com/cloudflare/pingora)
- [Rust 异步编程](https://rust-lang.github.io/async-book/)

## 许可证

MIT