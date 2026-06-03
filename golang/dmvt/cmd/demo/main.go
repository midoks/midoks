package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"time"

	"github.com/midoks/dmvt/splitter"
)

func main() {
	var (
		input          = flag.String("i", "", "输入视频文件路径")
		outputDir      = flag.String("o", "output", "输出目录")
		splitterType   = flag.String("t", "ffmpeg", "分片器类型: ffmpeg, mp4ff, mmap")
		segmentDuration = flag.Float64("d", 10.0, "分片时长 (秒)")
		showComparison = flag.Bool("compare", false, "显示分片器对比表")
	)
	flag.Parse()

	// 显示对比表
	if *showComparison {
		splitter.PrintSplitterComparison()
		return
	}

	// 验证输入
	if *input == "" {
		fmt.Println("用法: go run cmd/demo/main.go -i <输入视频> [选项]")
		fmt.Println("\n选项:")
		flag.PrintDefaults()
		fmt.Println("\n示例:")
		fmt.Println("  go run cmd/demo/main.go -i video.mp4 -t ffmpeg -d 10")
		fmt.Println("  go run cmd/demo/main.go -i video.mp4 -t mp4ff -d 10")
		fmt.Println("  go run cmd/demo/main.go -i video.mp4 -t mmap -d 10")
		fmt.Println("  go run cmd/demo/main.go -compare")
		os.Exit(1)
	}

	// 检查输入文件是否存在
	if _, err := os.Stat(*input); os.IsNotExist(err) {
		log.Fatalf("输入文件不存在: %s", *input)
	}

	// 确定分片器类型
	var st splitter.SplitterType
	switch *splitterType {
	case "ffmpeg":
		st = splitter.SplitterTypeFFmpegCLI
	case "mp4ff":
		st = splitter.SplitterTypeMp4ff
	case "mmap":
		st = splitter.SplitterTypeMmap
	default:
		log.Fatalf("未知的分片器类型: %s", *splitterType)
	}

	// 创建分片器
	s, err := splitter.CreateSplitter(st)
	if err != nil {
		log.Fatalf("创建分片器失败: %v", err)
	}

	// 执行分片
	fmt.Printf("使用 %s 分片器分片视频: %s\n", *splitterType, *input)
	fmt.Printf("分片时长: %.1f 秒\n", *segmentDuration)
	fmt.Printf("输出目录: %s\n", *outputDir)
	fmt.Println("开始分片...")

	startTime := time.Now()
	outputs, err := s.SplitToMultiple(*input, *outputDir, *segmentDuration)
	if err != nil {
		log.Fatalf("分片失败: %v", err)
	}

	elapsed := time.Since(startTime)
	fmt.Printf("\n分片完成! 共生成 %d 个片段\n", len(outputs))
	fmt.Printf("总耗时: %v\n", elapsed)
	if len(outputs) > 0 {
		fmt.Printf("平均每片耗时: %.2f ms\n", float64(elapsed.Milliseconds())/float64(len(outputs)))
	}

	// 显示分片器信息
	info := splitter.GetSplitterInfo(st)
	fmt.Printf("\n分片器特性:\n")
	fmt.Printf("  - 内存峰值: %d MB\n", info.MemoryPeak/(1024*1024))
	fmt.Printf("  - 最小粒度: %s\n", info.MinGranularity)
	fmt.Printf("  - 支持热切: %v\n", info.HotCutSupport)
}

// benchmark 性能测试
func benchmark(input string) {
	fmt.Println("开始性能测试...")

	types := []struct {
		name string
		st   splitter.SplitterType
	}{
		{"FFmpeg CLI", splitter.SplitterTypeFFmpegCLI},
		{"mp4ff 索引", splitter.SplitterTypeMp4ff},
		{"mmap 分片", splitter.SplitterTypeMmap},
	}

	for _, t := range types {
		s, err := splitter.CreateSplitter(t.st)
		if err != nil {
			log.Printf("创建 %s 分片器失败: %v", t.name, err)
			continue
		}

		outputDir := fmt.Sprintf("benchmark_%s", t.name)
		os.RemoveAll(outputDir)

		start := time.Now()
		outputs, err := s.SplitToMultiple(input, outputDir, 10.0)
		elapsed := time.Since(start)

		if err != nil {
			log.Printf("%s 分片失败: %v", t.name, err)
			continue
		}

		avgTime := float64(elapsed.Milliseconds()) / float64(len(outputs))
		fmt.Printf("%s: %d 个片段, 总耗时 %v, 平均 %.2f ms/片\n",
			t.name, len(outputs), elapsed, avgTime)

		os.RemoveAll(outputDir)
	}
}