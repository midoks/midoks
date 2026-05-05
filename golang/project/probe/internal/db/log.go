package db

import (
	"time"

	"github.com/pkg/errors"
	"gorm.io/gorm"

	"aiprobe/internal/model"
)

func GetLogList(page, size int) ([]model.Log, int64, error) {
	serverM := db.Model(&model.Log{})
	var count int64
	if err := serverM.Count(&count).Error; err != nil {
		return nil, 0, errors.Wrapf(err, "failed get server count")
	}

	var list []model.Log
	if err := db.Order(columnName("id")).Offset((page - 1) * size).Limit(size).Find(&list).Error; err != nil {
		return nil, 0, errors.WithStack(err)
	}
	return list, count, nil
}

func GetLogByID(id int64) (*model.Log, error) {
	var u model.Log
	if err := db.First(&u, id).Error; err != nil {
		return nil, errors.Wrapf(err, "failed get log")
	}
	return &u, nil
}

func LogDeleteByID(tx *gorm.DB, id int64) error {
	if tx == nil {
		tx = db
	}
	var d model.Log
	return tx.Where("id = ?", id).Delete(&d).Error
}

func AddLog(tx *gorm.DB, uid int64, content string) error {
	if tx == nil {
		tx = db
	}
	var u model.Log
	u.Uid = uid
	u.Content = content
	u.CreateTime = time.Now().Unix()

	return errors.WithStack(tx.Create(&u).Error)
}

func LogDeleteAll(tx *gorm.DB) error {
	if tx == nil {
		tx = db
	}
	var d model.Log
	return errors.WithStack(tx.Where("1 = 1").Delete(&d).Error)
}

func LogDeleteBeforeDays(tx *gorm.DB, days int) error {
	if tx == nil {
		tx = db
	}
	if days <= 0 {
		return nil
	}
	cutoff := time.Now().Add(-time.Duration(days) * 24 * time.Hour).Unix()
	var d model.Log
	return errors.WithStack(tx.Where("create_time < ?", cutoff).Delete(&d).Error)
}
