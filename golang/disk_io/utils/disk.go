package utils

import (
	"fmt"
	"os"
	"time"
)

type DiskWriteResult struct {
	FileSize       int64
	Duration       time.Duration
	AvgSpeedMBps   float64
	MaxSpeedMBps   float64
	ChunkSize      int
	Success        bool
	Error          error
}

func EstimateMaxWriteSpeed(fileSize int, chunkSize int) DiskWriteResult {
	result := DiskWriteResult{
		FileSize:  int64(fileSize),
		ChunkSize: chunkSize,
	}

	filePath := "disk_speed_test.tmp"

	data := make([]byte, chunkSize)
	for i := range data {
		data[i] = byte(i % 256)
	}

	start := time.Now()

	file, err := os.Create(filePath)
	if err != nil {
		result.Error = err
		return result
	}

	written := 0
	for written < fileSize {
		n, err := file.Write(data)
		if err != nil {
			file.Close()
			os.Remove(filePath)
			result.Error = err
			return result
		}
		written += n
	}

	err = file.Sync()
	if err != nil {
		file.Close()
		os.Remove(filePath)
		result.Error = err
		return result
	}

	file.Close()
	os.Remove(filePath)

	duration := time.Since(start)
	result.Duration = duration
	result.Success = true

	result.AvgSpeedMBps = float64(fileSize) / duration.Seconds() / (1024 * 1024)
	result.MaxSpeedMBps = result.AvgSpeedMBps * 1.1

	return result
}

func (r *DiskWriteResult) PrintResult() {
	fmt.Printf("总写入量: %.2f GB\n", float64(r.FileSize)/(1024*1024*1024))
	fmt.Printf("块大小: %.2f MB\n", float64(r.ChunkSize)/(1024*1024))
	fmt.Printf("总耗时: %.2f 秒\n", r.Duration.Seconds())
	fmt.Printf("平均写入速度: %.2f MB/s\n", r.AvgSpeedMBps)
	fmt.Printf("预估最大写入速度: %.2f MB/s\n", r.MaxSpeedMBps)
}

func EstimateMaxWriteSpeedSimple() float64 {
	result := EstimateMaxWriteSpeed(1024*1024*1024, 4*1024*1024)
	if !result.Success {
		fmt.Printf("测试失败: %v\n", result.Error)
		return 0
	}
	return result.MaxSpeedMBps
}
