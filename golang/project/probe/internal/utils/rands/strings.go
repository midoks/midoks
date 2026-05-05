package rands

import (
	"math/rand"
	"sync"
	"time"
)

var (
	locker = &sync.Mutex{}
	source = rand.New(rand.NewSource(time.Now().UnixNano()))
)

const (
	hexChars          = "0123456789abcdef"
	hexCharsLength    = len(hexChars)
	letterChars       = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
	letterCharsLength = len(letterChars)
)

// 获取随机字符串
func String(n int) string {
	if n <= 0 {
		return ""
	}
	b := make([]byte, n)
	locker.Lock()
	for i := 0; i < n; i++ {
		b[i] = letterChars[source.Int63()%int64(letterCharsLength)]
	}
	locker.Unlock()
	return string(b)
}

// 获取随机的一个16进制的字符串，即返回的字符串中只包含[0-9a-f]字符
func HexString(n int) string {
	if n <= 0 {
		return ""
	}
	b := make([]byte, n)
	locker.Lock()
	for i := 0; i < n; i++ {
		b[i] = hexChars[source.Int63()%int64(hexCharsLength)]
	}
	locker.Unlock()
	return string(b)
}
