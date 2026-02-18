package main

import (
	"flag"
	"fmt"
	"os"
	"time"

	"github.com/midoks/gtrace/internal/flamegraph"
	"github.com/midoks/gtrace/internal/metrics"
	"github.com/midoks/gtrace/internal/trace"
)

type options struct {
	pid      int
	duration time.Duration
	freq     int
	outDir   string
	mode     string
}

func parseOptions() options {
	var opt options
	flag.IntVar(&opt.pid, "pid", 0, "目标进程 PID（0 表示全局采样）")
	flag.DurationVar(&opt.duration, "duration", time.Minute, "采样时长，例如 30s、2m")
	flag.IntVar(&opt.freq, "freq", 99, "CPU 采样频率（Hz）")
	flag.StringVar(&opt.outDir, "out", "./output", "结果输出目录")
	flag.StringVar(&opt.mode, "mode", "record", "模式：record|flamegraph|top|mem|io")
	flag.Parse()
	return opt
}

func main() {
	opt := parseOptions()
	if err := run(opt); err != nil {
		fmt.Fprintf(os.Stderr, "错误: %v\n", err)
		os.Exit(1)
	}
}

func run(opt options) error {
	switch opt.mode {
	case "record":
		return runRecord(opt)
	case "flamegraph":
		return runFlamegraph(opt)
	case "top":
		return runTop(opt)
	case "mem":
		return runMem(opt)
	case "io":
		return runIO(opt)
	default:
		return fmt.Errorf("未知模式: %s", opt.mode)
	}
}

func runRecord(opt options) error {
	return trace.RecordPerf(opt.pid, opt.duration, opt.freq, opt.outDir)
}

func runFlamegraph(opt options) error {
	return flamegraph.Generate(opt.outDir)
}

func runTop(opt options) error {
	return metrics.PrintTopCPU(opt.outDir)
}

func runMem(opt options) error {
	return metrics.PrintMemStats(opt.outDir)
}

func runIO(opt options) error {
	return metrics.PrintIOStats(opt.outDir)
}
