package tools

import (
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"path/filepath"
	"strings"
)

// SHA256 is the algorithm identifier for HashData
const SHA256 = "SHA256"

// HashData returns the hexadecimal digest for provided data using the given algorithm.
// Currently only SHA256 is supported.
func HashData(algorithm string, data []byte) string {
	switch strings.ToUpper(algorithm) {
	case "SHA256":
		sum := sha256.Sum256(data)
		return hex.EncodeToString(sum[:])
	default:
		sum := sha256.Sum256(data)
		return hex.EncodeToString(sum[:])
	}
}

// JoinBasePath safely joins a base path with a requested relative path.
// It prevents path traversal by rejecting absolute paths and ".." segments.
func JoinBasePath(base string, req string) (string, error) {
	if IsMaliciousPath(req) {
		return "", fmt.Errorf("malicious path: %s", req)
	}
	// Ensure request path is treated as relative
	req = strings.TrimPrefix(req, "/")
	joined := filepath.Join(base, req)
	return filepath.Clean(joined), nil
}
