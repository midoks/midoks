package main

import (
	"context"
	"log"
	"time"

	pb "go_rpc_client/proto"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

const (
	address = "localhost:50051"
)

func main() {
	// 建立连接
	conn, err := grpc.Dial(address, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()
	c := pb.NewGreeterClient(conn)

	// 普通RPC调用
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	
	r, err := c.SayHello(ctx, &pb.HelloRequest{Name: "Go Client"})
	if err != nil {
		log.Fatalf("could not greet: %v", err)
	}
	log.Printf("Greeting: %s", r.GetMessage())

	// 流式RPC调用
	stream, err := c.SayHelloStream(context.Background(), &pb.HelloRequest{Name: "Streaming Go Client"})
	if err != nil {
		log.Fatalf("could not start stream: %v", err)
	}

	for {
		reply, err := stream.Recv()
		if err != nil {
			log.Printf("stream ended: %v", err)
			break
		}
		log.Printf("Stream reply: %s", reply.GetMessage())
	}
}