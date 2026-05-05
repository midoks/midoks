package api_tools

import (
	"fmt"
	"net"
	"net/http"
	"os"
	"os/exec"
	"regexp"
	"strings"
	"time"

	"aiprobe/internal/app/common"

	"github.com/gin-gonic/gin"
)

func Ping(c *gin.Context) {
	host := strings.TrimSpace(c.PostForm("host"))
	if host == "" {
		common.ErrorStrResp(c, "请输入主机地址", 1)
		return
	}

	cmd := exec.Command("ping", "-c", "4", "-W", "5", host)
	output, err := cmd.CombinedOutput()
	if err != nil {
		common.ErrorStrResp(c, fmt.Sprintf("Ping失败: %v", err), 1)
		return
	}

	common.SuccessResp(c, string(output))
}

func Tcping(c *gin.Context) {
	host := strings.TrimSpace(c.PostForm("host"))
	port := strings.TrimSpace(c.PostForm("port"))

	if host == "" {
		common.ErrorStrResp(c, "请输入主机地址", 1)
		return
	}
	if port == "" {
		port = "80"
	}

	start := time.Now()
	conn, err := net.DialTimeout("tcp", net.JoinHostPort(host, port), 5*time.Second)
	if err != nil {
		common.ErrorStrResp(c, fmt.Sprintf("连接失败: %v", err), 1)
		return
	}
	defer conn.Close()

	elapsed := time.Since(start)
	result := fmt.Sprintf("连接成功到 %s:%s，耗时: %v", host, port, elapsed)
	common.SuccessResp(c, result)
}

func Speedtest(c *gin.Context) {
	url := strings.TrimSpace(c.PostForm("url"))
	if url == "" {
		common.ErrorStrResp(c, "请输入网站URL", 1)
		return
	}

	if !strings.HasPrefix(url, "http://") && !strings.HasPrefix(url, "https://") {
		url = "https://" + url
	}

	var dnsTime, connectTime, totalTime int64
	var statusCode int
	var contentLength int64
	var server string

	start := time.Now()

	client := &http.Client{
		Timeout: 30 * time.Second,
	}

	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		common.ErrorStrResp(c, fmt.Sprintf("请求创建失败: %v", err), 1)
		return
	}

	dnsStart := time.Now()
	ips, err := net.LookupHost(strings.Split(strings.Replace(url, "https://", "", 1), "/")[0])
	if err == nil && len(ips) > 0 {
		dnsTime = time.Since(dnsStart).Milliseconds()
	}

	connectStart := time.Now()
	resp, err := client.Do(req)
	if err != nil {
		common.ErrorStrResp(c, fmt.Sprintf("请求失败: %v", err), 1)
		return
	}
	defer resp.Body.Close()

	connectTime = time.Since(connectStart).Milliseconds()
	totalTime = time.Since(start).Milliseconds()
	statusCode = resp.StatusCode
	contentLength = resp.ContentLength
	server = resp.Header.Get("Server")
	if server == "" {
		server = "Unknown"
	}

	result := map[string]interface{}{
		"dns":            dnsTime,
		"connect":        connectTime,
		"total":          totalTime,
		"status_code":    statusCode,
		"content_length": contentLength,
		"server":         server,
	}

	common.SuccessResp(c, result)
}

func Traceroute(c *gin.Context) {
	host := strings.TrimSpace(c.PostForm("host"))
	if host == "" {
		common.ErrorStrResp(c, "请输入目标主机", 1)
		return
	}

	cmd := exec.Command("traceroute", "-m", "30", "-w", "2", host)
	output, err := cmd.CombinedOutput()
	if err != nil {
		common.ErrorStrResp(c, fmt.Sprintf("路由追踪失败: %v", err), 1)
		return
	}

	common.SuccessResp(c, string(output))
}

func DNSQuery(c *gin.Context) {
	domain := strings.TrimSpace(c.PostForm("domain"))
	recordType := strings.TrimSpace(c.PostForm("type"))

	if domain == "" {
		common.ErrorStrResp(c, "请输入域名", 1)
		return
	}
	if recordType == "" {
		recordType = "A"
	}

	var records []string

	switch strings.ToUpper(recordType) {
	case "A":
		ips, err := net.LookupIP(domain)
		if err != nil {
			common.ErrorStrResp(c, fmt.Sprintf("DNS查询失败: %v", err), 1)
			return
		}
		for _, ip := range ips {
			if ip.To4() != nil {
				records = append(records, ip.String())
			}
		}
	case "AAAA":
		ips, err := net.LookupIP(domain)
		if err != nil {
			common.ErrorStrResp(c, fmt.Sprintf("DNS查询失败: %v", err), 1)
			return
		}
		for _, ip := range ips {
			if ip.To4() == nil {
				records = append(records, ip.String())
			}
		}
	case "MX":
		mxRecords, err := net.LookupMX(domain)
		if err != nil {
			common.ErrorStrResp(c, fmt.Sprintf("DNS查询失败: %v", err), 1)
			return
		}
		for _, mx := range mxRecords {
			records = append(records, fmt.Sprintf("%d %s", mx.Pref, mx.Host))
		}
	case "NS":
		nsRecords, err := net.LookupNS(domain)
		if err != nil {
			common.ErrorStrResp(c, fmt.Sprintf("DNS查询失败: %v", err), 1)
			return
		}
		for _, ns := range nsRecords {
			records = append(records, ns.Host)
		}
	case "TXT":
		txtRecords, err := net.LookupTXT(domain)
		if err != nil {
			common.ErrorStrResp(c, fmt.Sprintf("DNS查询失败: %v", err), 1)
			return
		}
		records = txtRecords
	case "CNAME":
		cname, err := net.LookupCNAME(domain)
		if err != nil {
			common.ErrorStrResp(c, fmt.Sprintf("DNS查询失败: %v", err), 1)
			return
		}
		records = append(records, cname)
	default:
		common.ErrorStrResp(c, "不支持的记录类型", 1)
		return
	}

	if len(records) == 0 {
		records = append(records, "未找到相关记录")
	}

	common.SuccessResp(c, records)
}

