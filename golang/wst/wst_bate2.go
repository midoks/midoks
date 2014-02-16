package main

import (
	"bytes"
	"fmt"
	"net"
	//"net/http"
	"os"
	"strconv"
	"time"
	//"strings"
	"runtime"
)

func help() {
	fmt.Printf("--------------------------------\n")
	fmt.Printf("Web Server Test\n")
	fmt.Printf("Date: 2014-2-15\n")
	fmt.Printf("Author: midoks@163.com\n")
	fmt.Printf("--------------------------------\n")
	fmt.Printf("-c|--clients <n>\tRun <n> HTTP clients at once. Default one.\n")
	fmt.Printf("-t|--time <sec>\t\tRun benchmark for <sec> seconds. Default 30.\n")
	fmt.Printf("-v|--version\t\tDisplay program version.\n")
	fmt.Printf("-p|--port <port>\tserver port <int> port, Default 80.\n")
	fmt.Printf("-k|--keep <keep-alive>\t\tkeep-alive <n> , Default off.\n")
	fmt.Printf("--------------------------------\n")
	fmt.Printf("exp: wst -c 1000 -t 10 -k midoks.cachecha.com\n")
	os.Exit(0)
}

func main() {

	var c, t, k, p int
	var url string

	if len(os.Args) > 1 {
		arg_option := os.Args[1:]
		for i := 0; i < len(arg_option); i++ {
			switch arg_option[i] {
			case "-c":
				c, _ = strconv.Atoi(arg_option[i+1])
				i++

			case "-t":
				t, _ = strconv.Atoi(arg_option[i+1])
				i++
			case "-k":
				k = 1
			case "-p":
				p, _ = strconv.Atoi(arg_option[i+1])
				i++
			case "-v":
				help()
				os.Exit(0)
			default:
				//help()
				//i++
				//os.Exit(0)
			}
		}

		if 0 == c {
			c = 1
		}

		if 0 == t {
			t = 1
		}

		if 0 == p {
			p = 80
		}

		url = os.Args[len(os.Args)-1]
		fmt.Println(c, t, k, p, url)
	} else {
		fmt.Println("please input you domain, at least!!!")
		help()
	}

	runtime.GOMAXPROCS(runtime.NumCPU())

	chan1 := make(chan int, c)

	time_begin := time.Now().UnixNano()
	go func() {

		for i := 1; i <= c; i++ {

			benchmark(url, chan1)
		}

		chan1 <- 0
		//<-chan1
	}()
	<-chan1
	//chan1 <- 0

	time_end := time.Now().UnixNano()
	time_xh := time_end - time_begin
	fmt.Println("total: ", c, "times")
	ms := time_xh / 1000000
	fmt.Println("cost: ", ms, "ms")
	fmt.Println("per times: ", ms/int64(c), "ms/times")

}

func benchmark(url string, chan1 chan int) {
	//url := "midoks.cachecha.com"
	//监听端口
	conn, err := net.Dial("tcp", url+":80")
	defer conn.Close()
	checkError(err)

	str := "GET / HTTP/1.1\r\n"
	str += "Host: " + url + "\r\n"
	str += "Connection: keep-alive\r\n"
	str += "User-Agent: wst/1.0\r\n"
	//str += "Referer: www.qq.com\r\n"
	str += "\r\n"

	_, err = conn.Write([]byte(str))
	checkError(err)

	buf := make([]byte, 51200)
	result := bytes.NewBuffer(nil)
	n, err := conn.Read(buf[0:])
	checkError(err)

	result.Write(buf[0:n])
	checkError(err)
	buf = nil

	//fmt.Println(n, string(result.Bytes()))
	//chan1 <- 0
}

func checkError(err error) {
	if err != nil {
		fmt.Fprintf(os.Stderr, "Fatal error: %s", err.Error())
		os.Exit(1)
	}
}
