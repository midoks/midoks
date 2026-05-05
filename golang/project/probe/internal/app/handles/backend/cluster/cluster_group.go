package cluster

import (
	"fmt"
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/app/form"
	"aiprobe/internal/db"
)

func ClusterGroups(c *gin.Context) {
	data := common.CommonVer(c)
	data["submenu"] = GetSubMenu()
	data["cluster_id"] = c.Query("cluster_id")
	c.HTML(http.StatusOK, "backend/cluster/groups/index.tmpl", data)
}

func ClusterGroupsAdd(c *gin.Context) {
	data := common.CommonVer(c)
	data["id"] = c.Query("id")
	data["submenu"] = GetSubMenu()
	data["cluster_id"] = c.Query("cluster_id")

	if data["id"] != "" {
		qid, err := strconv.ParseInt(data["id"].(string), 10, 64)
		if err != nil {

		}

		cg_data, err := db.GetClusterGroupByID(qid)
		fmt.Println(qid, cg_data, err)
		if err != nil {

		}
		data["data"] = cg_data

		fmt.Println(cg_data)
	}
	c.HTML(http.StatusOK, "backend/cluster/groups/add.tmpl", data)
}

func ClusterGroupsList(c *gin.Context) {
	result, count, _ := db.GetClusterGroupList(1, 10)
	common.SuccessLayuiResp(c, count, "ok", result)
}

func PostClusterGroupsAdd(c *gin.Context) {
	var field form.ClusterGroupAdd
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, 0)
		return
	}

	if field.ID != "" {
		id, _ := strconv.ParseInt(field.ID, 10, 64)
		err := db.UpdateClusterGroup(nil, field.Name, id)
		if err == nil {
			common.SuccessResp(c)
			return
		}
		common.ErrorResp(c, err, 0)
		return
	}

	err := db.AddClusterGroup(nil, field.Name, field.ClusterID)
	if err == nil {
		common.SuccessResp(c)
		return
	}
	common.ErrorResp(c, err, 0)
	return
}

func ClusterGroupsDelete(c *gin.Context) {
	var field form.ID
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	err := db.ClusterGroupDeleteByID(nil, field.ID)
	if err == nil {
		common.SuccessResp(c)
		return
	}
	common.ErrorResp(c, err, -1)
}
