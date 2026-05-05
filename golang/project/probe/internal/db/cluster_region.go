package db

import (
	"time"

	"gorm.io/gorm"

	"aiprobe/internal/model"

	"github.com/pkg/errors"
)

func GetClusterRegionList(page, size int) ([]model.ClusterRegion, int64, error) {
	cluster := db.Model(&model.ClusterRegion{})
	var count int64
	if err := cluster.Count(&count).Error; err != nil {
		return nil, 0, errors.Wrapf(err, "failed get cluster region")
	}

	var list []model.ClusterRegion
	if err := db.Order(columnName("id")).Offset((page - 1) * size).Limit(size).Find(&list).Error; err != nil {
		return nil, 0, errors.WithStack(err)
	}
	return list, count, nil
}

func AddClusterRegion(tx *gorm.DB, name string, mark string) error {
	if tx == nil {
		tx = db
	}
	data := &model.ClusterRegion{
		Name: name,
		Mark: mark,
	}

	data.CreateTime = time.Now().Unix()
	data.UpdateTime = time.Now().Unix()
	if err := errors.WithStack(tx.Create(data).Error); err != nil {
		return err
	}
	return nil
}

func UpdateClusterRegion(tx *gorm.DB, name string, mark string, id int64) error {
	if tx == nil {
		tx = db
	}
	data := &model.ClusterRegion{
		Name: name,
		Mark: mark,
	}

	data.UpdateTime = time.Now().Unix()
	if err := tx.Model(&model.ClusterRegion{}).
		Where("id = ?", id).
		Updates(&data).Error; err != nil {
		return err
	}
	return nil
}

func GetClusterRegionByID(id int64) (*model.ClusterRegion, error) {
	var data model.ClusterRegion
	if err := db.First(&data, id).Error; err != nil {
		return nil, errors.Wrapf(err, "failed get cluster region")
	}
	return &data, nil
}

func ClusterRegionDeleteByID(tx *gorm.DB, id int64) error {
	if tx == nil {
		tx = db
	}
	var d model.ClusterRegion
	return tx.Where("id = ?", id).Delete(&d).Error
}

func ClusterRegionsTriggerStatus(tx *gorm.DB, id int64) error {
	if tx == nil {
		tx = db
	}
	var data model.ClusterRegion
	if err := tx.First(&data, id).Error; err != nil {
		return errors.Wrapf(err, "failed get cluster region")
	}

	var status int
	if data.Status > 0 {
		status = 0
	} else {
		status = 1
	}

	data.UpdateTime = time.Now().Unix()
	data.Status = status

	if err := tx.Model(&model.ClusterRegion{}).
		Where("id = ?", id).
		Updates(&data).Error; err != nil {
		return err
	}
	return nil
}
