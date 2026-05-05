package ssh

import (
	"bytes"
	"crypto/rand"
	"io"
	"os"
	"path/filepath"
	"strconv"
	"testing"
	"time"
)

func envConfig() (Config, bool) {
	host := os.Getenv("SSH_HOST")
	user := os.Getenv("SSH_USER")
	pass := os.Getenv("SSH_PASS")
	portStr := os.Getenv("SSH_PORT")
	keyPath := os.Getenv("SSH_KEY_PATH")
	keyPass := os.Getenv("SSH_KEY_PASS")
	if host == "" || user == "" {
		return Config{}, false
	}
	port := 22
	if portStr != "" {
		if p, err := strconv.Atoi(portStr); err == nil {
			port = p
		}
	}
	var key []byte
	if keyPath != "" {
		if b, err := os.ReadFile(keyPath); err == nil {
			key = b
		}
	}
	var kp []byte
	if keyPass != "" {
		kp = []byte(keyPass)
	}
	return Config{
		Host:           host,
		Port:           port,
		User:           user,
		Password:       pass,
		PrivateKeyPEM:  key,
		PrivateKeyPass: kp,
		Timeout:        10 * time.Second,
	}, true
}

func TestRunEcho(t *testing.T) {
	cfg, ok := envConfig()
	if !ok {
		t.Skip("env not set: SSH_HOST/SSH_USER")
	}
	c, err := New(cfg)
	if err != nil {
		t.Fatalf("new ssh: %v", err)
	}
	defer c.Close()
	stdout, stderr, err := c.Run("echo -n hello")
	if err != nil {
		t.Fatalf("run: %v, stderr=%s", err, stderr)
	}
	if stdout != "hello" {
		t.Fatalf("unexpected stdout: %q", stdout)
	}
}

func TestUploadDownloadWithProgress(t *testing.T) {
	cfg, ok := envConfig()
	if !ok {
		t.Skip("env not set: SSH_HOST/SSH_USER")
	}
	c, err := New(cfg)
	if err != nil {
		t.Fatalf("new ssh: %v", err)
	}
	defer c.Close()

	tmpDir := t.TempDir()
	localSrc := filepath.Join(tmpDir, "src.bin")
	localDst := filepath.Join(tmpDir, "dst.bin")
	remote := "/tmp/ssh_test_" + strconv.FormatInt(time.Now().UnixNano(), 10) + ".bin"

	size := 256 * 1024
	data := make([]byte, size)
	if _, err := io.ReadFull(rand.Reader, data); err != nil {
		t.Fatalf("gen data: %v", err)
	}
	if err := os.WriteFile(localSrc, data, 0644); err != nil {
		t.Fatalf("write src: %v", err)
	}

	var upLast, upTotal int64
	err = c.Upload(localSrc, remote, 0644, func(w, tot int64) {
		upLast, upTotal = w, tot
	})
	if err != nil {
		t.Fatalf("upload: %v", err)
	}
	if upTotal != int64(size) || upLast != upTotal {
		t.Fatalf("upload progress mismatch: last=%d total=%d size=%d", upLast, upTotal, size)
	}

	var downLast, downTotal int64
	err = c.Download(remote, localDst, func(w, tot int64) {
		downLast, downTotal = w, tot
	})
	if err != nil {
		t.Fatalf("download: %v", err)
	}
	if downLast != downTotal || downTotal != int64(size) {
		t.Fatalf("download progress mismatch: last=%d total=%d size=%d", downLast, downTotal, size)
	}

	got, err := os.ReadFile(localDst)
	if err != nil {
		t.Fatalf("read dst: %v", err)
	}
	if !bytes.Equal(got, data) {
		t.Fatalf("content mismatch")
	}
}
