package main

import (
	"bytes"
	"fmt"
	"net"
	"os"
)

func main() {

	//监听端口
	conn, err := net.Dial("tcp", "127.0.0.1:80")
	defer conn.Close()
	checkError(err)

	_, err = conn.Write([]byte("GET / HTTP/1.0\r\n\r\n"))
	checkError(err)

	var buf [51200]byte
	result := bytes.NewBuffer(nil)
	n, err := conn.Read(buf[0:])
	checkError(err)

	result.Write(buf[0:n])
	checkError(err)

	fmt.Println(n, string(result.Bytes()))
	fmt.Println("ok")
}

func checkError(err error) {
	if err != nil {
		fmt.Fprintf(os.Stderr, "Fatal error: %s", err.Error())
		os.Exit(1)
	}
}
