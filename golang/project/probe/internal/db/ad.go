package db

import (
	"time"

	"github.com/pkg/errors"
	"gorm.io/gorm"

	"aiprobe/internal/model"
)

func GetAdList(page, size int, position string) ([]model.Ad, int64, error) {
	adM := db.Model(&model.Ad{})
	if position != "" {
		adM = adM.Where("position = ?", position)
	}
	var count int64
	if err := adM.Count(&count).Error; err != nil {
		return nil, 0, errors.Wrapf(err, "failed get ad count")
	}

	var list []model.Ad
	if err := adM.Order("sort DESC, id DESC").Offset((page - 1) * size).Limit(size).Find(&list).Error; err != nil {
		return nil, 0, errors.WithStack(err)
	}
	return list, count, nil
}

func GetAdByID(id int64) (*model.Ad, error) {
	var ad model.Ad
	if err := db.First(&ad, id).Error; err != nil {
		return nil, errors.Wrapf(err, "failed get ad")
	}
	return &ad, nil
}

func CreateAd(ad *model.Ad) error {
	ad.CreateTime = time.Now().Unix()
	ad.UpdateTime = time.Now().Unix()
	return db.Create(ad).Error
}

func UpdateAd(ad *model.Ad) error {
	ad.UpdateTime = time.Now().Unix()
	return db.Save(ad).Error
}

func DeleteAd(id int64) error {
	return db.Delete(&model.Ad{}, id).Error
}

func GetActiveAds(position string) ([]model.Ad, error) {
	var ads []model.Ad
	now := time.Now().Unix()
	query := db.Model(&model.Ad{}).Where("status = ?", true).
		Where("start_time <= ? OR start_time = 0", now).
		Where("end_time >= ? OR end_time = 0", now).
		Where("position = ?", position).
		Order("sort DESC, id DESC")

	if err := query.Find(&ads).Error; err != nil {
		return nil, errors.Wrapf(err, "failed get active ads")
	}
	return ads, nil
}

func UpdateAdStatus(id int64, status bool) error {
	return db.Model(&model.Ad{}).Where("id = ?", id).Update("status", status).Error
}

func AdUpdateSort(tx *gorm.DB, id int64, sort int) error {
	if tx == nil {
		tx = db
	}
	return tx.Model(&model.Ad{ID: id}).Update("sort", sort).Error
}
