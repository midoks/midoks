package tools

import (
	"io"
	"net"
	"net/http"
)

func GetPublicIP() (ip string, err error) {
	// - http://myexternalip.com/raw
	// - http://ip.dhcp.cn/?ip
	// - https://www.bt.cn/Api/getIpAddress
	resp, err := http.Get("http://myexternalip.com/raw")
	content, err := io.ReadAll(resp.Body)
	if err == nil {
		return string(content), nil
	}
	return "127.0.0.1", err
}

// 获取所有网络接口的 IP 地址
func GetAllIPs() ([]string, error) {
	var ips []string

	interfaces, err := net.Interfaces()
	if err != nil {
		return nil, err
	}

	for _, iface := range interfaces {
		// 跳过回环接口和未启用的接口
		if iface.Flags&net.FlagUp == 0 || iface.Flags&net.FlagLoopback != 0 {
			continue
		}

		addrs, err := iface.Addrs()
		if err != nil {
			continue
		}

		for _, addr := range addrs {
			var ip net.IP
			switch v := addr.(type) {
			case *net.IPNet:
				ip = v.IP
			case *net.IPAddr:
				ip = v.IP
			}

			// 跳过 IPv6 地址和非全局单播地址
			if ip == nil || ip.IsLoopback() || ip.IsLinkLocalUnicast() {
				continue
			}

			// 转换为 IPv4
			if ipv4 := ip.To4(); ipv4 != nil {
				ips = append(ips, ipv4.String())
			}
		}
	}

	return ips, nil
}
