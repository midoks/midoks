/**
 * 这是一个简单Web服务实现
 */
package main

import (
	"fmt"
	"log"
	"net/http"
)

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "ok!!!")
		fmt.Println("ok!")
	}) // 设置访问的路由
	err := http.ListenAndServe(":8000", nil) //设置监听的端口

	if err != nil {
		log.Fatal("listen and server:", err)
	}
}
