OUTPUT_DIR := output

.PHONY: build
build:
	go build ./cmd/gtrace

.PHONY: generate
generate:
	GOOS=linux GOARCH=amd64 go generate ./internal/ebpf

.PHONY: record
record:
	./gtrace -mode=record -duration=30s -freq=99 -out=$(OUTPUT_DIR)

.PHONY: flame
flame:
	./gtrace -mode=flamegraph -out=$(OUTPUT_DIR)
