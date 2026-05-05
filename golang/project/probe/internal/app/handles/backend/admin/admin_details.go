package admin

import (
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/db"
	// "aiprobe/internal/op"
)

func Details(c *gin.Context) {
	id := c.Query("id")
	idInt, _ := strconv.ParseInt(id, 10, 64)
	admin_data, _ := db.GetAdminByID(idInt)

	data := common.CommonVer(c)
	data["id"] = id
	data["Data"] = admin_data
	c.HTML(http.StatusOK, "backend/admin/details.tmpl", data)
}
