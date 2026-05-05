package cluster

import (
	// "fmt"

	"net/http"
	"strconv"

	"aiprobe/internal/app/common"
	"aiprobe/internal/app/form"
	"aiprobe/internal/db"

	"github.com/gin-gonic/gin"
)

func ClusterRegions(c *gin.Context) {
	data := common.CommonVer(c)
	c.HTML(http.StatusOK, "backend/cluster/regions/index.tmpl", data)
}

func ClusterRegionsAdd(c *gin.Context) {
	data := common.CommonVer(c)
	data["id"] = c.Query("id")
	if data["id"] != "" {
		qid, err := strconv.ParseInt(data["id"].(string), 10, 64)
		if err != nil {

		}

		cg_data, err := db.GetClusterRegionByID(qid)
		if err != nil {

		}
		data["data"] = cg_data
	}

	c.HTML(http.StatusOK, "backend/cluster/regions/add.tmpl", data)
}

func ClusterRegionsNodes(c *gin.Context) {
	data := common.CommonVer(c)
	c.HTML(http.StatusOK, "backend/cluster/regions/nodes.tmpl", data)
}

func ClusterRegionsList(c *gin.Context) {
	var field form.Page
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	result, count, _ := db.GetClusterRegionList(field.Page, field.Limit)
	common.SuccessLayuiResp(c, count, "ok", result)
}

func PostClusterRegionsNodesAdd(c *gin.Context) {
	var field form.ClusterRegionAdd
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, 0)
		return
	}

	if field.ID != "" {
		id, _ := strconv.ParseInt(field.ID, 10, 64)
		err := db.UpdateClusterRegion(nil, field.Name, field.Mark, id)
		if err == nil {
			common.SuccessResp(c)
			return
		}
		common.ErrorResp(c, err, 0)
		return
	}

	err := db.AddClusterRegion(nil, field.Name, field.Mark)
	if err == nil {
		common.SuccessResp(c)
		return
	}
	common.ErrorResp(c, err, 0)
	return
}

func ClusterRegionsDelete(c *gin.Context) {
	var field form.ID
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	err := db.ClusterRegionDeleteByID(nil, field.ID)
	if err == nil {
		common.SuccessResp(c)
		return
	}
	common.ErrorResp(c, err, -1)
}

func ClusterRegionsTriggerStatus(c *gin.Context) {
	var field form.ID
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	err := db.ClusterRegionsTriggerStatus(nil, field.ID)
	if err == nil {
		common.SuccessResp(c)
		return
	}
	common.ErrorResp(c, err, -1)
}
