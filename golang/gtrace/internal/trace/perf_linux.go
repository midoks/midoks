//go:build linux

package trace

import (
	"fmt"
	"os"
	"path/filepath"
	"time"
)

func RecordPerf(pid int, duration time.Duration, freq int, outDir string) error {
	if outDir == "" {
		outDir = "./output"
	}
	if err := os.MkdirAll(outDir, 0755); err != nil {
		return err
	}
	stacks := filepath.Join(outDir, "stacks.folded")
	f, err := os.Create(stacks)
	if err != nil {
		return err
	}
	defer f.Close()
	fmt.Fprintf(f, "process-%d;sample-placeholder 1\n", pid)
	fmt.Fprintf(f, "process-%d;sample-placeholder;funcA 1\n", pid)
	fmt.Fprintf(f, "process-%d;sample-placeholder;funcA;funcB 1\n", pid)
	meta := filepath.Join(outDir, "record.json")
	_ = os.WriteFile(meta, []byte(fmt.Sprintf(`{"pid":%d,"duration_ms":%d,"freq":%d}`, pid, int(duration.Milliseconds()), freq)), 0644)
	return nil
}
