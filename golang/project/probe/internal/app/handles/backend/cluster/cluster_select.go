package cluster

import (
	"net/http"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/db"
)

func SelectIp(c *gin.Context) {
	data := common.CommonVer(c)
	region_list, _, _ := db.GetClusterRegionList(1, 100)
	data["region_list"] = region_list
	c.HTML(http.StatusOK, "backend/cluster/select/ip.tmpl", data)
}

func SelectRegion(c *gin.Context) {
	data := common.CommonVer(c)
	region_list, _, _ := db.GetClusterRegionList(1, 100)
	data["region_list"] = region_list
	c.HTML(http.StatusOK, "backend/cluster/select/region.tmpl", data)
}

func SelectGroups(c *gin.Context) {
	data := common.CommonVer(c)
	region_list, _, _ := db.GetClusterGroupList(1, 100)
	data["groups_list"] = region_list
	c.HTML(http.StatusOK, "backend/cluster/select/groups.tmpl", data)
}

func SelectSsh(c *gin.Context) {
	data := common.CommonVer(c)
	data["cluster_id"] = c.Query("cluster_id")
	c.HTML(http.StatusOK, "backend/cluster/select/ssh.tmpl", data)
}
