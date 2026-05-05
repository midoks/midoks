package entity

import (
	"aiprobe/internal/model"
)

type AdminRecipientsEntityList struct {
	model.AdminRecipients
	AdminName string `json:"admin_name"` // 管理员名称
	MediaName string `json:"media_name"` // 媒介名称
	GroupName string `json:"group_name"` // 分组名称
}
