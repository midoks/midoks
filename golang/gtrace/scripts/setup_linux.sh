#!/usr/bin/env bash
set -euo pipefail

echo "==> Detecting package manager..."
PM=""
if command -v apt-get >/dev/null 2>&1; then PM="apt";
elif command -v dnf >/dev/null 2>&1; then PM="dnf";
elif command -v yum >/dev/null 2>&1; then PM="yum";
elif command -v zypper >/dev/null 2>&1; then PM="zypper";
fi
echo "==> Package manager: ${PM:-unknown}"

echo "==> Installing toolchain: clang/llvm, bpftool, pahole (dwarf utils)"
case "${PM}" in
  apt)
    sudo apt-get update
    sudo apt-get install -y clang llvm bpftool pahole make gcc pkg-config
    ;;
  dnf)
    sudo dnf install -y clang llvm bpftool dwarves make gcc pkg-config
    ;;
  yum)
    sudo yum install -y clang llvm bpftool dwarves make gcc pkg-config || true
    ;;
  zypper)
    sudo zypper install -y clang llvm bpftool dwarves make gcc pkg-config
    ;;
  *)
    echo "Unknown package manager. Please install manually: clang llvm bpftool dwarves"
    ;;
esac

echo "==> Installing bpf2go (cilium/ebpf)"
GO_BIN="${GOBIN:-$(go env GOPATH)/bin}"
go install github.com/cilium/ebpf/cmd/bpf2go@latest
echo "bpf2go installed to: ${GO_BIN}"

echo "==> Checking kernel BTF (for CO-RE) ..."
if [ -f /sys/kernel/btf/vmlinux ]; then
  echo "Found /sys/kernel/btf/vmlinux"
else
  echo "No /sys/kernel/btf/vmlinux. You may need a kernel with BTF or install a btfhub package."
fi

echo "==> Generating vmlinux.h ..."
if command -v bpftool >/dev/null 2>&1 && [ -f /sys/kernel/btf/vmlinux ]; then
  bpftool btf dump file /sys/kernel/btf/vmlinux format c > internal/ebpf/vmlinux.h
  echo "Generated internal/ebpf/vmlinux.h"
else
  echo "Skip generating vmlinux.h (bpftool or BTF not available)."
fi

echo "==> Done. Next steps:"
echo "  1) GOOS=linux GOARCH=$(go env GOARCH) go generate ./internal/ebpf"
echo "  2) go build ./cmd/gtrace"
echo "  3) sudo ./gtrace -mode=record -pid=0 -duration=30s -freq=99 -out=./output"
