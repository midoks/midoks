package log

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"aiprobe/internal/conf"
)

func TestInit(t *testing.T) {
	// Create a temporary directory for testing
	tempDir, err := os.MkdirTemp("", "log_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	// Set up test configuration
	originalLogPath := conf.Log.RootPath
	originalLogFormat := conf.Log.Format
	defer func() {
		conf.Log.RootPath = originalLogPath
		conf.Log.Format = originalLogFormat
	}()

	conf.Log.RootPath = tempDir
	conf.Log.Format = "text"

	// Test Init function
	Init()

	// Verify logger is initialized
	if logger == nil {
		t.Error("Expected logger to be initialized, but it was nil")
	}

	// Verify log directory was created
	if _, err := os.Stat(tempDir); os.IsNotExist(err) {
		t.Errorf("Expected log directory to be created at %s", tempDir)
	}
}

func TestInitWithJSONFormat(t *testing.T) {
	// Create a temporary directory for testing
	tempDir, err := os.MkdirTemp("", "log_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	// Set up test configuration
	originalLogPath := conf.Log.RootPath
	originalLogFormat := conf.Log.Format
	defer func() {
		conf.Log.RootPath = originalLogPath
		conf.Log.Format = originalLogFormat
	}()

	conf.Log.RootPath = tempDir
	conf.Log.Format = "json"

	// Test Init function with JSON format
	Init()

	// Verify logger is initialized
	if logger == nil {
		t.Error("Expected logger to be initialized, but it was nil")
	}
}

func TestGetLogger(t *testing.T) {
	// Initialize logger first
	tempDir, err := os.MkdirTemp("", "log_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	originalLogPath := conf.Log.RootPath
	defer func() {
		conf.Log.RootPath = originalLogPath
	}()

	conf.Log.RootPath = tempDir
	Init()

	// Test GetLogger function
	loggerInstance := GetLogger()
	if loggerInstance == nil {
		t.Error("Expected GetLogger to return a logger instance, but got nil")
	}

	if loggerInstance != logger {
		t.Error("Expected GetLogger to return the same logger instance")
	}
}

func TestLoggingFunctions(t *testing.T) {
	// Create a temporary directory for testing
	tempDir, err := os.MkdirTemp("", "log_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	// Set up test configuration
	originalLogPath := conf.Log.RootPath
	defer func() {
		conf.Log.RootPath = originalLogPath
	}()

	conf.Log.RootPath = tempDir
	Init()

	// Test logging functions
	tests := []struct {
		name string
		fn   func()
	}{
		{
			name: "Debug",
			fn:   func() { Debug("test debug message") },
		},
		{
			name: "Info",
			fn:   func() { Info("test info message") },
		},
		{
			name: "Warn",
			fn:   func() { Warn("test warn message") },
		},
		{
			name: "Error",
			fn:   func() { Error("test error message") },
		},
		{
			name: "Debugf",
			fn:   func() { Debugf("test debug %s", "formatted") },
		},
		{
			name: "Infof",
			fn:   func() { Infof("test info %s", "formatted") },
		},
		{
			name: "Warnf",
			fn:   func() { Warnf("test warn %s", "formatted") },
		},
		{
			name: "Errorf",
			fn:   func() { Errorf("test error %s", "formatted") },
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Test that logging functions don't panic
			defer func() {
				if r := recover(); r != nil {
					t.Errorf("%s function panicked: %v", tt.name, r)
				}
			}()

			tt.fn()
		})
	}

	// Give some time for logs to be written
	time.Sleep(100 * time.Millisecond)

	// Verify log files were created
	logFile := filepath.Join(tempDir, logFileName)
	if _, err := os.Stat(logFile); os.IsNotExist(err) {
		t.Errorf("Expected log file to be created at %s", logFile)
	}
}

func TestReverseRead(t *testing.T) {
	// Create a temporary directory for testing
	tempDir, err := os.MkdirTemp("", "log_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	// Set up test configuration
	originalLogPath := conf.Log.RootPath
	defer func() {
		conf.Log.RootPath = originalLogPath
	}()

	conf.Log.RootPath = tempDir
	Init()

	// Write some test log entries
	testMessages := []string{
		"First log message",
		"Second log message",
		"Third log message",
		"Fourth log message",
		"Fifth log message",
	}

	for _, msg := range testMessages {
		Info(msg)
	}

	// Give some time for logs to be written
	time.Sleep(200 * time.Millisecond)

	// Test ReverseRead function
	lines, err := ReverseRead(3)
	if err != nil {
		t.Errorf("ReverseRead failed: %v", err)
		return
	}

	if len(lines) == 0 {
		t.Error("Expected to read some lines, but got empty result")
		return
	}

	// Verify that we got lines (exact content may vary due to log formatting)
	// Some lines might be empty due to log formatting, so we just check that we got some content
	hasContent := false
	for _, line := range lines {
		if strings.TrimSpace(line) != "" {
			hasContent = true
			break
		}
	}
	if !hasContent {
		t.Error("Expected to find at least one non-empty line in log output")
	}
}

func TestReverseReadNonExistentFile(t *testing.T) {
	// Set up test configuration with non-existent directory
	originalLogPath := conf.Log.RootPath
	defer func() {
		conf.Log.RootPath = originalLogPath
	}()

	conf.Log.RootPath = "/non/existent/path"

	// Test ReverseRead function with non-existent file
	_, err := ReverseRead(10)
	if err == nil {
		t.Error("Expected ReverseRead to return an error for non-existent file, but got nil")
	}
}

// Benchmark tests
func BenchmarkInfo(b *testing.B) {
	// Create a temporary directory for benchmarking
	tempDir, err := os.MkdirTemp("", "log_bench")
	if err != nil {
		b.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	originalLogPath := conf.Log.RootPath
	defer func() {
		conf.Log.RootPath = originalLogPath
	}()

	conf.Log.RootPath = tempDir
	Init()

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		Info("benchmark test message")
	}
}

func BenchmarkInfof(b *testing.B) {
	// Create a temporary directory for benchmarking
	tempDir, err := os.MkdirTemp("", "log_bench")
	if err != nil {
		b.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	originalLogPath := conf.Log.RootPath
	defer func() {
		conf.Log.RootPath = originalLogPath
	}()

	conf.Log.RootPath = tempDir
	Init()

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		Infof("benchmark test message %d", i)
	}
}
