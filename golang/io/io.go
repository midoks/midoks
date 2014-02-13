package main

import (
	"fmt"
	"io/ioutil"
	"os"
)

func write(fn string, value string) (err error) {
	file, err := os.Create(fn)
	if err != nil {
		fmt.Println("Create File Fail: ", err.Error())
		return err
	}
	defer file.Close()
	file.WriteString(value)
	return nil
}

func read(fn string) (str string, err error) {
	file, err := os.OpenFile(fn, 1, 1)
	if err != nil {
		fmt.Println("Open File Fail: ", err.Error())
		return fn, err
	}
	defer file.Close()

	buf := make([]byte, 1024)
	v, _ := file.Read(buf[0:])
	os.Stdout.Write(buf[:v])
	return string(str), err
}

func read2(fn string) (str string, err error) {
	bytes, err := ioutil.ReadFile(fn)
	if err != nil {
		return
	}
	x := string(bytes)
	return x, err
}

func main() {
	fn := "n.txt"
	value := "你好啊,世界!!!"

	_ = write(fn, value)

	v, _ := read2(fn)
	fmt.Println(v)
	fmt.Println("hello world!!")
}
