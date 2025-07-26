# gRPC Rust服务端 + Golang客户端 Demo

这是一个演示如何使用Rust实现gRPC服务端，Golang实现gRPC客户端的项目。

## 项目结构

```
.
├── proto/
│   └── hello.proto          # Protocol Buffers定义文件
├── server/                  # Rust gRPC服务端
│   ├── Cargo.toml
│   ├── build.rs
│   └── src/
│       └── main.rs
├── client/                  # Golang gRPC客户端
│   ├── go.mod
│   ├── main.go
│   ├── generate_proto.sh
│   └── proto/
│       ├── hello.pb.go
│       └── hello_grpc.pb.go
└── README.md
```

## 功能特性

- **普通RPC调用**: 客户端发送请求，服务端返回单个响应
- **流式RPC调用**: 服务端返回多个响应流

## 运行步骤

### 1. 启动Rust服务端

```bash
cd server
cargo run
```

服务端将在 `localhost:50051` 上启动。

### 2. 运行Golang客户端

首先需要下载依赖：

```bash
cd client
go mod tidy
```

然后运行客户端：

```bash
go run main.go
```

## 预期输出

### 服务端输出
```
HelloService Server listening on [::1]:50051
Got a request: Request { metadata: MetadataMap { headers: {"content-type": "application/grpc", "user-agent": "grpc-go/1.59.0", "te": "trailers"} }, message: HelloRequest { name: "World" }, extensions: Extensions }
Got a stream request: Request { metadata: MetadataMap { headers: {"content-type": "application/grpc", "user-agent": "grpc-go/1.59.0", "te": "trailers"} }, message: HelloRequest { name: "Stream User" }, extensions: Extensions }
```

### 客户端输出
```
=== 测试普通RPC调用 ===
服务器响应: Hello World!

=== 测试流式RPC调用 ===
流响应: Hello Stream User - Message 1
流响应: Hello Stream User - Message 2
流响应: Hello Stream User - Message 3
流响应: Hello Stream User - Message 4
流响应: Hello Stream User - Message 5
流结束
```

## 技术栈

- **服务端**: Rust + Tonic (gRPC框架)
- **客户端**: Golang + google.golang.org/grpc
- **协议**: Protocol Buffers 3

## 注意事项

1. 确保系统已安装Rust和Go环境
2. 如需重新生成protobuf代码，可以使用 `client/generate_proto.sh` 脚本
3. 服务端默认监听IPv6地址，如有问题可修改为IPv4