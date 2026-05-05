package db

import (
	"github.com/pkg/errors"

	"aiprobe/internal/model"
)

func GetAdminLogsList(page, size int) ([]model.AdminLogs, int64, error) {
	adminM := db.Model(&model.AdminLogs{})
	var count int64
	if err := adminM.Count(&count).Error; err != nil {
		return nil, 0, errors.Wrapf(err, "failed get admin logs count")
	}

	var list []model.AdminLogs
	if err := db.Order(columnName("id")).Offset((page - 1) * size).Limit(size).Find(&list).Error; err != nil {
		return nil, 0, errors.WithStack(err)
	}
	return list, count, nil
}

func GetAdminLogsListByAdminId(admin_id int64, page, size int) ([]model.AdminLogs, int64, error) {
	adminM := db.Model(&model.AdminLogs{})
	var count int64
	if err := adminM.Count(&count).Error; err != nil {
		return nil, 0, errors.Wrapf(err, "failed get admin logs count")
	}

	var list []model.AdminLogs
	if err := db.Order(columnName("id")).Where("admin_id =?", admin_id).Offset((page - 1) * size).Limit(size).Find(&list).Error; err != nil {
		return nil, 0, errors.WithStack(err)
	}
	return list, count, nil
}
