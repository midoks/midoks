package admin

import (
	"net/http"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
)

func RecipientsLogs(c *gin.Context) {
	data := common.CommonVer(c)
	data["submenu"] = GetRecipientsSubMenu()
	c.HTML(http.StatusOK, "backend/admin/recipients/logs.tmpl", data)
}
