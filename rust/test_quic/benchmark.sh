#!/bin/bash

# QUIC vs TCP 性能对比测试脚本

echo "=== QUIC vs TCP 性能对比测试 ==="
echo

# 检查是否已编译
echo "编译项目..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "编译失败！"
    exit 1
fi

echo "编译完成！"
echo

# 测试QUIC性能
echo "=== QUIC 性能测试 ==="
echo "启动QUIC服务器..."
./target/release/server &
SERVER_PID=$!
sleep 2  # 等待服务器启动

echo "运行QUIC客户端..."
./target/release/client

echo "关闭QUIC服务器..."
kill $SERVER_PID
sleep 1

echo
echo "=== TCP 性能测试 ==="
echo "启动TCP服务器..."
./target/release/tcp_server &
TCP_SERVER_PID=$!
sleep 2  # 等待服务器启动

echo "运行TCP客户端..."
./target/release/tcp_client

echo "关闭TCP服务器..."
kill $TCP_SERVER_PID
sleep 1

echo
echo "=== UDP 性能测试 ==="
echo "启动UDP服务器..."
./target/release/udp_server &
UDP_SERVER_PID=$!
sleep 2  # 等待服务器启动

echo "运行UDP客户端..."
./target/release/udp_client

echo "关闭UDP服务器..."
kill $UDP_SERVER_PID

echo
echo "=== 测试完成 ==="
echo "请查看上面的输出来对比QUIC、TCP和UDP的性能差异。"
echo "主要关注点："
echo "1. 连接建立时间（UDP无连接）"
echo "2. 消息往返时间"
echo "3. 协议开销对比"
echo "4. 可靠性 vs 性能权衡"