package db

import (
	"errors"
	"time"

	"aiprobe/internal/model"
)

func GetClusterNodeIpaddrByNodeID(node_id int64) ([]model.ClusterNodeIpaddr, error) {
	var data []model.ClusterNodeIpaddr
	if err := db.Order(columnName("id")).Where("node_id", node_id).Where("is_deleted", 0).Find(&data).Error; err != nil {
		return nil, err
	}
	return data, nil
}

// 根据创建时间排序，获取一条删除的监控ID (重复使用)
func GetClusterNodeIpaddrDeletedID() (int64, error) {
	var data model.ClusterNodeIpaddr
	if err := db.Order(columnName("create_time")).Where("is_deleted=?", 1).First(&data).Error; err != nil {
		return 0, errors.New("failed get cluster node ipaddr deleted data: " + err.Error())
	}
	return data.ID, nil
}

func FindClusterNodeIpaddrByNodeIDAndIp(node_id int64, ip string) (*model.ClusterNodeIpaddr, error) {
	var data model.ClusterNodeIpaddr
	if err := db.Order(columnName("id")).Where("node_id", node_id).Where("ip", ip).First(&data).Error; err != nil {
		return nil, err
	}
	return &data, nil
}

func ExistClusterNodeIpaddrByNodeIDAndIp(node_id int64, ip string) bool {
	// 查询是否存在（包括已软删除的）
	var existing model.ClusterNodeIpaddr
	err := db.Unscoped().Where("node_id = ? AND ip = ?", node_id, ip).First(&existing).Error
	if err != nil {
		return false
	}
	return true
}

func ClusterNodeIpaddrSoftDeleteByNodeID(node_id int64) error {
	if err := db.Model(&model.ClusterNodeIpaddr{}).
		Where("node_id = ?", node_id).
		Updates(map[string]interface{}{
			"is_deleted":  1,
			"update_time": time.Now().Unix(),
		}).Error; err != nil {
		return err
	}
	return nil
}

func ClusterNodeIpaddrSoftDeleteByID(id int64) error {
	if err := db.Model(&model.ClusterNodeIpaddr{}).
		Where("id = ?", id).
		Updates(map[string]interface{}{
			"is_deleted":  1,
			"update_time": time.Now().Unix(),
		}).Error; err != nil {
		return err
	}
	return nil
}

func ClusterNodeIpaddrDeleteByID(id int64) error {
	var d model.ClusterNodeIpaddr
	return db.Where("id = ?", id).Delete(&d).Error
}
