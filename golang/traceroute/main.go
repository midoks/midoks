package main

import (
	"fmt"
	"log"

	"github.com/0ne-zero/goTraceroute/pkg/core/hop"
	"github.com/0ne-zero/goTraceroute/pkg/core/options"
	"github.com/0ne-zero/goTraceroute/pkg/core/traceroute"
)

func main() {
	// Create traceroute options with defaults (max hops, timeouts, etc.)
	opts := options.NewTracerouteOptions()

	// Customize options as needed, (they've default value)
	opts.SetProbeProtocol(options.PROTOCOL_TCP) // Use TCP instead of default UDP
	opts.SetFirstHop(1)                         // Start from TTL=1
	opts.SetMaxHops(30)                         // Limit max TTL to 30 hops
	opts.SetTimeoutMs(5000)                     // 2 seconds timeout per probe
	opts.SetDelayMs(100)                        // 100 ms delay between probes
	opts.SetRetries(1)                          // Retry once if probe fails
	opts.SetMaxConsecutiveNoReplies(5)          // Stop early if 5 consecutive TTL probes get no replies (no ICMP or TCP response)

	// --- Synchronous traceroute ---

	// We pass a buffered channel sized to max hops, so traceroute can send results without blocking
	resChan := make(chan hop.TracerouteHop, opts.MaxHops())

	// Start traceroute and wait until it completes
	if err := traceroute.Traceroute("google.com", opts, resChan); err != nil {
		log.Fatal(err)
	}

	// Read results from the channel
	for h := range resChan {
		fmt.Printf("TTL %d\t%s\t%v\n", h.TTL, h.Address, h.ElapsedTime)
	}

	// --- Asynchronous traceroute ---

	// Useful if you want to show hops live or do other work concurrently
	resChan = make(chan hop.TracerouteHop)
	errChan := make(chan error)

	// Run traceroute in a separate goroutine, send any error to errChan
	go func() {
		if err := traceroute.Traceroute("google.com", opts, resChan); err != nil {
			errChan <- err
		}
	}()

	// Collect results as they arrive, or exit if an error occurs
	for {
		select {
		case hop, ok := <-resChan:
			if !ok {
				fmt.Println("Traceroute completed successfully")
				return // Channel closed → traceroute finished
			}
			fmt.Printf("TTL %d\t%s\t%v\n", hop.TTL, hop.Address, hop.ElapsedTime)
		case err := <-errChan:
			log.Fatal(err)
		}
	}
}