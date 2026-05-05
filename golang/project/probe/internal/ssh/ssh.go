package ssh

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"

	"golang.org/x/crypto/ssh"
)

type ProgressFunc func(written int64, total int64)

type Config struct {
	Host           string
	Port           int
	User           string
	Password       string
	PrivateKeyPEM  []byte
	PrivateKeyPass []byte
	Timeout        time.Duration
}

type Client struct {
	raw *ssh.Client
}

func New(cfg Config) (*Client, error) {
	if cfg.Port == 0 {
		cfg.Port = 22
	}
	auths := []ssh.AuthMethod{}
	if len(cfg.PrivateKeyPEM) > 0 {
		signer, err := parseKey(cfg.PrivateKeyPEM, cfg.PrivateKeyPass)
		if err != nil {
			return nil, err
		}
		auths = append(auths, ssh.PublicKeys(signer))
	}
	if cfg.Password != "" {
		auths = append(auths, ssh.Password(cfg.Password))
	}
	cc := &ssh.ClientConfig{
		User:            cfg.User,
		Auth:            auths,
		HostKeyCallback: ssh.InsecureIgnoreHostKey(),
		Timeout:         cfg.Timeout,
	}
	addr := fmt.Sprintf("%s:%d", cfg.Host, cfg.Port)
	c, err := ssh.Dial("tcp", addr, cc)
	if err != nil {
		return nil, err
	}
	return &Client{raw: c}, nil
}

func parseKey(pem []byte, pass []byte) (ssh.Signer, error) {
	if len(pass) > 0 {
		return ssh.ParsePrivateKeyWithPassphrase(pem, pass)
	}
	return ssh.ParsePrivateKey(pem)
}

func (c *Client) Close() error {
	if c.raw != nil {
		return c.raw.Close()
	}
	return nil
}

func (c *Client) Run(cmd string) (string, string, error) {
	s, err := c.raw.NewSession()
	if err != nil {
		return "", "", err
	}
	defer s.Close()
	var stdout, stderr strings.Builder
	s.Stdout = &stdout
	s.Stderr = &stderr
	err = s.Run(cmd)
	return stdout.String(), stderr.String(), err
}

func (c *Client) Upload(localPath, remotePath string, perm os.FileMode, cb ProgressFunc) error {
	f, err := os.Open(localPath)
	if err != nil {
		return err
	}
	defer f.Close()
	info, err := f.Stat()
	if err != nil {
		return err
	}
	total := info.Size()
	dir := filepath.Dir(remotePath)
	_, _, _ = c.Run("mkdir -p " + quote(dir))
	s, err := c.raw.NewSession()
	if err != nil {
		return err
	}
	defer s.Close()
	w, err := s.StdinPipe()
	if err != nil {
		return err
	}
	s.Stdout = io.Discard
	s.Stderr = io.Discard
	chmod := fmt.Sprintf("%#o", perm)
	cmd := fmt.Sprintf("sh -c 'cat > %s && chmod %s %s'", quote(remotePath), chmod, quote(remotePath))
	if err := s.Start(cmd); err != nil {
		return err
	}
	if _, err := copyWithProgress(f, w, total, cb); err != nil {
		_ = w.Close()
		_ = s.Wait()
		return err
	}
	_ = w.Close()
	return s.Wait()
}

func (c *Client) Download(remotePath, localPath string, cb ProgressFunc) error {
	size, _ := c.remoteSize(remotePath)
	s, err := c.raw.NewSession()
	if err != nil {
		return err
	}
	defer s.Close()
	pr, pw := io.Pipe()
	s.Stdout = pw
	s.Stderr = io.Discard
	go func() {
		_ = s.Run("sh -c 'cat " + quote(remotePath) + "'")
		_ = pw.Close()
	}()
	out, err := os.Create(localPath)
	if err != nil {
		return err
	}
	defer out.Close()
	_, err = copyWithProgress(pr, out, size, cb)
	return err
}

func (c *Client) remoteSize(p string) (int64, error) {
	out, _, err := c.Run("sh -c 'wc -c < " + quote(p) + "'")
	if err != nil {
		return 0, err
	}
	sc := bufio.NewScanner(strings.NewReader(out))
	if sc.Scan() {
		txt := strings.TrimSpace(sc.Text())
		n, err := strconv.ParseInt(txt, 10, 64)
		if err == nil {
			return n, nil
		}
	}
	return 0, nil
}

func copyWithProgress(r io.Reader, w io.Writer, total int64, cb ProgressFunc) (int64, error) {
	var written int64
	buf := make([]byte, 64*1024)
	for {
		nr, er := r.Read(buf)
		if nr > 0 {
			nw, ew := w.Write(buf[:nr])
			if nw > 0 {
				written += int64(nw)
				if cb != nil {
					cb(written, total)
				}
			}
			if ew != nil {
				return written, ew
			}
			if nr != nw {
				return written, io.ErrShortWrite
			}
		}
		if er != nil {
			if er == io.EOF {
				break
			}
			return written, er
		}
	}
	return written, nil
}

func quote(s string) string {
	return "'" + strings.ReplaceAll(s, "'", "'\"'\"'") + "'"
}
