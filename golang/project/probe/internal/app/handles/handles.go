package handles

import (
	"net/http"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/conf"
)

func AdminPage(c *gin.Context) {
	if !conf.Security.InstallLock {
		c.Redirect(302, "/install")
	}
	data := common.CommonVer(c)
	c.HTML(http.StatusOK, "index.tmpl", data)
}

func Home(c *gin.Context) {
	data := common.CommonVer(c)
	c.HTML(http.StatusOK, "backend/console/index.tmpl", data)
}

func NotFound(c *gin.Context) {
	data := common.CommonVer(c)
	c.HTML(http.StatusOK, "404.tmpl", data)
}
