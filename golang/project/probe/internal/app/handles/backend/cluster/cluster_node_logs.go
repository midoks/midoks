package cluster

import (
	"net/http"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/app/form"
	"aiprobe/internal/db"
)

func NodeLogs(c *gin.Context) {
	data := common.CommonVer(c)
	data["submenu"] = GetNodeSubMenu()
	data["node_id"] = c.Query("node_id")
	data["cluster_id"] = c.Query("cluster_id")
	c.HTML(http.StatusOK, "backend/cluster/node/logs.tmpl", data)
}

func NodeLogsList(c *gin.Context) {
	var field form.ClusterNodeQuery
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}
	result, count, err := db.GetClusterNodeLogsListByID(field.ID, field.Page.Page, field.Page.Limit)
	if err != nil {
		common.ErrorResp(c, err, -2)
		return
	}
	common.SuccessLayuiResp(c, count, "ok", result)
}
