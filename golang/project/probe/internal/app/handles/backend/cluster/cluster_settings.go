package cluster

import (
	"errors"
	"net/http"
	"strconv"
	"time"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/app/form"
	"aiprobe/internal/db"
	"aiprobe/internal/model"
	// utils "aiprobe/internal/utils"
)

func GetSettingSubMenu() []form.SubSettingMenu {
	menu := []form.SubSettingMenu{
		{
			Number: 1,
			Name:   "基础设置",
			Link:   "clusters/cluster/settings",
			Type:   "a",
		},
		{
			Number: 2,
			Name:   "line",
			Link:   "",
			Type:   "line",
		},
		{
			Number: 3,
			Name:   "健康检查",
			Link:   "clusters/cluster/settings/health",
			Type:   "a",
		},
	}
	return menu
}

func ClusterSettings(c *gin.Context) {
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

func PostClusterSettings(c *gin.Context) {
	var field form.ClusterSettings
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	if field.Name == "" {
		common.ErrorResp(c, errors.New("name cannot be empty!"), -1)
		return
	}

	common_data := &model.Cluster{
		Name:       field.Name,
		UpdateTime: time.Now().Unix(),
	}

	if err := db.GetDb().Model(&model.Cluster{}).Where("id = ?", field.ID).Updates(common_data).Error; err != nil {
		common.ErrorResp(c, err, -1)
		return
	}
	common.SuccessResp(c)
}
