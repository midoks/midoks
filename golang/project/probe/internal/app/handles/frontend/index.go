package frontend

import (
	"net/http"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
)

func Home(c *gin.Context) {
	data := common.CommonVer(c)
	c.HTML(http.StatusOK, "frontend/index/index.tmpl", data)
}
