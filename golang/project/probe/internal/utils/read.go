package tools

import (
	"io"
	"os"
)

func ReverseRead(name string, lineNum uint) ([]string, error) {
	// 打开文件
	file, err := os.Open(name)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	// 获取文件大小
	fs, err := file.Stat()
	if err != nil {
		return nil, err
	}
	fileSize := fs.Size()

	var offset int64 = -1   // 偏移量，初始化为-1，若为0则会读到EOF
	char := make([]byte, 1) // 用于读取单个字节
	lineStr := ""           // 存放一行的数据
	buff := make([]string, 0, 100)
	for (-offset) <= fileSize {
		// 通过Seek函数从末尾移动游标然后每次读取一个字节
		file.Seek(offset, io.SeekEnd)
		_, err := file.Read(char)
		if err != nil {
			return buff, err
		}
		if char[0] == '\n' {
			offset--  // windows跳过'\r'
			lineNum-- // 到此读取完一行
			buff = append(buff, lineStr)
			lineStr = ""
			if lineNum == 0 {
				return buff, nil
			}
		} else {
			lineStr = string(char) + lineStr
		}
		offset--
	}
	buff = append(buff, lineStr)
	return buff, nil
}
