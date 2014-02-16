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

type req_info struct {
	keep_alive int
	port       int
	url        string
}

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
	fmt.Printf("-k|--keep <int>\t\tkeep-alive <n> , Default OFF.\n")
	fmt.Printf("--------------------------------\n")
	fmt.Printf("exp: wst -c 1000 -t 10 -k midoks.cachecha.com\n")
	os.Exit(0)
}

func main() {

	var c, t, k, p int
	var _url string

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
				k, _ = strconv.Atoi(arg_option[i+1])
				i++
			case "-p":
				p, _ = strconv.Atoi(arg_option[i+1])
				i++
			case "-v":
				help()
				os.Exit(0)
			default:
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

		_url = os.Args[len(os.Args)-1]
		fmt.Println(c, t, k, p, _url)
	} else {
		fmt.Println("please input you domain, at least!!!")
		help()
	}

	ri := req_info{keep_alive: k, url: _url, port: p}

	runtime.GOMAXPROCS(runtime.NumCPU())
	chan1 := make(chan int64, c)

	var tp int64

	for i := 1; i <= c; i++ {
		go benchmark_go(ri, chan1)
		tp += <-chan1
	}

	fmt.Println("total: ", c, "times")
	ms := tp / 1000000
	fmt.Println("cost: ", ms, "ms")
	fmt.Println("per times: ", ms/int64(c), "ms/times")

}

func benchmark_go(ri req_info, chan1 chan int64) {
	time_begin := time.Now().UnixNano()

	runtime.Gosched()
	benchmark(ri)

	time_end := time.Now().UnixNano()
	time_xh := time_end - time_begin
	//fmt.Println("cost:")
	chan1 <- time_xh
}

func benchmark(ri req_info) {

	conn, err := net.Dial("tcp", ri.url+":"+strconv.Itoa(ri.port)) //监听端口
	defer conn.Close()
	checkError(err)

	str := "GET / HTTP/1.1\r\n"
	str += "Host: " + ri.url + "\r\n"

	if 0 != ri.keep_alive {
		str += "Connection: keep-alive\r\n"
		str += "keep-alive: timeout=" + strconv.Itoa(ri.keep_alive) + ", max=100\r\n"
	}

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
}

func checkError(err error) {
	if err != nil {
		fmt.Fprintf(os.Stderr, "Fatal error: %s", err.Error())
		os.Exit(1)
	}
}