func FindPing(c *gin.Context) {
	hostsStr := strings.TrimSpace(c.PostForm("hosts"))
	if hostsStr == "" {
		common.ErrorStrResp(c, "请输入IP地址或域名", 1)
		return
	}

	hosts := strings.Split(hostsStr, "\n")
	var results []string

	for _, host := range hosts {
		host = strings.TrimSpace(host)
		if host == "" {
			continue
		}

		cmd := exec.Command("ping", "-c", "2", "-W", "3", host)
		output, err := cmd.CombinedOutput()

		if err != nil {
			results = append(results, fmt.Sprintf("%s: 无法到达", host))
		} else {
			outputStr := string(output)
			re := regexp.MustCompile(`time=([0-9.]+) ms`)
			matches := re.FindAllStringSubmatch(outputStr, -1)
			if len(matches) > 0 {
				results = append(results, fmt.Sprintf("%s: OK (延迟: %s)", host, matches[0][1]))
			} else {
				results = append(results, fmt.Sprintf("%s: OK", host))
			}
		}
	}

	common.SuccessResp(c, strings.Join(results, "\n"))
}

func NetworkInfo(c *gin.Context) {
	result := map[string]interface{}{}

	ifaces, err := net.Interfaces()
	if err == nil {
		for _, iface := range ifaces {
			if iface.Flags&net.FlagUp == 0 || iface.Flags&net.FlagLoopback != 0 {
				continue
			}
			addrs, err := iface.Addrs()
			if err == nil {
				for _, addr := range addrs {
					if ipNet, ok := addr.(*net.IPNet); ok && ipNet.IP.To4() != nil {
						if ipNet.IP.String() != "127.0.0.1" {
							result["local_ip"] = ipNet.IP.String()
							break
						}
					}
				}
			}
			if _, ok := result["local_ip"]; ok {
				break
			}
		}
	}

	hostname, _ := os.Hostname()
	result["hostname"] = hostname
	result["dns"] = "8.8.8.8, 114.114.114.114"

	common.SuccessResp(c, result)
}

type BatchHostResult struct {
	Host     string `json:"host"`
	Ping     string `json:"ping"`
	PingOK   bool   `json:"ping_ok"`
	Tcping   string `json:"tcping"`
	TcpingOK bool   `json:"tcping_ok"`
	DNS      string `json:"dns"`
	HTTP     string `json:"http"`
	HTTPOK   bool   `json:"http_ok"`
}

func BatchCheck(c *gin.Context) {
	hostsStr := strings.TrimSpace(c.PostForm("hosts"))
	if hostsStr == "" {
		common.ErrorStrResp(c, "请输入主机列表", 1)
		return
	}

	hosts := strings.Split(hostsStr, "\n")
	var results []BatchHostResult

	for _, hostLine := range hosts {
		hostLine = strings.TrimSpace(hostLine)
		if hostLine == "" {
			continue
		}

		result := BatchHostResult{Host: hostLine}

		host := hostLine
		port := "80"

		if strings.Contains(hostLine, ":") {
			parts := strings.Split(hostLine, ":")
			host = parts[0]
			port = parts[1]
		}

		cmd := exec.Command("ping", "-c", "1", "-W", "2", host)
		output, err := cmd.CombinedOutput()
		if err != nil {
			result.Ping = "失败"
			result.PingOK = false
		} else {
			outputStr := string(output)
			re := regexp.MustCompile(`time=([0-9.]+) ms`)
			matches := re.FindStringSubmatch(outputStr)
			if len(matches) > 0 {
				result.Ping = matches[1] + "ms"
				result.PingOK = true
			} else {
				result.Ping = "成功"
				result.PingOK = true
			}
		}

		conn, err := net.DialTimeout("tcp", net.JoinHostPort(host, port), 3*time.Second)
		if err != nil {
			result.Tcping = "失败"
			result.TcpingOK = false
		} else {
			conn.Close()
			result.Tcping = "成功"
			result.TcpingOK = true
		}

		results = append(results, result)
	}

	common.SuccessResp(c, results)
}
