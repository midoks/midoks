package op

import (
	"fmt"
	"time"

	"github.com/pkg/errors"
	"gorm.io/gorm"

	"aiprobe/internal/db"
	"aiprobe/internal/model"
	utils "aiprobe/internal/utils"
)

func InitAdmin(user string, pass string) error {
	data, err := db.GetAdminByID(1)
	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {

			salt := utils.RandString(16)
			admin := &model.Admin{
				Username:   user,
				Password:   model.TwoHashPwd(pass, salt),
				Salt:       salt,
				AllowLogin: true,
				Status:     true,
				SuperAdmin: true,
				FullName:   "超级管理员",
			}

			admin.CreateTime = time.Now().Unix()
			admin.UpdateTime = time.Now().Unix()
			if err := db.CreateAdmin(nil, admin); err != nil {
				return err
			}
		}
	}

	fmt.Println("data:", data)
	return nil
}

func InitSetting() error {
	err := InitSettingAdminData()
	if err != nil {
		fmt.Println("InitSettingAdminData:", err)
		return err
	}

	err = InitSettingWebData()
	if err != nil {
		fmt.Println("InitSettingWebData:", err)
		return err
	}

	err = InitSettingDbConfData()
	if err != nil {
		fmt.Println("InitSettingDbConfData:", err)
		return err
	}

	err = InitSettingLogData()
	if err != nil {
		fmt.Println("InitSettingLogData:", err)
		return err
	}
	fmt.Println("InitSetting completed successfully")
	return nil
}

func InitSettingAdminData() error {
	common_data := &model.SysSetting{
		Code: db.SettingAdminUI,
		Uid:  0,
	}

	common_data.SetAdminUIValue(model.SysSettingAdminUIValue{
		ProductName: "testspeed",
		SystemName:  "测速面板",
	})
	common_data.UpdateTime = time.Now().Unix()
	var err error
	_, err = db.GetSysSettingByCode(db.SettingAdminUI)
	if err == nil {
		if err := db.GetDb().Model(&model.SysSetting{}).Where("code = ?", db.SettingAdminUI).Updates(common_data).Error; err != nil {
			return err
		}
		return nil
	}

	common_data.CreateTime = time.Now().Unix()
	if err := db.GetDb().Create(common_data).Error; err != nil {
		return err
	}
	return nil
}

func InitSettingWebData() error {
	common_data := &model.SysSetting{
		Code: db.SettingWebUI,
		Uid:  0,
	}

	common_data.SetWebUIValue(model.SysSettingWebUIValue{
		Name:     "MGO",
		Subtitle: "系统开发模板",
	})
	common_data.UpdateTime = time.Now().Unix()
	var err error
	_, err = db.GetSysSettingByCode(db.SettingWebUI)
	if err == nil {
		if err := db.GetDb().Model(&model.SysSetting{}).Where("code = ?", db.SettingWebUI).Updates(common_data).Error; err != nil {
			return err
		}
		return nil
	}

	common_data.CreateTime = time.Now().Unix()
	if err := db.GetDb().Create(common_data).Error; err != nil {
		return err
	}
	return nil
}

func InitSettingDbConfData() error {
	common_data := &model.SysSetting{
		Code: db.SettingDbConf,
		Uid:  0,
	}

	common_data.SetDbConfValue(model.SysSettingDbConfValue{
		LogSaveDays: 180,
	})
	common_data.UpdateTime = time.Now().Unix()
	var err error
	_, err = db.GetSysSettingByCode(db.SettingDbConf)
	if err == nil {
		if err := db.GetDb().Model(&model.SysSetting{}).Where("code = ?", db.SettingDbConf).Updates(common_data).Error; err != nil {
			return err
		}
		return nil
	}

	common_data.CreateTime = time.Now().Unix()
	if err := db.GetDb().Create(common_data).Error; err != nil {
		return err
	}
	return nil
}

func InitSettingLogData() error {
	common_data := &model.SysSetting{
		Code: db.SettingLog,
		Uid:  0,
	}

	common_data.SetLogValue(model.SysSettingLogValue{
		AllowedManualDelete:   true,
		AllowedManual:         true,
		SaveDay:               180,
		MaxCapacityLimit:      100,
		MaxCapacityUnit:       "mib",
		AllowedModClearConfig: true,
	})
	common_data.UpdateTime = time.Now().Unix()
	var err error
	_, err = db.GetSysSettingByCode(db.SettingLog)
	if err == nil {
		if err := db.GetDb().Model(&model.SysSetting{}).Where("code = ?", db.SettingLog).Updates(common_data).Error; err != nil {
			return err
		}
		return nil
	}

	common_data.CreateTime = time.Now().Unix()
	if err := db.GetDb().Create(common_data).Error; err != nil {
		return err
	}
	return nil
}
