#!/bin/bash

# 完整的网络协议性能对比测试脚本
# 测试 QUIC、TCP、UDP 三种协议的性能

echo "======================================"
echo "    网络协议性能对比测试工具"
echo "    测试协议: QUIC vs TCP vs UDP vs HTTPS"
echo "======================================"
echo

# 检查是否已编译
echo "[1/4] 编译项目..."
cargo build --release --quiet
if [ $? -ne 0 ]; then
    echo "❌ 编译失败！"
    exit 1
fi
echo "✅ 编译完成"
echo

# 创建结果文件
RESULT_FILE="benchmark_results_$(date +%Y%m%d_%H%M%S).txt"
echo "[2/4] 创建测试结果文件: $RESULT_FILE"
echo "网络协议性能测试结果 - $(date)" > $RESULT_FILE
echo "=========================================" >> $RESULT_FILE
echo >> $RESULT_FILE

# 测试QUIC性能
echo "[3/4] 开始性能测试..."
echo
echo "🚀 测试 1/4: QUIC 协议性能"
echo "QUIC 协议测试结果:" >> $RESULT_FILE
echo "启动QUIC服务器..."
./target/release/server &
QUIC_PID=$!
sleep 3

echo "运行QUIC客户端..."
./target/release/client 2>&1 | tee -a $RESULT_FILE
echo >> $RESULT_FILE

echo "关闭QUIC服务器..."
kill $QUIC_PID 2>/dev/null
sleep 2

# 测试TCP性能
echo
echo "🔗 测试 2/4: TCP 协议性能"
echo "TCP 协议测试结果:" >> $RESULT_FILE
echo "启动TCP服务器..."
./target/release/tcp_server &
TCP_PID=$!
sleep 2

echo "运行TCP客户端..."
./target/release/tcp_client 2>&1 | tee -a $RESULT_FILE
echo >> $RESULT_FILE

echo "关闭TCP服务器..."
kill $TCP_PID 2>/dev/null
sleep 2

# 测试UDP性能
echo
echo "📡 测试 3/4: UDP 协议性能"
echo "UDP 协议测试结果:" >> $RESULT_FILE
echo "启动UDP服务器..."
./target/release/udp_server &
UDP_PID=$!
sleep 2

echo "运行UDP客户端..."
./target/release/udp_client 2>&1 | tee -a $RESULT_FILE
echo >> $RESULT_FILE

echo "关闭UDP服务器..."
kill $UDP_PID 2>/dev/null
sleep 1

# 测试HTTPS性能
echo
echo "🔒 测试 4/4: HTTPS 协议性能"
echo "HTTPS 协议测试结果:" >> $RESULT_FILE
echo "启动HTTPS服务器..."
./target/release/https_server &
HTTPS_PID=$!
sleep 2

echo "运行HTTPS客户端..."
./target/release/https_client 2>&1 | tee -a $RESULT_FILE
echo >> $RESULT_FILE

echo "关闭HTTPS服务器..."
kill $HTTPS_PID 2>/dev/null

# 生成总结
echo "[5/5] 生成测试总结..."
echo
echo "测试总结:" >> $RESULT_FILE
echo "1. QUIC: 适合现代Web应用，支持多路复用和连接迁移" >> $RESULT_FILE
echo "2. TCP:  传统可靠协议，适合大多数应用场景" >> $RESULT_FILE
echo "3. UDP:  最低延迟，适合实时应用但需要应用层可靠性" >> $RESULT_FILE
echo "4. HTTPS: 安全的HTTP协议，适合需要加密的Web应用" >> $RESULT_FILE
echo >> $RESULT_FILE
echo "详细分析请参考 test_results.md 文件" >> $RESULT_FILE

echo "======================================"
echo "           测试完成！"
echo "======================================"
echo
echo "📊 测试结果已保存到: $RESULT_FILE"
echo "📋 详细分析文档: test_results.md"
echo "🔧 项目说明文档: README.md"
echo
echo "主要发现:"
echo "• UDP: 最低延迟，无连接开销"
echo "• TCP: 可靠传输，成熟稳定"
echo "• QUIC: 现代协议，适合复杂网络环境"
echo "• HTTPS: 安全传输，双重握手开销"
echo
echo "建议: 根据应用需求选择合适的协议"
echo "- 实时游戏/视频: UDP"
echo "- 传统Web应用: TCP"
echo "- 现代Web/移动应用: QUIC"
echo "- 安全Web应用: HTTPS"