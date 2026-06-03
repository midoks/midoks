package main

import (
	"disk_io/utils"
	"fmt"
)

func main() {
	fmt.Println("开始测试硬盘写入速度...")

	result := utils.EstimateMaxWriteSpeed(1024*1024*1024, 4*1024*1024)
	if !result.Success {
		fmt.Printf("测试失败: %v\n", result.Error)
		return
	}

	fmt.Println("\n测试完成!")
	result.PrintResult()
}
