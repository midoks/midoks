package log

import (
	"net/http"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/app/form"
	"aiprobe/internal/db"
)

func GetLogSubMenu() []form.SubMenu {
	menu := []form.SubMenu{
		{
			Number: 1,
			Name:   "查询",
			Link:   "log",
		},
		{
			Number: 2,
			Name:   "清理",
			Link:   "log/clean",
		},
		{
			Number: 3,
			Name:   "设置",
			Link:   "log/settings",
		},
	}
	return menu
}

func Home(c *gin.Context) {
	data := common.CommonVer(c)
	data["submenu"] = GetLogSubMenu()
	c.HTML(http.StatusOK, "backend/log/index.tmpl", data)
}

func List(c *gin.Context) {
	var field form.Page
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	result, count, _ := db.GetLogList(field.Page, field.Limit)
	common.SuccessLayuiResp(c, count, "ok", result)
}

func Delete(c *gin.Context) {
	var field form.ID
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	err := db.LogDeleteByID(nil, field.ID)
	if err == nil {
		common.SuccessResp(c)
		return
	}
	common.ErrorResp(c, err, -1)
}
