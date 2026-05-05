package system

import (
	"net/http"
	"time"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/app/form"
	"aiprobe/internal/conf"
	"aiprobe/internal/db"
	"aiprobe/internal/model"
	"aiprobe/internal/op"
)

func GetSysAdvancedSubMenu() []form.ClusterSubMenu {
	menu := []form.ClusterSubMenu{
		{
			Number: 1,
			Name:   "数据库",
			Link:   "system/database",
		},
		{
			Number: 2,
			Name:   "API节点",
			Link:   "clusters/cluster/list",
		},
		{
			Number: 3,
			Name:   "用户节点",
			Link:   "clusters/cluster/create_node",
		},
		{
			Number: 4,
			Name:   "日志数据库",
			Link:   "system/db",
		},
		{
			Number: 5,
			Name:   "迁移",
			Link:   "clusters/cluster/create_node",
		},
	}
	return menu
}

func GetSysAdvancedDatabaseSubMenu() []form.SubMenu {
	menu := []form.SubMenu{
		{
			Number: 1,
			Name:   "配置模板",
			Link:   "system/database/index",
		},
		{
			Number: 2,
			Name:   "手动清理",
			Link:   "system/database/cleans",
		},
		{
			Number: 3,
			Name:   "自动清理设置",
			Link:   "system/database/clean_setting",
		},
	}
	return menu
}

func Database(c *gin.Context) {
	data := common.CommonVer(c)
	data["submenu"] = GetSysAdvancedSubMenu()
	data["database_submenu"] = GetSysAdvancedDatabaseSubMenu()
	data["Data"] = conf.Database
	c.HTML(http.StatusOK, "backend/system/database/index.tmpl", data)
}

// func DatabaseUpdate(c *gin.Context) {
// 	data := common.CommonVer(c)
// 	data["submenu"] = GetSysAdvancedSubMenu()
// 	data["database_submenu"] = GetSysAdvancedDatabaseSubMenu()
// 	c.HTML(http.StatusOK, "backend/system/database/update.tmpl", data)
// }

func DatabaseClean(c *gin.Context) {
	data := common.CommonVer(c)
	data["submenu"] = GetSysAdvancedSubMenu()
	data["database_submenu"] = GetSysAdvancedDatabaseSubMenu()
	c.HTML(http.StatusOK, "backend/system/database/cleans.tmpl", data)
}

func DatabaseList(c *gin.Context) {
	tables, err := op.GetTableList()
	if err != nil {
		common.ErrorResp(c, err, -1)
		return
	}
	common.SuccessLayuiResp(c, int64(len(tables)), "ok", tables)
}
func PostDatabaseClean(c *gin.Context) {
	var field form.DatabaseCommon
	var err error
	if err = c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	if err := op.CleanTableByName(field.TableName); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	common.SuccessResp(c)
}

func PostDatabaseDelete(c *gin.Context) {
	var field form.DatabaseCommon
	var err error
	if err = c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}
	if err := op.DeleteTableByName(field.TableName); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	common.SuccessResp(c)
}

func DatabaseCleanSetting(c *gin.Context) {
	data := common.CommonVer(c)
	data["submenu"] = GetSysAdvancedSubMenu()
	data["database_submenu"] = GetSysAdvancedDatabaseSubMenu()

	// 获取数据库配置设置
	setting_db_conf, err := db.GetSysSettingByCode(db.SettingDbConf)
	if err == nil {
		db_conf, _ := setting_db_conf.GetDbConfValue()
		data["log_save_days"] = db_conf.LogSaveDays
	} else {
		data["log_save_days"] = 180
	}

	c.HTML(http.StatusOK, "backend/system/database/clean_setting.tmpl", data)
}

func PostDatabaseCleanSetting(c *gin.Context) {
	var field form.SettingDbConf
	var err error
	if err = c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	common_data := &model.SysSetting{
		Code: db.SettingDbConf,
		Uid:  0,
	}

	common_data.SetDbConfValue(model.SysSettingDbConfValue{
		LogSaveDays: field.LogSaveDays,
	})
	common_data.UpdateTime = time.Now().Unix()
	_, err = db.GetSysSettingByCode(db.SettingDbConf)
	if err == nil {
		if err := db.GetDb().Model(&model.SysSetting{}).Where("code = ?", db.SettingDbConf).Updates(common_data).Error; err != nil {
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
