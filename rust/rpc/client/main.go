package main

import (
	"context"
	"fmt"
	"io"
	"log"
	"time"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	pb "grpc-client/proto"
)

func main() {
	// 连接到服务器
	conn, err := grpc.Dial("localhost:50051", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("连接失败: %v", err)
	}
	defer conn.Close()

	// 创建客户端
	client := pb.NewHelloServiceClient(conn)

	// 测试普通RPC调用
	fmt.Println("=== 测试普通RPC调用 ===")
	testSayHello(client)

	// 测试流式RPC调用
	fmt.Println("\n=== 测试流式RPC调用 ===")
	testSayHelloStream(client)
}

func testSayHello(client pb.HelloServiceClient) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()

	req := &pb.HelloRequest{
		Name: "World",
	}

	resp, err := client.SayHello(ctx, req)
	if err != nil {
		log.Fatalf("SayHello调用失败: %v", err)
	}

	fmt.Printf("服务器响应: %s\n", resp.GetMessage())
}

func testSayHelloStream(client pb.HelloServiceClient) {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	req := &pb.HelloRequest{
		Name: "Stream User",
	}

	stream, err := client.SayHelloStream(ctx, req)
	if err != nil {
		log.Fatalf("SayHelloStream调用失败: %v", err)
	}

	for {
		resp, err := stream.Recv()
		if err == io.EOF {
			fmt.Println("流结束")
			break
		}
		if err != nil {
			log.Fatalf("接收流数据失败: %v", err)
		}

		fmt.Printf("流响应: %s\n", resp.GetMessage())
	}
}