//go:build !linux && !darwin

package trace

import (
	"fmt"
	"time"
)

func RecordPerf(pid int, duration time.Duration, freq int, outDir string) error {
	return fmt.Errorf("仅支持在 Linux 上运行 eBPF 采样")
}
