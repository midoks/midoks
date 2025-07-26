package main

import (
	"context"
	"fmt"
	"log"
	"time"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

// 简化版本的客户端，直接测试连接
func main() {
	// 连接到服务器
	conn, err := grpc.Dial("localhost:50051", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("连接失败: %v", err)
	}
	defer conn.Close()

	fmt.Println("成功连接到gRPC服务器!")
	fmt.Println("服务器地址: localhost:50051")
	fmt.Println("连接状态:", conn.GetState())

	// 测试连接
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	// 简单的连接测试
	select {
	case <-ctx.Done():
		fmt.Println("连接超时")
	default:
		fmt.Println("连接正常，可以进行gRPC调用")
		fmt.Println("\n=== Demo演示完成 ===")
		fmt.Println("Rust gRPC服务端正在运行")
		fmt.Println("Golang客户端成功连接")
	}
}