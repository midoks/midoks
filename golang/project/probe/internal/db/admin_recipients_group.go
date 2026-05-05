package db

import (
	"github.com/pkg/errors"
	"gorm.io/gorm"

	"aiprobe/internal/model"
)

func GetAdminRecipientsGroupList(page, size int) ([]model.AdminMediaGroup, int64, error) {
	adminM := db.Model(&model.AdminMediaGroup{})
	var count int64
	if err := adminM.Count(&count).Error; err != nil {
		return nil, 0, errors.Wrapf(err, "failed get admin recipient group list")
	}

	var list []model.AdminMediaGroup
	if err := db.Order(columnName("id")).Offset((page - 1) * size).Limit(size).Find(&list).Error; err != nil {
		return nil, 0, errors.WithStack(err)
	}
	return list, count, nil
}

func GetAdminRecipientsGroupByID(id int64) (*model.AdminMediaGroup, error) {
	var u model.AdminMediaGroup
	if err := db.First(&u, id).Error; err != nil {
		return nil, errors.Wrapf(err, "failed get admin recipient group by id")
	}
	return &u, nil
}

func AdminRecipientsGroupDelete(tx *gorm.DB, id int64) error {
	if tx == nil {
		tx = db
	}
	var d model.AdminMediaGroup
	return tx.Where("id = ?", id).Delete(&d).Error
}
