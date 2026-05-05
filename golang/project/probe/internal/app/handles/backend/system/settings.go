package system

import (
	"errors"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/app/form"
	"aiprobe/internal/db"
	"aiprobe/internal/model"
	utils "aiprobe/internal/utils"
)

func GetSysBaseSubMenu() []form.SubMenu {
	menu := []form.SubMenu{
		{
			Number: 1,
			Name:   "管理员界面设置",
			Link:   "system/settings",
		},
		{
			Number: 2,
			Name:   "前端界面设置",
			Link:   "system/settings/web",
		},
		{
			Number: 3,
			Name:   "个人资料",
			Link:   "system/settings/profile",
		},
		{
			Number: 4,
			Name:   "登录设置",
			Link:   "system/settings/login",
		},
	}
	return menu
}

func Home(c *gin.Context) {
	data := common.CommonVer(c)
	data["submenu"] = GetSysBaseSubMenu()
	c.HTML(http.StatusOK, "backend/system/settings/index.tmpl", data)
}

func PostHome(c *gin.Context) {
	var field form.SettingAdminUI
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	common_data := &model.SysSetting{
		Code: db.SettingAdminUI,
		Uid:  0,
	}

	common_data.SetAdminUIValue(model.SysSettingAdminUIValue{
		ProductName: field.ProductName,
		SystemName:  field.SystemName,
	})
	common_data.UpdateTime = time.Now().Unix()
	_, err := db.GetSysSettingByCode(db.SettingAdminUI)
	if err == nil {
		if err := db.GetDb().Model(&model.SysSetting{}).Where("code = ?", db.SettingAdminUI).Updates(common_data).Error; err != nil {
			common.ErrorResp(c, err, -1)
			return
		}
		common.SuccessResp(c)
		return
	}

	common_data.CreateTime = time.Now().Unix()
	if err := db.GetDb().Create(common_data).Error; err != nil {
		common.ErrorResp(c, err, -1)
		return
	}
	common.SuccessResp(c)
}

func Web(c *gin.Context) {
	data := common.CommonVer(c)
	data["submenu"] = GetSysBaseSubMenu()

	setting_web_ui_data, err := db.GetSysSettingByCode(db.SettingWebUI)
	if err == nil {
		data["setting_web_ui"] = setting_web_ui_data
	}
	c.HTML(http.StatusOK, "backend/system/settings/web.tmpl", data)
}

func PostWeb(c *gin.Context) {
	var field form.SettingWebUI
	var err error
	if err = c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	common_data := &model.SysSetting{
		Code: db.SettingWebUI,
		Uid:  0,
	}

	common_data.SetWebUIValue(model.SysSettingWebUIValue{
		Name:     field.Name,
		Subtitle: field.Subtitle,
	})
	common_data.UpdateTime = time.Now().Unix()
	_, err = db.GetSysSettingByCode(db.SettingWebUI)
	if err == nil {
		if err = db.GetDb().Model(&model.SysSetting{}).Where("code = ?", db.SettingWebUI).Updates(common_data).Error; err != nil {
			common.ErrorResp(c, err, -1)
			return
		}
		common.SuccessResp(c)
		return
	}

	common_data.CreateTime = time.Now().Unix()
	if err = db.GetDb().Create(common_data).Error; err != nil {
		common.ErrorResp(c, err, -1)
		return
	}
	common.SuccessResp(c)
}

func Profile(c *gin.Context) {
	data := common.CommonVer(c)
	data["submenu"] = GetSysBaseSubMenu()
	c.HTML(http.StatusOK, "backend/system/settings/profile.tmpl", data)
}

func PostProfile(c *gin.Context) {
	var field form.SettingProfile
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	if field.Name == "" {
		common.ErrorResp(c, errors.New("你的姓名,不能为空!"), -2)
		return
	}

	common_data := &model.Admin{
		FullName:   field.Name,
		UpdateTime: time.Now().Unix(),
	}
	data := common.CommonVer(c)
	adminID := data["login_uid"]
	if err := db.GetDb().Model(&model.Admin{}).Where("id = ?", adminID).Updates(common_data).Error; err != nil {
		common.ErrorResp(c, err, -1)
		return
	}
	common.SuccessResp(c)
}

func Login(c *gin.Context) {
	data := common.CommonVer(c)
	data["submenu"] = GetSysBaseSubMenu()
	c.HTML(http.StatusOK, "backend/system/settings/login.tmpl", data)
}

func PostLogin(c *gin.Context) {
	var field form.SettingLogin
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	if field.Name == "" {
		common.ErrorResp(c, errors.New("你的姓名,不能为空!"), -2)
		return
	}
	common_data := &model.Admin{
		FullName:   field.Name,
		UpdateTime: time.Now().Unix(),
	}

	if field.Password != "" || field.Password2 != "" {
		if field.Password != field.Password2 {
			common.ErrorResp(c, errors.New("两次密码不一致!"), -2)
			return
		}

		salt := utils.RandString(16)
		common_data.Salt = salt
		common_data.Password = model.TwoHashPwd(field.Password, salt)
	}

	data := common.CommonVer(c)
	adminID := data["login_uid"]
	if err := db.GetDb().Model(&model.Admin{}).Where("id = ?", adminID).Updates(common_data).Error; err != nil {
		common.ErrorResp(c, err, -1)
		return
	}
	common.SuccessResp(c)
}

func LoginLogs(c *gin.Context) {
	data := common.CommonVer(c)
	data["submenu"] = GetSysBaseSubMenu()
	c.HTML(http.StatusOK, "backend/system/settings/login_logs.tmpl", data)
}

func LoginLogsList(c *gin.Context) {
	var field form.AdminPage
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	result, count, _ := db.GetAdminLogsListByAdminId(field.AdminId, field.Page.Page, field.Page.Limit)
	common.SuccessLayuiResp(c, count, "ok", result)
}
