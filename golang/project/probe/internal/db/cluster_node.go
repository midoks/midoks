package db

import (
	"github.com/pkg/errors"
	"gorm.io/gorm"

	"aiprobe/internal/app/form"
	"aiprobe/internal/model"
)

func GetClusterNodeListByArgs(field form.ClusterNodeList) ([]model.ClusterNode, int64, error) {
	page := field.Page.Page
	size := field.Page.Limit

	cluster := db.Model(&model.ClusterNode{})
	var count int64
	if err := cluster.Count(&count).Error; err != nil {
		return nil, 0, errors.Wrapf(err, "failed get cluster node")
	}

	var list []model.ClusterNode

	mm := db.Order(columnName("id"))
	if field.ClusterID > 0 {
		mm = mm.Where("cluster_id =?", field.ClusterID)
	}

	if err := mm.Offset((page - 1) * size).Limit(size).Find(&list).Error; err != nil {
		return nil, 0, errors.WithStack(err)
	}
	return list, count, nil
}

func GetClusterNodeList(page, size int) ([]model.ClusterNode, int64, error) {
	cluster := db.Model(&model.ClusterNode{})
	var count int64
	if err := cluster.Count(&count).Error; err != nil {
		return nil, 0, errors.Wrapf(err, "failed get cluster node")
	}

	var list []model.ClusterNode
	if err := db.Order(columnName("id")).Offset((page - 1) * size).Limit(size).Find(&list).Error; err != nil {
		return nil, 0, errors.WithStack(err)
	}
	return list, count, nil
}

func GetClusterNodeListByClusterID(cluster_id int64, page, size int) ([]model.ClusterNode, int64, error) {
	cluster := db.Model(&model.ClusterNode{})
	var count int64
	if err := cluster.Count(&count).Error; err != nil {
		return nil, 0, errors.Wrapf(err, "failed get cluster node list")
	}

	var list []model.ClusterNode
	if err := db.Order(columnName("id")).Where("cluster_id =?", cluster_id).Offset((page - 1) * size).Limit(size).Find(&list).Error; err != nil {
		return nil, 0, errors.WithStack(err)
	}
	return list, count, nil
}

func GetClusterNodeByID(id int64) (*model.ClusterNode, error) {
	var data model.ClusterNode
	if err := db.First(&data, id).Error; err != nil {
		return nil, errors.Wrapf(err, "failed get cluster node")
	}
	return &data, nil
}

func GetClusterNodeByUniqueIdAndSecret(unique_id string, secret string) (*model.ClusterNode, error) {
	var data model.ClusterNode
	if err := db.Where("unique_id = ?", unique_id).Where("secret = ?", secret).First(&data).Error; err != nil {
		return nil, errors.Wrapf(err, "failed get cluster node")
	}
	return &data, nil
}

func ClusterNodeDeleteByID(tx *gorm.DB, id int64) error {
	if tx == nil {
		tx = db
	}
	var d model.ClusterNode
	return tx.Where("id = ?", id).Delete(&d).Error
}

// 节点安装成功,状态修改
func ClusterNodeInstallDone(node_id int64, is_installed bool) error {
	if err := db.Model(&model.ClusterNode{}).Where("id = ?", node_id).Update("is_installed", is_installed).Error; err != nil {
		return err
	}
	return nil
}
