package cmd

import (
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"text/template"

	"github.com/urfave/cli"
)

var Install = cli.Command{
	Name:        "install",
	Usage:       "install mgo as a systemd service",
	Description: `install mgo as a systemd service on Linux`,
	Action:      runInstall,
	Flags: []cli.Flag{
		stringFlag("config, c", "", "custom configuration file path"),
		stringFlag("name, n", "mgo_web", "service name"),
		stringFlag("user, u", "root", "run as user"),
		stringFlag("dir, d", ".", "installation directory"),
	},
}

const systemdServiceTemplate = `[Unit]
Description=MGO Service
After=network.target

[Service]
Type=simple
User={{.User}}
WorkingDirectory={{.Dir}}
ExecStart={{.Dir}}/mgo_web web
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
`

func runInstall(c *cli.Context) error {
	if runtime.GOOS != "linux" {
		return fmt.Errorf("this command is only supported on Linux systems")
	}

	serviceName := c.String("name")
	user := c.String("user")
	installDir := c.String("dir")
	configPath := c.String("config")

	if installDir == "." {
		execPath, err := os.Executable()
		if err != nil {
			return fmt.Errorf("failed to get executable path: %v", err)
		}
		installDir = filepath.Dir(execPath)
	}

	fmt.Printf("Installing mgo as systemd service '%s'...\n", serviceName)
	fmt.Printf("Installation directory: %s\n", installDir)
	fmt.Printf("Running as user: %s\n", user)

	if os.Getuid() != 0 {
		return fmt.Errorf("this command requires root privileges. Please run with sudo")
	}

	serviceContent, err := executeTemplate(systemdServiceTemplate, map[string]string{
		"User":   user,
		"Dir":    installDir,
		"Config": configPath,
	})
	if err != nil {
		return fmt.Errorf("failed to generate service file: %v", err)
	}

	servicePath := filepath.Join("/etc/systemd/system", serviceName+".service")

	if err := os.WriteFile(servicePath, []byte(serviceContent), 0644); err != nil {
		return fmt.Errorf("failed to write service file: %v", err)
	}

	fmt.Printf("Service file created at: %s\n", servicePath)

	if err := runCommand("systemctl", "daemon-reload"); err != nil {
		return fmt.Errorf("failed to reload systemd: %v", err)
	}

	if err := runCommand("systemctl", "enable", serviceName); err != nil {
		return fmt.Errorf("failed to enable service: %v", err)
	}

	if err := runCommand("systemctl", "start", serviceName); err != nil {
		return fmt.Errorf("failed to start service: %v", err)
	}

	fmt.Printf("\n✓ Service '%s' installed and started successfully!\n", serviceName)
	fmt.Printf("  Service file: %s\n", servicePath)
	fmt.Printf("  To check status: systemctl status %s\n", serviceName)
	fmt.Printf("  To view logs: journalctl -u %s -f\n", serviceName)
	fmt.Printf("  To stop: systemctl stop %s\n", serviceName)
	fmt.Printf("  To restart: systemctl restart %s\n", serviceName)

	return nil
}

func executeTemplate(tmpl string, data map[string]string) (string, error) {
	t, err := template.New("service").Parse(tmpl)
	if err != nil {
		return "", err
	}

	var buf strings.Builder
	if err := t.Execute(&buf, data); err != nil {
		return "", err
	}

	return buf.String(), nil
}
