package db

import (
	"time"

	"gorm.io/gorm"

	"aiprobe/internal/model"

	"github.com/pkg/errors"
)

func GetClusterGroupList(page, size int) ([]model.ClusterGroup, int64, error) {
	cluster := db.Model(&model.ClusterGroup{})
	var count int64
	if err := cluster.Count(&count).Error; err != nil {
		return nil, 0, errors.Wrapf(err, "failed get cluster group")
	}

	var list []model.ClusterGroup
	if err := db.Order(columnName("id")).Offset((page - 1) * size).Limit(size).Find(&list).Error; err != nil {
		return nil, 0, errors.WithStack(err)
	}
	return list, count, nil
}

func AddClusterGroup(tx *gorm.DB, name string, clusterId int64) error {
	if tx == nil {
		tx = db
	}
	data := &model.ClusterGroup{
		Name:      name,
		ClusterId: clusterId,
	}

	data.CreateTime = time.Now().Unix()
	data.UpdateTime = time.Now().Unix()
	if err := errors.WithStack(tx.Create(data).Error); err != nil {
		return err
	}
	return nil
}

func UpdateClusterGroup(tx *gorm.DB, name string, id int64) error {
	if tx == nil {
		tx = db
	}
	data := &model.ClusterGroup{
		Name: name,
	}

	data.UpdateTime = time.Now().Unix()
	if err := tx.Model(&model.ClusterGroup{}).
		Where("id = ?", id).
		Updates(&data).Error; err != nil {
		return err
	}
	return nil
}

func GetClusterGroupByID(id int64) (*model.ClusterGroup, error) {
	var data model.ClusterGroup
	if err := db.First(&data, id).Error; err != nil {
		return nil, errors.Wrapf(err, "failed get cluster group")
	}
	return &data, nil
}

func ClusterGroupDeleteByID(tx *gorm.DB, id int64) error {
	if tx == nil {
		tx = db
	}
	var d model.ClusterGroup
	return tx.Where("id = ?", id).Delete(&d).Error
}
