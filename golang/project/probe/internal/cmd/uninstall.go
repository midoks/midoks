package cmd

import (
	"fmt"
	"os"
	"path/filepath"
	"runtime"

	"github.com/urfave/cli"
)

var Uninstall = cli.Command{
	Name:        "uninstall",
	Usage:       "uninstall mgo systemd service",
	Description: `uninstall mgo systemd service from Linux`,
	Action:      runUninstall,
	Flags: []cli.Flag{
		stringFlag("name, n", "mgo_web", "service name"),
		boolFlag("purge", "remove service file as well"),
	},
}

func runUninstall(c *cli.Context) error {
	if runtime.GOOS != "linux" {
		return fmt.Errorf("this command is only supported on Linux systems")
	}

	serviceName := c.String("name")
	purge := c.Bool("purge")

	fmt.Printf("Uninstalling mgo service '%s'...\n", serviceName)

	if os.Getuid() != 0 {
		return fmt.Errorf("this command requires root privileges. Please run with sudo")
	}

	servicePath := filepath.Join("/etc/systemd/system", serviceName+".service")

	if _, err := os.Stat(servicePath); os.IsNotExist(err) {
		return fmt.Errorf("service file not found: %s", servicePath)
	}

	if err := runCommand("systemctl", "stop", serviceName); err != nil {
		fmt.Printf("Warning: failed to stop service: %v\n", err)
	}

	if err := runCommand("systemctl", "disable", serviceName); err != nil {
		fmt.Printf("Warning: failed to disable service: %v\n", err)
	}

	if purge {
		if err := os.Remove(servicePath); err != nil {
			return fmt.Errorf("failed to remove service file: %v", err)
		}
		fmt.Printf("Service file removed: %s\n", servicePath)

		if err := runCommand("systemctl", "daemon-reload"); err != nil {
			fmt.Printf("Warning: failed to reload systemd: %v\n", err)
		}

		if err := runCommand("systemctl", "reset-failed"); err != nil {
			fmt.Printf("Warning: failed to reset failed state: %v\n", err)
		}
	}

	fmt.Printf("\n✓ Service '%s' uninstalled successfully!\n", serviceName)
	fmt.Printf("  Service file: %s\n", servicePath)
	fmt.Printf("  Note: The installation directory and data files are not removed.\n")
	fmt.Printf("  To remove them manually, delete the installation directory.\n")

	return nil
}
