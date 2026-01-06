package main

import (
	"bufio"
	"context"
	"flag"
	"fmt"
	"net"
	"os"
	"strings"
	"time"
)

var (
	timeoutSec = flag.Int("timeout", 5, "lookup timeout in seconds per host")
)

type ipClass string

const (
	classGUA        ipClass = "GUA"
	classULA        ipClass = "ULA"
	classLinkLocal  ipClass = "LinkLocal"
	classMulticast  ipClass = "Multicast"
	classLoopback   ipClass = "Loopback"
	classUnspecified ipClass = "Unspecified"
	classDoc        ipClass = "Documentation"
	classOther      ipClass = "Other"
)

func classifyIPv6(ip net.IP) ipClass {
	// Ensure it's 16-byte IPv6
	ip = ip.To16()
	if ip == nil || ip.To4() != nil {
		return classOther
	}

	// Define known ranges
	var (
		_, netGUA, _       = net.ParseCIDR("2000::/3")    // global unicast
		_, netULA, _       = net.ParseCIDR("fc00::/7")    // unique local
		_, netLinkLocal, _ = net.ParseCIDR("fe80::/10")   // link-local
		_, netMulticast, _ = net.ParseCIDR("ff00::/8")    // multicast
		_, netDoc, _       = net.ParseCIDR("2001:db8::/32") // documentation
		loopback           = net.ParseIP("::1")
		unspecified        = net.ParseIP("::")
	)

	switch {
	case ip.Equal(unspecified):
		return classUnspecified
	case ip.Equal(loopback):
		return classLoopback
	case netMulticast.Contains(ip):
		return classMulticast
	case netLinkLocal.Contains(ip):
		return classLinkLocal
	case netULA.Contains(ip):
		return classULA
	case netDoc.Contains(ip):
		return classDoc
	case netGUA.Contains(ip):
		return classGUA
	default:
		return classOther
	}
}

func processLine(line string, resolver *net.Resolver, timeout time.Duration) (string, []string, error) {
	name := strings.TrimSpace(line)
	if name == "" {
		return "", nil, nil
	}
	// ignore comments starting with #
	if strings.HasPrefix(name, "#") {
		return "", nil, nil
	}

	ctx, cancel := context.WithTimeout(context.Background(), timeout)
	defer cancel()

	// Use LookupIPAddr so we can control via context
	addrs, err := resolver.LookupIPAddr(ctx, name)
	if err != nil {
		return name, nil, err
	}

	var results []string
	for _, a := range addrs {
		ip := a.IP
		// skip IPv4
		if ip.To4() != nil {
			continue
		}
		cls := classifyIPv6(ip)
		results = append(results, fmt.Sprintf("%s (%s)", ip.String(), cls))
	}

	return name, results, nil
}

func main() {
	flag.Parse()
	if flag.NArg() < 1 {
		fmt.Fprintf(os.Stderr, "usage: %s <input-file>\n", os.Args[0])
		flag.PrintDefaults()
		os.Exit(2)
	}

	filename := flag.Arg(0)
	f, err := os.Open(filename)
	if err != nil {
		fmt.Fprintf(os.Stderr, "error opening %s: %v\n", filename, err)
		os.Exit(1)
	}
	defer f.Close()

	timeout := time.Duration(*timeoutSec) * time.Second
	resolver := net.DefaultResolver

	scanner := bufio.NewScanner(f)
	lineno := 0
	for scanner.Scan() {
		lineno++
		line := scanner.Text()
		name, results, err := processLine(line, resolver, timeout)
		if name == "" && len(results) == 0 && err == nil {
			// blank or comment
			continue
		}
		if err != nil {
			fmt.Printf("%d\t%s\tERROR: %v\n", lineno, strings.TrimSpace(line), err)
			continue
		}
		if len(results) == 0 {
			fmt.Printf("%d\t%s\t(no AAAA records)\n", lineno, name)
			continue
		}
		// Print one line per hostname with comma-separated IPs + classes
		fmt.Printf("%d\t%s\t%s\n", lineno, name, strings.Join(results, ", "))
	}

	if serr := scanner.Err(); serr != nil {
		fmt.Fprintf(os.Stderr, "error reading file: %v\n", serr)
		os.Exit(1)
	}
}
