.PHONY: help build test release clean install uninstall release-local release-check build-linux build-macos build-windows

help:
	@echo "Network Probe - Makefile commands:"
	@echo ""
	@echo "Development:"
	@echo "  make build          - Build the project in debug mode"
	@echo "  make test           - Run tests"
	@echo "  make clean          - Clean build artifacts"
	@echo ""
	@echo "Release:"
	@echo "  make release        - Build release binary"
	@echo "  make build-linux    - Build for Linux (x86_64)"
	@echo "  make build-macos    - Build for macOS (native)"
	@echo "  make build-windows  - Build for Windows (x86_64)"
	@echo ""
	@echo "Packaging:"
	@echo "  make package-deb    - Build Debian package"
	@echo "  make package-rpm    - Build RPM package"
	@echo "  make package-all    - Build all packages"
	@echo ""
	@echo "Installation:"
	@echo "  make install        - Install the binary (requires sudo)"
	@echo "  make uninstall      - Uninstall the binary (requires sudo)"

build:
	cargo build

test:
	cargo test --all

release:
	cargo build --release

clean:
	cargo clean

install:
	@echo "Installing Network Probe..."
	@cargo install --path .
	@echo "Network Probe installed successfully!"

uninstall:
	@echo "Uninstalling Network Probe..."
	@cargo uninstall network-probe
	@echo "Network Probe uninstalled successfully!"

build-linux:
	@echo "Building for Linux x86_64..."
	@echo "Note: Cross-compilation requires rustup and target installation"
	@echo "Run: rustup target add x86_64-unknown-linux-gnu"
	cargo build --release --target x86_64-unknown-linux-gnu || echo "Cross-compilation failed. Using rustup is recommended."

build-macos:
	@echo "Building for macOS (native)..."
	cargo build --release
	@echo "macOS binary built at target/release/network-probe"

build-windows:
	@echo "Building for Windows x86_64..."
	@echo "Note: Cross-compilation requires rustup and target installation"
	@echo "Run: rustup target add x86_64-pc-windows-gnu"
	cargo build --release --target x86_64-pc-windows-gnu || echo "Cross-compilation failed. Using rustup is recommended."

package-deb:
	@echo "Building Debian package..."
	@echo "Note: Requires cargo-deb (cargo install cargo-deb)"
	cargo deb || echo "Packaging failed. Install cargo-deb first."

package-rpm:
	@echo "Building RPM package..."
	@echo "Note: Requires cargo-rpm (cargo install cargo-rpm)"
	cargo rpm build || echo "Packaging failed. Install cargo-rpm first."

package-all: package-deb package-rpm
	@echo "All packages built successfully!"

docker-build:
	@echo "Building Docker image..."
	@docker build -t network-probe:latest .

docker-run:
	@echo "Running Docker container..."
	@docker run -it --rm network-probe:latest

lint:
	@echo "Running linters..."
	@cargo clippy --all-targets --all-features

fmt:
	@echo "Formatting code..."
	@cargo fmt --all

check: lint test
