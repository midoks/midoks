#!/usr/bin/env bash
set -euo pipefail

OUT_DIR="${1:-./output}"
FREQ="${FREQ:-99}"
DURATION="${DURATION:-30s}"

echo "==> Generate eBPF bindings (Linux only)"
GOOS=linux GOARCH=$(go env GOARCH) go generate ./internal/ebpf || true

echo "==> Build gtrace"
go build ./cmd/gtrace

echo "==> Start a busy process for demo (yes > /dev/null)"
(yes > /dev/null) & DEMO_PID=$!
trap "kill ${DEMO_PID} >/dev/null 2>&1 || true" EXIT
sleep 1

echo "==> Record CPU samples (may require sudo)"
sudo ./gtrace -mode=record -pid=${DEMO_PID} -duration=${DURATION} -freq=${FREQ} -out=${OUT_DIR}

echo "==> Generate flamegraph preview"
./gtrace -mode=flamegraph -out=${OUT_DIR}
ls -l ${OUT_DIR}/flamegraph.svg

echo "==> Print top/mem/io"
./gtrace -mode=top -out=${OUT_DIR} || true
./gtrace -mode=mem -out=${OUT_DIR} || true
./gtrace -mode=io  -out=${OUT_DIR} || true

echo "==> Done. Output in ${OUT_DIR}"
