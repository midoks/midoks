package frontend

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func Ping(c *gin.Context) {
	data := map[string]interface{}{
		"title": "在线Ping - AIProbe",
	}
	c.HTML(http.StatusOK, "frontend/ping/index.tmpl", data)
}

func Tcping(c *gin.Context) {
	data := map[string]interface{}{
		"title": "在线Tcping - AIProbe",
	}
	c.HTML(http.StatusOK, "frontend/tcping/index.tmpl", data)
}

func Speedtest(c *gin.Context) {
	data := map[string]interface{}{
		"title": "网站测速 - AIProbe",
	}
	c.HTML(http.StatusOK, "frontend/speedtest/index.tmpl", data)
}

func Traceroute(c *gin.Context) {
	data := map[string]interface{}{
		"title": "路由追踪 - AIProbe",
	}
	c.HTML(http.StatusOK, "frontend/traceroute/index.tmpl", data)
}

func DNS(c *gin.Context) {
	data := map[string]interface{}{
		"title": "DNS查询 - AIProbe",
	}
	c.HTML(http.StatusOK, "frontend/dns/index.tmpl", data)
}

func FindPing(c *gin.Context) {
	data := map[string]interface{}{
		"title": "FindPing - AIProbe",
	}
	c.HTML(http.StatusOK, "frontend/findping/index.tmpl", data)
}

func Network(c *gin.Context) {
	data := map[string]interface{}{
		"title": "本地网络 - AIProbe",
	}
	c.HTML(http.StatusOK, "frontend/network/index.tmpl", data)
}

func Batch(c *gin.Context) {
	data := map[string]interface{}{
		"title": "批量检测 - AIProbe",
	}
	c.HTML(http.StatusOK, "frontend/batch/index.tmpl", data)
}

func Help(c *gin.Context) {
	data := map[string]interface{}{
		"title": "帮助支持 - AIProbe",
	}
	c.HTML(http.StatusOK, "frontend/help/index.tmpl", data)
}

func Settings(c *gin.Context) {
	data := map[string]interface{}{
		"title": "习惯设置 - AIProbe",
	}
	c.HTML(http.StatusOK, "frontend/settings/index.tmpl", data)
}
