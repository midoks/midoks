package main

import (
	"flag"
	"fmt"
	"log"
	"os"

	"github.com/0ne-zero/goTraceroute/pkg/core/hop"
	"github.com/0ne-zero/goTraceroute/pkg/core/options"
	"github.com/0ne-zero/goTraceroute/pkg/core/traceroute"
)

func usage() {
	fmt.Fprintf(os.Stderr, "Usage: %s [options] <target>\n", os.Args[0])
	fmt.Fprintf(os.Stderr, "\nOptions:\n")
	flag.PrintDefaults()
	fmt.Fprintf(os.Stderr, "\nExample:\n")
	fmt.Fprintf(os.Stderr, "  %s google.com\n", os.Args[0])
	fmt.Fprintf(os.Stderr, "  %s -maxhops 20 -protocol udp example.com\n", os.Args[0])
}

func main() {
	help := flag.Bool("h", false, "Show this help message")
	flag.BoolVar(help, "help", false, "Show this help message")

	protocol := flag.String("protocol", "tcp", "Protocol to use (tcp, udp)")
	firstHop := flag.Int("firsthop", 1, "First TTL value")
	maxHops := flag.Int("maxhops", 30, "Maximum TTL value")
	timeoutMs := flag.Int("timeout", 5000, "Timeout per probe in milliseconds")
	delayMs := flag.Int("delay", 100, "Delay between probes in milliseconds")
	retries := flag.Int("retries", 1, "Number of retries per probe")
	maxNoReply := flag.Int("maxnoreply", 5, "Max consecutive no-replies before stopping")

	flag.Usage = usage
	flag.Parse()

	if *help {
		flag.Usage()
		os.Exit(0)
	}

	if flag.NArg() == 0 {
		fmt.Fprintf(os.Stderr, "Error: target host is required\n")
		flag.Usage()
		os.Exit(1)
	}

	target := flag.Arg(0)

	opts := options.NewTracerouteOptions()

	switch *protocol {
	case "tcp":
		opts.SetProbeProtocol(options.PROTOCOL_TCP)
	case "udp":
		opts.SetProbeProtocol(options.PROTOCOL_UDP)
	default:
		fmt.Fprintf(os.Stderr, "Error: invalid protocol %q\n", *protocol)
		os.Exit(1)
	}

	opts.SetFirstHop(*firstHop)
	opts.SetMaxHops(*maxHops)
	opts.SetTimeoutMs(*timeoutMs)
	opts.SetDelayMs(*delayMs)
	opts.SetRetries(*retries)
	opts.SetMaxConsecutiveNoReplies(*maxNoReply)

	resChan := make(chan hop.TracerouteHop)
	errChan := make(chan error)

	go func() {
		if err := traceroute.Traceroute(target, opts, resChan); err != nil {
			errChan <- err
		}
	}()

	for {
		select {
		case hop, ok := <-resChan:
			if !ok {
				fmt.Println("Traceroute completed successfully")
				return
			}
			fmt.Printf("TTL %d\t%s\t%v\n", hop.TTL, hop.Address, hop.ElapsedTime)
		case err := <-errChan:
			log.Fatal(err)
		}
	}
}
