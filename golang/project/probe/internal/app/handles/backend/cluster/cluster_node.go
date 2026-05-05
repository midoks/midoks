package cluster

import (
	"encoding/json"
	"errors"
	"strconv"

	// "fmt"
	"net/http"
	// "strconv"
	"strings"
	"time"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/app/form"
	"aiprobe/internal/db"
	"aiprobe/internal/model"
	tools "aiprobe/internal/utils"
)

func parseClusterNodeIpArray(ipJson string) ([]form.ClusterNodeIpAddr, error) {
	if ipJson == "" {
		return nil, nil
	}
	var ipArray []form.ClusterNodeIpAddr
	if err := json.Unmarshal([]byte(ipJson), &ipArray); err != nil {
		return nil, errors.New("invalid ip_addresses_json format: " + ipJson)
	}
	return ipArray, nil
}

func syncClusterNodeIpAddresses(nodeID int64, ipArray []form.ClusterNodeIpAddr) error {
	if len(ipArray) == 0 {
		return nil
	}

	if err := db.ClusterNodeIpaddrSoftDeleteByNodeID(nodeID); err != nil {
		return err
	}

	for _, ipinfo := range ipArray {
		existing := db.ExistClusterNodeIpaddrByNodeIDAndIp(nodeID, ipinfo.Ip)
		if existing {
			updateData := map[string]interface{}{
				"description":      ipinfo.Description,
				"ip":               ipinfo.Ip,
				"can_access":       ipinfo.CanAccess,
				"can_health_check": ipinfo.CanHealthCheck,
				"is_healthy":       true,
				"is_on":            ipinfo.IsOn,
				"is_up":            true,
				"order":            1,
				"is_deleted":       0,
				"update_time":      time.Now().Unix(),
			}
			if err := db.GetDb().Unscoped().Model(&model.ClusterNodeIpaddr{}).Where("node_id = ? AND ip = ?", nodeID, ipinfo.Ip).Updates(updateData).Error; err != nil {
				return err
			}
		} else {
			common_ip_data := &model.ClusterNodeIpaddr{
				NodeID:         nodeID,
				Ip:             ipinfo.Ip,
				Description:    ipinfo.Description,
				CanAccess:      ipinfo.CanAccess,
				CanHealthCheck: ipinfo.CanHealthCheck,
				IsHealthy:      true,
				IsOn:           ipinfo.IsOn,
				IsUp:           true,
				Order:          1,
				IsDeleted:      0,
			}

			if deleteID, err := db.GetClusterNodeIpaddrDeletedID(); err == nil {
				updateData := map[string]interface{}{
					"node_id":          nodeID,
					"description":      ipinfo.Description,
					"ip":               ipinfo.Ip,
					"can_access":       ipinfo.CanAccess,
					"can_health_check": ipinfo.CanHealthCheck,
					"is_healthy":       true,
					"is_on":            ipinfo.IsOn,
					"is_up":            true,
					"order":            1,
					"is_deleted":       0,
					"update_time":      time.Now().Unix(),
				}
				if err := db.GetDb().Unscoped().Model(&model.ClusterNodeIpaddr{}).Where("id", deleteID).Updates(updateData).Error; err != nil {
					return err
				}
			} else {
				common_ip_data.CreateTime = time.Now().Unix()
				common_ip_data.UpdateTime = time.Now().Unix()
				if err := db.GetDb().Create(common_ip_data).Error; err != nil {
					return err
				}
			}
		}
	}
	return nil
}

func GetNodeSubMenu() []form.SubMenu {
	menu := []form.SubMenu{
		{
			Number: 1,
			Name:   "节点看板",
			Link:   "clusters/node/boards",
			Icon:   "layui-icon-home",
		},
		{
			Number: 2,
			Name:   "节点详情",
			Link:   "clusters/node/details",
			Icon:   "layui-icon-file",
		},
		{
			Number: 3,
			Name:   "运行日志",
			Link:   "clusters/node/logs",
			Icon:   "layui-icon-rss",
		},
		{
			Number: 4,
			Name:   "安装节点",
			Link:   "clusters/node/install",
			Icon:   "layui-icon-download-circle",
		},
		{
			Number: 5,
			Name:   "节点设置",
			Link:   "clusters/node/settings",
			Icon:   "layui-icon-set",
		},
	}
	return menu
}

func Node(c *gin.Context) {
	data := common.CommonVer(c)
	c.HTML(http.StatusOK, "backend/cluster/node/index.tmpl", data)
}

func NodeList(c *gin.Context) {
	var field form.ClusterNodeList
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	result, count, err := db.GetClusterNodeListByArgs(field)
	if err != nil {
		common.ErrorResp(c, err, -2)
		return
	}
	common.SuccessLayuiResp(c, count, "ok", result)
}

func CreateNode(c *gin.Context) {
	method := strings.ToUpper(c.Request.Method)
	if method == "POST" {
		PostCreateNode(c)
		return
	}
	data := common.CommonVer(c)
	data["submenu"] = GetSubMenu()
	data["cluster_id"] = c.Query("cluster_id")
	c.HTML(http.StatusOK, "backend/cluster/node/create.tmpl", data)
}

func NodeBoards(c *gin.Context) {
	data := common.CommonVer(c)
	data["submenu"] = GetNodeSubMenu()
	data["node_id"] = c.Query("node_id")
	data["cluster_id"] = c.Query("cluster_id")
	c.HTML(http.StatusOK, "backend/cluster/node/boards.tmpl", data)
}

func NodeDatail(c *gin.Context) {
	data := common.CommonVer(c)
	data["submenu"] = GetNodeSubMenu()
	data["node_id"] = c.Query("node_id")
	data["cluster_id"] = c.Query("cluster_id")

	node_id := c.Query("node_id")
	node_idint, _ := strconv.ParseInt(node_id, 10, 64)
	node_data, _ := db.GetClusterNodeByID(node_idint)
	data["Data"] = node_data

	c.HTML(http.StatusOK, "backend/cluster/node/details.tmpl", data)
}

func PostCreateNode(c *gin.Context) {
	var field form.ClusterCreateNode
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	if field.Name == "" {
		common.ErrorResp(c, errors.New("节点名称不能空!"), -1)
		return
	}

	secret := tools.RandString(32)
	unique_id := tools.RandString(32)

	nodeip := &model.ClusterNode{
		Name:        field.Name,
		ClusterID:   field.ClusterID,
		IsInstalled: false,
		Secret:      secret,
		UniqueID:    unique_id,
		CreateTime:  time.Now().Unix(),
		UpdateTime:  time.Now().Unix(),
	}

	if err := db.GetDb().Create(nodeip).Error; err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	if field.IpAddressesJson != "" {
		ipArray, err := parseClusterNodeIpArray(field.IpAddressesJson)
		if err != nil {
			common.ErrorResp(c, err, -1)
			return
		}

		if err := syncClusterNodeIpAddresses(nodeip.ID, ipArray); err != nil {
			common.ErrorResp(c, err, -1)
			return
		}
	}

	common.SuccessResp(c, map[string]interface{}{"id": nodeip.ID})
}

func PostDeleteNode(c *gin.Context) {
	var field form.ID
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	err := db.ClusterNodeDeleteByID(nil, field.ID)
	if err == nil {
		common.SuccessResp(c)
		return
	}
	common.ErrorResp(c, err, -1)

}
