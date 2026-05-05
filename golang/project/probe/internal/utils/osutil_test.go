package tools

import (
	"os"
	"path/filepath"
	"testing"
)

func TestIsFile(t *testing.T) {
	// Create a temporary directory for testing
	tempDir, err := os.MkdirTemp("", "osutil_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	// Create a test file
	testFile := filepath.Join(tempDir, "test.txt")
	err = os.WriteFile(testFile, []byte("test content"), 0644)
	if err != nil {
		t.Fatalf("Failed to create test file: %v", err)
	}

	// Create a test directory
	testDir := filepath.Join(tempDir, "testdir")
	err = os.MkdirAll(testDir, 0755)
	if err != nil {
		t.Fatalf("Failed to create test directory: %v", err)
	}

	tests := []struct {
		name string
		path string
		want bool
	}{
		{
			name: "existing file",
			path: testFile,
			want: true,
		},
		{
			name: "existing directory",
			path: testDir,
			want: false,
		},
		{
			name: "non-existent path",
			path: filepath.Join(tempDir, "nonexistent"),
			want: false,
		},
		{
			name: "empty path",
			path: "",
			want: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := IsFile(tt.path); got != tt.want {
				t.Errorf("IsFile() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestIsDir(t *testing.T) {
	// Create a temporary directory for testing
	tempDir, err := os.MkdirTemp("", "osutil_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	// Create a test file
	testFile := filepath.Join(tempDir, "test.txt")
	err = os.WriteFile(testFile, []byte("test content"), 0644)
	if err != nil {
		t.Fatalf("Failed to create test file: %v", err)
	}

	// Create a test directory
	testDir := filepath.Join(tempDir, "testdir")
	err = os.MkdirAll(testDir, 0755)
	if err != nil {
		t.Fatalf("Failed to create test directory: %v", err)
	}

	tests := []struct {
		name string
		path string
		want bool
	}{
		{
			name: "existing directory",
			path: testDir,
			want: true,
		},
		{
			name: "temp directory itself",
			path: tempDir,
			want: true,
		},
		{
			name: "existing file",
			path: testFile,
			want: false,
		},
		{
			name: "non-existent path",
			path: filepath.Join(tempDir, "nonexistent"),
			want: false,
		},
		{
			name: "empty path",
			path: "",
			want: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := IsDir(tt.path); got != tt.want {
				t.Errorf("IsDir() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestIsExist(t *testing.T) {
	// Create a temporary directory for testing
	tempDir, err := os.MkdirTemp("", "osutil_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	// Create a test file
	testFile := filepath.Join(tempDir, "test.txt")
	err = os.WriteFile(testFile, []byte("test content"), 0644)
	if err != nil {
		t.Fatalf("Failed to create test file: %v", err)
	}

	// Create a test directory
	testDir := filepath.Join(tempDir, "testdir")
	err = os.MkdirAll(testDir, 0755)
	if err != nil {
		t.Fatalf("Failed to create test directory: %v", err)
	}

	tests := []struct {
		name string
		path string
		want bool
	}{
		{
			name: "existing file",
			path: testFile,
			want: true,
		},
		{
			name: "existing directory",
			path: testDir,
			want: true,
		},
		{
			name: "temp directory itself",
			path: tempDir,
			want: true,
		},
		{
			name: "non-existent path",
			path: filepath.Join(tempDir, "nonexistent"),
			want: false,
		},
		{
			name: "empty path",
			path: "",
			want: false,
		},
		{
			name: "invalid path",
			path: "/invalid/path/that/does/not/exist",
			want: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := IsExist(tt.path); got != tt.want {
				t.Errorf("IsExist() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestCurrentUsername(t *testing.T) {
	// Save original environment variables
	originalUSER := os.Getenv("USER")
	originalUSERNAME := os.Getenv("USERNAME")
	defer func() {
		os.Setenv("USER", originalUSER)
		os.Setenv("USERNAME", originalUSERNAME)
	}()

	t.Run("with USER environment variable", func(t *testing.T) {
		os.Setenv("USER", "testuser")
		os.Unsetenv("USERNAME")

		got := CurrentUsername()
		if got != "testuser" {
			t.Errorf("CurrentUsername() = %v, want %v", got, "testuser")
		}
	})

	t.Run("with USERNAME environment variable", func(t *testing.T) {
		os.Unsetenv("USER")
		os.Setenv("USERNAME", "testuser2")

		got := CurrentUsername()
		if got != "testuser2" {
			t.Errorf("CurrentUsername() = %v, want %v", got, "testuser2")
		}
	})

	t.Run("without environment variables", func(t *testing.T) {
		os.Unsetenv("USER")
		os.Unsetenv("USERNAME")

		got := CurrentUsername()
		// Should fallback to user.Current()
		// We can't predict the exact username, but it should not be empty
		// unless there's a system error
		if got == "" {
			t.Log("CurrentUsername() returned empty string, which may be expected on some systems")
		} else {
			t.Logf("CurrentUsername() = %v (from user.Current())", got)
		}
	})

	t.Run("with both environment variables set", func(t *testing.T) {
		os.Setenv("USER", "user1")
		os.Setenv("USERNAME", "user2")

		got := CurrentUsername()
		// USER should take precedence
		if got != "user1" {
			t.Errorf("CurrentUsername() = %v, want %v (USER should take precedence)", got, "user1")
		}
	})

	t.Run("with empty environment variables", func(t *testing.T) {
		os.Setenv("USER", "")
		os.Setenv("USERNAME", "testuser3")

		got := CurrentUsername()
		// Should use USERNAME since USER is empty
		if got != "testuser3" {
			t.Errorf("CurrentUsername() = %v, want %v", got, "testuser3")
		}
	})
}

// Test edge cases
func TestIsFileEdgeCases(t *testing.T) {
	t.Run("symlink to file", func(t *testing.T) {
		// Create a temporary directory for testing
		tempDir, err := os.MkdirTemp("", "osutil_test_symlink")
		if err != nil {
			t.Fatalf("Failed to create temp dir: %v", err)
		}
		defer os.RemoveAll(tempDir)

		// Create a test file
		testFile := filepath.Join(tempDir, "test.txt")
		err = os.WriteFile(testFile, []byte("test content"), 0644)
		if err != nil {
			t.Fatalf("Failed to create test file: %v", err)
		}

		// Create a symlink to the file
		symlinkPath := filepath.Join(tempDir, "symlink.txt")
		err = os.Symlink(testFile, symlinkPath)
		if err != nil {
			t.Skipf("Failed to create symlink (may not be supported): %v", err)
		}

		// Test that symlink to file is detected as file
		if !IsFile(symlinkPath) {
			t.Errorf("IsFile() should return true for symlink to file")
		}
	})
}

func TestIsDirEdgeCases(t *testing.T) {
	t.Run("symlink to directory", func(t *testing.T) {
		// Create a temporary directory for testing
		tempDir, err := os.MkdirTemp("", "osutil_test_symlink")
		if err != nil {
			t.Fatalf("Failed to create temp dir: %v", err)
		}
		defer os.RemoveAll(tempDir)

		// Create a test directory
		testDir := filepath.Join(tempDir, "testdir")
		err = os.MkdirAll(testDir, 0755)
		if err != nil {
			t.Fatalf("Failed to create test directory: %v", err)
		}

		// Create a symlink to the directory
		symlinkPath := filepath.Join(tempDir, "symlink_dir")
		err = os.Symlink(testDir, symlinkPath)
		if err != nil {
			t.Skipf("Failed to create symlink (may not be supported): %v", err)
		}

		// Test that symlink to directory is detected as directory
		if !IsDir(symlinkPath) {
			t.Errorf("IsDir() should return true for symlink to directory")
		}
	})
}

// Benchmark tests
func BenchmarkIsFile(b *testing.B) {
	// Create a temporary file for benchmarking
	tempDir, err := os.MkdirTemp("", "osutil_bench")
	if err != nil {
		b.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	testFile := filepath.Join(tempDir, "test.txt")
	os.WriteFile(testFile, []byte("test content"), 0644)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		IsFile(testFile)
	}
}

func BenchmarkIsDir(b *testing.B) {
	// Create a temporary directory for benchmarking
	tempDir, err := os.MkdirTemp("", "osutil_bench")
	if err != nil {
		b.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		IsDir(tempDir)
	}
}

func BenchmarkIsExist(b *testing.B) {
	// Create a temporary file for benchmarking
	tempDir, err := os.MkdirTemp("", "osutil_bench")
	if err != nil {
		b.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	testFile := filepath.Join(tempDir, "test.txt")
	os.WriteFile(testFile, []byte("test content"), 0644)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		IsExist(testFile)
	}
}

func BenchmarkCurrentUsername(b *testing.B) {
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		CurrentUsername()
	}
}
