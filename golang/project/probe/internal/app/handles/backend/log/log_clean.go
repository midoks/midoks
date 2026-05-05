package log

import (
	"net/http"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/app/form"
	"aiprobe/internal/db"
)

func Clean(c *gin.Context) {
	data := common.CommonVer(c)
	data["submenu"] = GetLogSubMenu()
	c.HTML(http.StatusOK, "backend/log/clean.tmpl", data)
}

func PostLogClean(c *gin.Context) {
	var field form.LogClean
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, 0)
		return
	}
	if field.Clean == "all" {
		if err := db.LogDeleteAll(nil); err != nil {
			common.ErrorResp(c, err, 0)
			return
		}
	} else {
		if err := db.LogDeleteBeforeDays(nil, int(field.Day)); err != nil {
			common.ErrorResp(c, err, 0)
			return
		}
	}
	common.SuccessResp(c)
}
