package cluster

import (
	"encoding/json"
	"errors"
	"net/http"
	"strconv"
	"time"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/app/form"
	"aiprobe/internal/db"
	"aiprobe/internal/model"
)

func GetNodeSettingSubMenu() []form.SubSettingMenu {
	menu := []form.SubSettingMenu{
		{
			Number: 1,
			Name:   "基础设置",
			Link:   "clusters/node/settings",
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
			Name:   "SSH设置",
			Link:   "clusters/node/settings/ssh",
			Type:   "a",
		},
	}
	return menu
}

func NodeSettings(c *gin.Context) {
	node_id := c.Query("node_id")

	data := common.CommonVer(c)
	data["submenu"] = GetNodeSubMenu()
	data["setting_menu"] = GetNodeSettingSubMenu()
	data["node_id"] = node_id
	data["cluster_id"] = c.Query("cluster_id")

	node_idint, _ := strconv.ParseInt(node_id, 10, 64)
	node_data, _ := db.GetClusterNodeByID(node_idint)
	data["Data"] = node_data

	ipaddrs, err := db.GetClusterNodeIpaddrByNodeID(node_idint)
	data["IpAddressesJson"] = "[]"
	if err == nil {
		ipaddrs_json, err := json.Marshal(ipaddrs)
		if err == nil {
			data["IpAddressesJson"] = string(ipaddrs_json)
		}
	}

	c.HTML(http.StatusOK, "backend/cluster/node/settings.tmpl", data)
}

func PostNodeSettings(c *gin.Context) {
	var field form.ClusterNodeSettings
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	// IpAddressesJson
	var ipArray []form.ClusterNodeIpAddr
	if field.IpAddressesJson != "" {
		var err error
		ipArray, err = parseClusterNodeIpArray(field.IpAddressesJson)
		if err != nil {
			common.ErrorResp(c, err, -1)
			return
		}

		if err := syncClusterNodeIpAddresses(field.ID, ipArray); err != nil {
			common.ErrorResp(c, err, -1)
			return
		}
	}

	if field.Name == "" {
		common.ErrorResp(c, errors.New("节点名称不能空!"), -1)
		return
	}

	common_data := &model.ClusterNode{
		Name:       field.Name,
		UpdateTime: time.Now().Unix(),
	}

	if err := db.GetDb().Model(&model.ClusterNode{}).Where("id = ?", field.ID).Updates(common_data).Error; err != nil {
		common.ErrorResp(c, err, -1)
		return
	}
	common.SuccessResp(c)
}
