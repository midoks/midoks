package flamegraph

import (
	"fmt"
	"os"
	"path/filepath"
)

func Generate(outDir string) error {
	stacksPath := filepath.Join(outDir, "stacks.folded")
	if _, err := os.Stat(stacksPath); err != nil {
		return fmt.Errorf("未找到折叠栈文件: %s", stacksPath)
	}
	svgPath := filepath.Join(outDir, "flamegraph.svg")
	data, err := os.ReadFile(stacksPath)
	if err != nil {
		return err
	}
	if len(data) == 0 {
		return fmt.Errorf("折叠栈为空: %s", stacksPath)
	}
	svg := minimalSVG()
	return os.WriteFile(svgPath, []byte(svg), 0644)
}

func minimalSVG() string {
	return `<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="200"><text x="10" y="20">Flamegraph 预览（请在 Linux 上采样生成真实图）</text></svg>`
}
