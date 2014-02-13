package main

import (
	"fmt"
	"log"
	"net/http"
	"time"
)

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		time_begin := time.Now().UnixNano()

		//time.Sleep(time.Second * 1)
		fmt.Fprintf(w, "hello world!!!")
		time_end := time.Now().UnixNano()
		time_xh := time_end - time_begin
		fmt.Println(time_xh)
	})

	time_now := time.Now()
	fmt.Println(time_now)

	// 设置访问的路由
	err := http.ListenAndServe(":8000", nil) //设置监听的端口
	if err != nil {
		log.Fatal("listen and server:", err)
	}
}
