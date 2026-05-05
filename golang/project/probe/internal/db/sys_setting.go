package db

import (
	"github.com/pkg/errors"
	"gorm.io/gorm"

	"aiprobe/internal/model"
)

const (
	SettingAdminUI = "admin_ui"
	SettingWebUI   = "web_ui"
	SettingDbConf  = "db_conf"
	SettingLog     = "log_sys"
)

func GetSysSettingByCode(code string) (*model.SysSetting, error) {
	var u model.SysSetting
	if err := db.Where("code = ?", code).First(&u).Error; err != nil {
		return nil, errors.Wrapf(err, "failed get sys setting")
	}
	return &u, nil
}

func SysSettingDeleteByCode(tx *gorm.DB, code string) error {
	if tx == nil {
		tx = db
	}
	var d model.SysSetting
	return tx.Where("code = ?", code).Delete(&d).Error
}
