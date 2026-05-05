package db

import (
	"github.com/pkg/errors"

	"aiprobe/internal/model"
)

func GetDbNodeList(page, size int) ([]model.DbNode, int64, error) {
	mm := db.Model(&model.DbNode{})
	var count int64
	if err := mm.Count(&count).Error; err != nil {
		return nil, 0, errors.Wrapf(err, "failed get dbnode count")
	}

	var list []model.DbNode
	if err := db.Order(columnName("id")).Offset((page - 1) * size).Limit(size).Find(&list).Error; err != nil {
		return nil, 0, errors.WithStack(err)
	}
	return list, count, nil
}

func GetDbNodeByID(id int64) (*model.DbNode, error) {
	var u model.DbNode
	if err := db.First(&u, id).Error; err != nil {
		return nil, errors.Wrapf(err, "failed get log")
	}
	return &u, nil
}
