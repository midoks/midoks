#!/bin/bash

# 创建proto目录
mkdir -p proto

# 生成Go代码
protoc --go_out=. --go_opt=paths=source_relative \
    --go-grpc_out=. --go-grpc_opt=paths=source_relative \
    ../proto/hello.proto

# 移动生成的文件到proto目录
mv hello.pb.go proto/
mv hello_grpc.pb.go proto/

echo "Proto文件生成完成！"