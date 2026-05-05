package cluster

import (
	"github.com/gin-gonic/gin"
	"net/http"
	"strconv"

	"aiprobe/internal/app/common"
	"aiprobe/internal/db"
)

func ClusterSettingsHealth(c *gin.Context) {
	cluster_id := c.Query("cluster_id")
	data := common.CommonVer(c)
	data["submenu"] = GetSubMenu()
	data["setting_menu"] = GetSettingSubMenu()
	data["cluster_id"] = cluster_id

	cluster_idint, _ := strconv.ParseInt(cluster_id, 10, 64)
	cluster_data, _ := db.GetClusterByID(cluster_idint)

	data["Data"] = cluster_data
	c.HTML(http.StatusOK, "backend/cluster/settings/index.tmpl", data)
}
