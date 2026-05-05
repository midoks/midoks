package db

import (
	"fmt"
	"time"

	"gorm.io/gorm"

	"aiprobe/internal/app/entity"
	"aiprobe/internal/model"
	"aiprobe/internal/utils/cache"

	"github.com/pkg/errors"
)

func GetClusterSshList(page, size int) ([]model.ClusterSsh, int64, error) {
	cluster := db.Model(&model.ClusterSsh{})
	var count int64
	if err := cluster.Count(&count).Error; err != nil {
		return nil, 0, errors.Wrapf(err, "failed get cluster ssh")
	}

	var list []model.ClusterSsh
	if err := db.Order(columnName("id")).Offset((page - 1) * size).Limit(size).Find(&list).Error; err != nil {
		return nil, 0, errors.WithStack(err)
	}
	return list, count, nil
}

func GetClusterSshListByLimit(limit int) ([]entity.ClusterSsh, error) {
	var models []model.ClusterSsh
	if err := db.Order(columnName("id")).Limit(limit).Find(&models).Error; err != nil {
		return nil, errors.WithStack(err)
	}
	out := make([]entity.ClusterSsh, 0, len(models))
	for _, m := range models {
		out = append(out, entity.ClusterSsh{
			ID:       m.ID,
			Name:     m.Name,
			Method:   m.Method,
			Username: m.Username,
		})
	}
	return out, nil
}

func GetClusterSshListBySuggest(clusterID int64) ([]entity.ClusterSsh, error) {
	out := []entity.ClusterSsh{}

	ids, err := ClusterNodeLoginFindFrequentSshIDs(clusterID)
	fmt.Println("ids:", ids, err)
	if err != nil {
		return out, err
	}

	if len(ids) == 0 {
		return out, nil
	}

	var models []model.ClusterSsh
	if err := db.Order(columnName("id")).Where("id IN (?)", ids).Limit(3).Find(&models).Error; err != nil {
		return nil, errors.WithStack(err)
	}
	out = make([]entity.ClusterSsh, 0, len(models))
	for _, m := range models {
		out = append(out, entity.ClusterSsh{
			ID:       m.ID,
			Name:     m.Name,
			Method:   m.Method,
			Username: m.Username,
		})
	}
	return out, nil
}

func GetClusterSshByID(id int64) (*model.ClusterSsh, error) {
	// Check cache first
	cacheKey := fmt.Sprintf("ssh_%d", id)
	if cached, found := cache.GetGlobal(cacheKey); found {
		if ssh, ok := cached.(*model.ClusterSsh); ok {
			return ssh, nil
		}
	}

	var data model.ClusterSsh
	if err := db.First(&data, id).Error; err != nil {
		return nil, errors.Wrapf(err, "failed get cluster ssh")
	}

	// Cache for 5 minutes
	cache.SetGlobal(cacheKey, &data, 5*time.Minute)

	return &data, nil
}

func ClusterSshDeleteByID(tx *gorm.DB, id int64) error {
	if tx == nil {
		tx = db
	}
	var d model.ClusterSsh
	return tx.Where("id = ?", id).Delete(&d).Error
}
