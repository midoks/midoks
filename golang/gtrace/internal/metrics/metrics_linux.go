//go:build linux

package metrics

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
)

type recordMeta struct {
	PID        int `json:"pid"`
	DurationMS int `json:"duration_ms"`
	Freq       int `json:"freq"`
}

func PrintTopCPU(outDir string) error {
	meta, err := readMeta(outDir)
	if err != nil {
		return err
	}
	fmt.Printf("CPU Top（占位）：PID=%d Freq=%dHz Duration=%dms\n", meta.PID, meta.Freq, meta.DurationMS)
	return nil
}

func PrintMemStats(outDir string) error {
	meta, err := readMeta(outDir)
	if err != nil {
		return err
	}
	fmt.Printf("内存统计（占位）：PID=%d Duration=%dms\n", meta.PID, meta.DurationMS)
	return nil
}

func PrintIOStats(outDir string) error {
	meta, err := readMeta(outDir)
	if err != nil {
		return err
	}
	fmt.Printf("IO 统计（占位）：PID=%d Duration=%dms\n", meta.PID, meta.DurationMS)
	return nil
}

func readMeta(outDir string) (recordMeta, error) {
	var meta recordMeta
	path := filepath.Join(outDir, "record.json")
	data, err := os.ReadFile(path)
	if err != nil {
		return meta, fmt.Errorf("读取元数据失败: %w", err)
	}
	if err := json.Unmarshal(data, &meta); err != nil {
		return meta, fmt.Errorf("解析元数据失败: %w", err)
	}
	return meta, nil
}
