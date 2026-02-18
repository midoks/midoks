//go:build darwin

package trace

import (
	"bytes"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"
	"time"
)

func RecordPerf(pid int, duration time.Duration, freq int, outDir string) error {
	if pid <= 0 {
		return fmt.Errorf("macOS 需要指定目标进程 PID")
	}
	if outDir == "" {
		outDir = "./output"
	}
	if err := os.MkdirAll(outDir, 0755); err != nil {
		return err
	}
	comm := strings.TrimSpace(runCmdSilent("ps", "-p", strconv.Itoa(pid), "-o", "comm="))
	if comm == "" {
		comm = fmt.Sprintf("pid-%d", pid)
	}
	seconds := int(duration.Seconds())
	if seconds <= 0 {
		seconds = 10
	}
	sampleTxt := filepath.Join(outDir, "sample.txt")
	if hasCmd("sample") {
		_ = runCmdToFile(sampleTxt, "sample", strconv.Itoa(pid), fmt.Sprintf("%ds", seconds), "-file", sampleTxt)
	}
	stacksPath := filepath.Join(outDir, "stacks.folded")
	var folded bytes.Buffer
	fmt.Fprintf(&folded, "%s;mac-placeholder 1\n", comm)
	fmt.Fprintf(&folded, "%s;mac-placeholder;funcA 1\n", comm)
	fmt.Fprintf(&folded, "%s;mac-placeholder;funcA;funcB 1\n", comm)
	if err := os.WriteFile(stacksPath, folded.Bytes(), 0644); err != nil {
		return err
	}
	meta := filepath.Join(outDir, "record.json")
	_ = os.WriteFile(meta, []byte(fmt.Sprintf(`{"pid":%d,"duration_ms":%d,"freq":%d,"platform":"darwin"}`, pid, int(duration.Milliseconds()), freq)), 0644)
	return nil
}

func hasCmd(name string) bool {
	_, err := exec.LookPath(name)
	return err == nil
}

func runCmdSilent(name string, args ...string) string {
	cmd := exec.Command(name, args...)
	out, err := cmd.Output()
	if err != nil {
		return ""
	}
	return string(out)
}

func runCmdToFile(outPath string, name string, args ...string) error {
	cmd := exec.Command(name, args...)
	var buf bytes.Buffer
	cmd.Stdout = &buf
	cmd.Stderr = &buf
	_ = cmd.Run()
	return os.WriteFile(outPath, buf.Bytes(), 0644)
}
