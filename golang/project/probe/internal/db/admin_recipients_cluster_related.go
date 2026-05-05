package db

import (
	"fmt"
	"time"

	"github.com/pkg/errors"
	"gorm.io/gorm"

	"aiprobe/internal/model"
)

func GetAdminRecipientsClusterRelatedByRecipientID(recipient_id int64) ([]model.AdminRecipientsClusterRelated, error) {
	recipientIDStr := fmt.Sprintf("%d", recipient_id)
	var relations []model.AdminRecipientsClusterRelated
	if err := db.Where("recipient_id = ?", recipientIDStr).Where("status", true).Find(&relations).Error; err != nil {
		return nil, errors.Wrap(err, "查询失败")
	}
	return relations, nil
}

func UpdateAdminRecipientsClusterRelated(tx *gorm.DB, recipient_id int64, cluster_id []int64) (bool, error) {
	if tx == nil {
		tx = db
	}

	// 首先根据 recipient_id 查出数据库中已有的 cluster_id 列表
	var existingRelations []model.AdminRecipientsClusterRelated
	recipientIDStr := fmt.Sprintf("%d", recipient_id)
	if err := tx.Where("recipient_id = ?", recipientIDStr).Find(&existingRelations).Error; err != nil {
		return false, errors.Wrap(err, "查询现有关联失败")
	}

	// 构建现有 cluster_id 的映射，方便快速查找
	existingClusterMap := make(map[int64]model.AdminRecipientsClusterRelated)
	for _, relation := range existingRelations {
		existingClusterMap[relation.ClusterID] = relation
	}

	// 构建传入 cluster_id 的映射，方便快速查找
	inputClusterMap := make(map[int64]bool)
	for _, cid := range cluster_id {
		inputClusterMap[cid] = true
	}

	// 处理传入的 cluster_id
	for _, cid := range cluster_id {
		if relation, exists := existingClusterMap[cid]; exists {
			// 数据库中存在，将 status 设置为 1
			if err := tx.Model(&relation).Update("status", true).Error; err != nil {
				return false, errors.Wrap(err, "更新关联状态失败")
			}
		} else {
			// 数据库中不存在，添加新记录
			newRelation := model.AdminRecipientsClusterRelated{
				RecipientID: recipientIDStr,
				ClusterID:   cid,
				Status:      true,
				CreateTime:  time.Now().Unix(),
				UpdateTime:  time.Now().Unix(),
			}
			if err := tx.Create(&newRelation).Error; err != nil {
				return false, errors.Wrap(err, "创建新关联失败")
			}
		}
	}

	// 处理数据库中存在但传入列表中不存在的 cluster_id
	for cid, relation := range existingClusterMap {
		if !inputClusterMap[cid] {
			// 数据库中存在但传入列表中不存在，将 status 设置为 0
			if err := tx.Model(&relation).Update("status", false).Error; err != nil {
				return false, errors.Wrap(err, "更新关联状态失败")
			}
		}
	}

	return true, nil
}
