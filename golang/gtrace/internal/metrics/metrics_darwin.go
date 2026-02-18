//go:build darwin

package metrics

import (
	"bufio"
	"fmt"
	"os/exec"
	"sort"
	"strconv"
	"strings"
)

type procCPU struct {
	pid  int
	pcpu float64
	comm string
}

func PrintTopCPU(outDir string) error {
	out, err := exec.Command("ps", "-A", "-o", "pid,pcpu,comm").Output()
	if err != nil {
		fmt.Println("提示：受限环境无法执行 ps，请在本机终端运行该命令")
		return nil
	}
	var items []procCPU
	sc := bufio.NewScanner(strings.NewReader(string(out)))
	for sc.Scan() {
		line := strings.TrimSpace(sc.Text())
		if line == "" || strings.HasPrefix(line, "PID") {
			continue
		}
		fields := strings.Fields(line)
		if len(fields) < 3 {
			continue
		}
		pid, _ := strconv.Atoi(fields[0])
		pcpu, _ := strconv.ParseFloat(fields[1], 64)
		comm := strings.Join(fields[2:], " ")
		items = append(items, procCPU{pid: pid, pcpu: pcpu, comm: comm})
	}
	sort.Slice(items, func(i, j int) bool { return items[i].pcpu > items[j].pcpu })
	n := 10
	if len(items) < n {
		n = len(items)
	}
	for i := 0; i < n; i++ {
		fmt.Printf("%5d  %6.2f%%  %s\n", items[i].pid, items[i].pcpu, items[i].comm)
	}
	return nil
}

func PrintMemStats(outDir string) error {
	out, err := exec.Command("ps", "-A", "-o", "pid,rss,comm").Output()
	if err != nil {
		fmt.Println("提示：受限环境无法执行 ps，请在本机终端运行该命令")
		return nil
	}
	type procMem struct {
		pid  int
		rss  int
		comm string
	}
	var items []procMem
	sc := bufio.NewScanner(strings.NewReader(string(out)))
	for sc.Scan() {
		line := strings.TrimSpace(sc.Text())
		if line == "" || strings.HasPrefix(line, "PID") {
			continue
		}
		fields := strings.Fields(line)
		if len(fields) < 3 {
			continue
		}
		pid, _ := strconv.Atoi(fields[0])
		rss, _ := strconv.Atoi(fields[1])
		comm := strings.Join(fields[2:], " ")
		items = append(items, procMem{pid: pid, rss: rss, comm: comm})
	}
	sort.Slice(items, func(i, j int) bool { return items[i].rss > items[j].rss })
	n := 10
	if len(items) < n {
		n = len(items)
	}
	for i := 0; i < n; i++ {
		fmt.Printf("%5d  %8d KB  %s\n", items[i].pid, items[i].rss, items[i].comm)
	}
	return nil
}

func PrintIOStats(outDir string) error {
	out, err := exec.Command("iostat", "-d", "-n", "5", "-w", "1").CombinedOutput()
	if err != nil {
		fmt.Println("提示：受限环境无法执行 iostat，请在本机终端运行该命令")
		return nil
	}
	fmt.Print(string(out))
	return nil
}
