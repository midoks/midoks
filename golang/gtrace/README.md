# gtrace

基于 Go + eBPF 的进程跟踪与性能分析工具（骨架版本）。

## 功能概览
- CPU 采样与火焰图生成（折叠栈文件）
- 进程级指标（CPU/Mem/IO）框架与占位输出
- 跨平台构建：Linux 提供采样实现；非 Linux 明确提示不可用

## 快速开始
```bash
go build ./cmd/gtrace

./gtrace -mode=record -pid=0 -duration=30s -freq=99 -out=./output
./gtrace -mode=flamegraph -out=./output
./gtrace -mode=top -out=./output
./gtrace -mode=mem -out=./output
./gtrace -mode=io  -out=./output
```

> 提示：真实 eBPF 采样仅支持 Linux；当前仓库在非 Linux 平台将输出占位信息。

## 生成 eBPF 代码（Linux）
- 安装 clang/llvm、内核头文件；
- 运行：
```bash
GOOS=linux GOARCH=amd64 go generate ./internal/ebpf
```
这将使用 bpf2go 将 `internal/ebpf/perf.bpf.c` 编译为 Go 绑定文件。

## Linux 服务器配置与开发测试
- 一键脚本（自动安装工具链并尝试生成 vmlinux.h）：
```bash
bash scripts/setup_linux.sh
```
- 开发测试（示例拉起一个 busy 进程采样并生成预览）：
```bash
bash scripts/dev_test.sh ./output
```
- 手工步骤：
  - 安装依赖：clang、llvm、bpftool、pahole（dwarves）、gcc、pkg-config
  - 安装 bpf2go：`go install github.com/cilium/ebpf/cmd/bpf2go@latest`
  - 生成 vmlinux.h（可选，CO-RE 推荐）：
    `bpftool btf dump file /sys/kernel/btf/vmlinux format c > internal/ebpf/vmlinux.h`
  - 生成 Go 绑定：`GOOS=linux GOARCH=$(go env GOARCH) go generate ./internal/ebpf`
  - 构建：`go build ./cmd/gtrace`
  - 运行（可能需要 sudo/CAP_BPF）：`sudo ./gtrace -mode=record -pid=0 -duration=30s -freq=99 -out=./output`

提示：
- 需要启用 BPF/BTF 的现代 Linux 内核；建议在生产服务器上使用 root 或授予 CAP_BPF/CAP_SYS_ADMIN 权限
- 端到端真实火焰图需将 `RecordPerf` 接入生成的 bpf 对象并从 map 聚合折叠栈（当前为占位）

## 目录结构
- [cmd/gtrace](./cmd/gtrace): CLI 入口
- [internal/trace](./internal/trace): 采样/记录实现
- [internal/flamegraph](./internal/flamegraph): 火焰图折叠文件与生成
- [internal/metrics](./internal/metrics): CPU/Mem/IO 指标输出
- [internal/ebpf](./internal/ebpf): eBPF 程序与代码生成入口

## 后续扩展建议
- 将 `RecordPerf` 接入生成的 bpf2go 绑定，读取 stack map 与计数；
- 增加过滤：PID、进程名、cgroup、CPU 掩码；
- 完整的 IO（块设备/网络）与内存（RSS/缺页）事件追踪；
- 导出 speedscope JSON 与 SVG 火焰图的完整绘制。

## macOS 支持说明
- 提供基础支持：record 会生成占位折叠栈和元数据；flamegraph 可生成最简 SVG 预览
- 指标输出：top/mem/io 可在 macOS 上运行（依赖系统 ps/iostat）
- 建议增强：如需真实火焰图，可集成 dtrace/profile provider 或解析 `sample`/`spindump` 输出转折叠栈
