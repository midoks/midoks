package main

import (
	"fmt"
	"time"

	"log"
	"os"

	"github.com/urfave/cli"

	"aiprobe/internal/cmd"
	"aiprobe/internal/conf"
)

const (
	Version = "1.0"
	AppName = "probe"
	CodeDev = true
)

func init() {
	conf.App.Version = Version
	if CodeDev {
		conf.App.Version = fmt.Sprintf("%s%d", Version, time.Now().Unix())
	}
	conf.App.Name = AppName
}

func main() {
	app := cli.NewApp()
	app.Name = AppName
	app.Version = Version
	app.Usage = "a probe service"
	app.Commands = []cli.Command{
		cmd.Web,
		cmd.Root,
		cmd.Install,
		cmd.Uninstall,
	}

	if err := app.Run(os.Args); err != nil {
		log.Fatalf("failed to start application: %v", err)
	}
}
