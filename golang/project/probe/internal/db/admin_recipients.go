package db

import (
	"github.com/pkg/errors"
	"gorm.io/gorm"

	"aiprobe/internal/app/entity"
	"aiprobe/internal/model"
)

func GetAdminRecipientsList(page, size int) ([]entity.AdminRecipientsEntityList, int64, error) {
	m := db.Model(&model.AdminRecipients{})
	var count int64
	if err := m.Count(&count).Error; err != nil {
		return nil, 0, errors.Wrapf(err, "failed get recipients data list")
	}

	var list []model.AdminRecipients
	if err := db.Order(columnName("id")).Offset((page - 1) * size).Limit(size).Find(&list).Error; err != nil {
		return nil, 0, errors.WithStack(err)
	}

	if len(list) == 0 {
		return []entity.AdminRecipientsEntityList{}, count, nil
	}

	adminIDs := make([]int64, 0)
	mediaIDs := make([]int64, 0)
	groupIDs := make([]int64, 0)
	for _, item := range list {
		if item.AdminID > 0 {
			adminIDs = append(adminIDs, item.AdminID)
		}
		if item.MediaID > 0 {
			mediaIDs = append(mediaIDs, item.MediaID)
		}
		if item.GroupID > 0 {
			groupIDs = append(groupIDs, item.GroupID)
		}
	}

	adminMap := make(map[int64]string)
	if len(adminIDs) > 0 {
		var admins []model.Admin
		db.Where("id IN ?", adminIDs).Find(&admins)
		for _, a := range admins {
			adminMap[a.ID] = a.Username
		}
	}

	mediaMap := make(map[int64]string)
	if len(mediaIDs) > 0 {
		var medias []model.AdminMediaInstance
		db.Where("id IN ?", mediaIDs).Find(&medias)
		for _, m := range medias {
			mediaMap[m.ID] = m.Name
		}
	}

	groupMap := make(map[int64]string)
	if len(groupIDs) > 0 {
		var groups []model.AdminMediaGroup
		db.Where("id IN ?", groupIDs).Find(&groups)
		for _, g := range groups {
			groupMap[g.ID] = g.Name
		}
	}

	result := make([]entity.AdminRecipientsEntityList, 0, len(list))
	for _, item := range list {
		entityItem := entity.AdminRecipientsEntityList{
			AdminRecipients: item,
			AdminName:       adminMap[item.AdminID],
			MediaName:       mediaMap[item.MediaID],
			GroupName:       groupMap[item.GroupID],
		}
		result = append(result, entityItem)
	}

	return result, count, nil
}

func GetAdminRecipientsByID(id int64) (*entity.AdminRecipientsEntityList, error) {
	var u model.AdminRecipients
	if err := db.First(&u, id).Error; err != nil {
		return nil, errors.Wrapf(err, "failed get recipients data")
	}

	entityItem := entity.AdminRecipientsEntityList{
		AdminRecipients: u,
	}

	if u.AdminID > 0 {
		if admin, err := GetAdminByID(u.AdminID); err == nil {
			entityItem.AdminName = admin.Username
		}
	}

	if u.MediaID > 0 {
		if media, err := GetAdminRecipientsInstancesByID(u.MediaID); err == nil {
			entityItem.MediaName = media.Name
		}
	}

	if u.GroupID > 0 {
		if group, err := GetAdminRecipientsGroupByID(u.GroupID); err == nil {
			entityItem.GroupName = group.Name
		}
	}

	return &entityItem, nil
}

func AdminRecipientsDeleteByID(tx *gorm.DB, id int64) error {
	if tx == nil {
		tx = db
	}
	var d model.AdminRecipients
	return tx.Where("id = ?", id).Delete(&d).Error
}
