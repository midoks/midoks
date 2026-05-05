package admin

import (
	"net/http"
	"strconv"
	"strings"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/db"
	// "aiprobe/internal/op"
)

func Update(c *gin.Context) {
	id := c.Query("id")
	idint, _ := strconv.ParseInt(id, 10, 64)
	admin_data, _ := db.GetAdminByID(idint)

	auth := []string{}
	authMap := map[string]bool{}
	if admin_data.Auth != "" {
		auth = strings.Split(admin_data.Auth, ",")
		for _, code := range auth {
			authMap[code] = true
		}
	}

	data := common.CommonVer(c)
	data["id"] = id
	data["Data"] = admin_data
	data["AuthMap"] = authMap
	c.HTML(http.StatusOK, "backend/admin/update.tmpl", data)
}
