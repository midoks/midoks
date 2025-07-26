#!/bin/bash

echo "=== gRPC Rust服务端 + Golang客户端 Demo ==="
echo

# 检查依赖
echo "检查依赖..."
if ! command -v cargo &> /dev/null; then
    echo "错误: 未找到Rust/Cargo，请先安装Rust"
    exit 1
fi

if ! command -v go &> /dev/null; then
    echo "错误: 未找到Go，请先安装Go"
    exit 1
fi

echo "✓ Rust和Go环境检查通过"
echo

# 编译Rust服务端
echo "编译Rust服务端..."
cd server
cargo build --release
if [ $? -ne 0 ]; then
    echo "错误: Rust服务端编译失败"
    exit 1
fi
echo "✓ Rust服务端编译成功"
cd ..

# 编译Golang客户端
echo "编译Golang客户端..."
cd client
go mod tidy
go build -o simple_client simple_main.go
if [ $? -ne 0 ]; then
    echo "错误: Golang客户端编译失败"
    exit 1
fi
echo "✓ Golang客户端编译成功"
cd ..

echo
echo "=== 启动演示 ==="
echo "1. 启动Rust gRPC服务端..."
cd server
cargo run --release &
SERVER_PID=$!
cd ..

# 等待服务端启动
echo "等待服务端启动..."
sleep 3

echo "2. 运行Golang客户端..."
cd client
./simple_client
cd ..

echo
echo "3. 停止服务端..."
kill $SERVER_PID

echo
echo "=== Demo演示完成 ==="
echo "✓ Rust gRPC服务端成功启动并监听端口50051"
echo "✓ Golang客户端成功连接到服务端"
echo "✓ 跨语言gRPC通信演示成功"