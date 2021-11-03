package main

import (
	"fmt"
	"runtime" //执行并行段时需要引入该包
	"time"
)

const (
	NUM = 8
)

type vint struct {
	n []int
}

func (v vint) Doadd(p, i, n int, u []int, c chan int) {
	for ti := i; ti < n; ti++ {
		v.n[p] += u[ti]
		time.Sleep(1 * time.Second)
	}
	c <- 1
	return
}
func (v vint) Doall(ncpu int, u []int) (sum int) {
	c := make(chan int, ncpu)
	var segment int
	if NUM%ncpu == 0 {
		segment = NUM / ncpu
	} else {
		segment = NUM / (ncpu - 1)
	}

	for i := 0; i < ncpu; i++ {
		start := i * segment
		temp := start + segment
		var end int
		if temp < NUM {
			end = temp
		} else {
			end = NUM
		}
		go v.Doadd(i, start, end, u, c)
	}
	for i := 0; i < ncpu; i++ {
		<-c
	}
	for i := 0; i < ncpu; i++ {
		sum += v.n[i]
	}
	return
}

func NcpuCalc() {
	ncpu := runtime.NumCPU()
	fmt.Println(ncpu)
	runtime.GOMAXPROCS(ncpu)
	u := make([]int, NUM)
	for i := 0; i < NUM; i++ {
		u[i] = 1
	}
	v := new(vint)
	v.n = make([]int, ncpu)

	ts := time.Now().UnixNano()
	sum := v.Doall(ncpu, u)
	te := time.Now().UnixNano()
	fmt.Println((te-ts)/int64(time.Second), "s", sum)
}

func OcupCalc() {
	u := make([]int, NUM)
	for i := 0; i < NUM; i++ {
		u[i] = 1
	}
	ts := time.Now().UnixNano()
	sum := 0
	for i := 0; i < NUM; i++ {
		sum += u[i]
		time.Sleep(1 * time.Second)
	}
	te := time.Now().UnixNano()
	fmt.Println((te-ts)/int64(time.Second), "s")
}

func main() {

	//串行段
	OcupCalc()

	//并行段
	NcpuCalc()

	return
}
