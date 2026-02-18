//go:build !linux && !darwin

package metrics

import (
	"fmt"
)

func PrintTopCPU(outDir string) error {
	return fmt.Errorf("仅在 Linux 上可用（需要采样结果）")
}

func PrintMemStats(outDir string) error {
	return fmt.Errorf("仅在 Linux 上可用（需要采样结果）")
}

func PrintIOStats(outDir string) error {
	return fmt.Errorf("仅在 Linux 上可用（需要采样结果）")
}
