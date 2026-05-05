package db

import (
	"github.com/pkg/errors"

	"aiprobe/internal/model"
)

func GetClusterNodeLogsListByID(node_id int64, page, size int) ([]model.ClusterNodeLogs, int64, error) {
	node_logs := db.Model(&model.ClusterNodeLogs{})
	var count int64
	if err := node_logs.Count(&count).Error; err != nil {
		return nil, 0, errors.Wrapf(err, "failed get cluster node logs list")
	}

	var list []model.ClusterNodeLogs
	if err := db.Order(columnName("id")).Where("node_id =?", node_id).Offset((page - 1) * size).Limit(size).Find(&list).Error; err != nil {
		return nil, 0, errors.WithStack(err)
	}
	return list, count, nil
}
