package cluster

import (
	"errors"
	"time"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/app/form"
	"aiprobe/internal/db"
	"aiprobe/internal/model"
)

func PostNodeLoginAdd(c *gin.Context) {
	var field form.ClusterNodeLoginAdd
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	if field.NodeID < 1 {
		common.ErrorResp(c, errors.New("add exception[node_id]!"), -1)
		return
	}

	common_data := &model.ClusterNodeLogin{
		Name:       "ssh",
		NodeID:     field.NodeID,
		UpdateTime: time.Now().Unix(),
	}

	common_data.SetParams(model.ClusterNodeLoginParams{
		Host:  field.Host,
		Port:  field.Port,
		SshID: field.SshID,
	})

	// 先查找是否存在该节点的登录记录；存在则更新，不存在则创建
	var existing model.ClusterNodeLogin
	err := db.GetDb().Where("node_id = ?", field.NodeID).First(&existing).Error
	if err == nil && existing.ID > 0 {
		if err := db.GetDb().Model(&model.ClusterNodeLogin{}).Where("id = ?", existing.ID).Updates(common_data).Error; err != nil {
			common.ErrorResp(c, err, -1)
			return
		}
		common.SuccessResp(c)
		return
	}
	common_data.Status = true
	common_data.CreateTime = time.Now().Unix()
	if err := db.GetDb().Create(common_data).Error; err != nil {
		common.ErrorResp(c, err, -1)
		return
	}
	common.SuccessResp(c)
}
