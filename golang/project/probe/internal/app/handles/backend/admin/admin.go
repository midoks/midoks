package admin

import (
	// "encoding/json"
	"errors"
	// "fmt"
	"net/http"
	"strconv"
	"strings"

	// "time"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/app/form"
	"aiprobe/internal/db"
	"aiprobe/internal/model"
	// utils "aiprobe/internal/utils"
	// "aiprobe/internal/op"
)

func Home(c *gin.Context) {
	data := common.CommonVer(c)
	c.HTML(http.StatusOK, "backend/admin/index.tmpl", data)
}

func Add(c *gin.Context) {
	id := c.Query("id")
	idint, _ := strconv.ParseInt(id, 10, 64)

	admin_data, _ := db.GetAdminByID(idint)
	if admin_data == nil {
		admin_data = &model.Admin{}
	}

	auth := []string{}
	authMap := map[string]bool{}
	if admin_data.Auth != "" {
		auth = strings.Split(admin_data.Auth, ",")
		for _, code := range auth {
			authMap[code] = true
		}
	}

	data := common.CommonVer(c)
	data["Data"] = admin_data
	data["AuthMap"] = authMap
	c.HTML(http.StatusOK, "backend/admin/add.tmpl", data)
}

func PostAdd(c *gin.Context) {
	var f form.AdminAdd
	if err := c.ShouldBind(&f); err != nil {
		common.ErrorResp(c, err, 0)
		return
	}

	f.Auth = c.PostFormMap("auth")
	codes := []string{}
	for k, v := range f.Auth {
		if v == "on" {
			codes = append(codes, k)
		}
	}
	codesStr := strings.Join(codes, ",")
	if f.ID > 0 {
		db.UpdateAdmin(nil, f.ID, f.Username, f.Password, f.FullName, codesStr, f.AllowLogin, f.SuperAdmin)
	} else {
		db.AddAdmin(nil, f.Username, f.Password, f.FullName, codesStr, f.AllowLogin, f.SuperAdmin)
	}
	common.SuccessResp(c)
}

func List(c *gin.Context) {
	var field form.Page
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	result, count, _ := db.GetAdminList(field.Page, field.Limit)
	common.SuccessLayuiResp(c, count, "ok", result)
}

func AdminTriggerStatus(c *gin.Context) {
	var field form.ID
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	err := db.AdminTriggerStatus(nil, field.ID)
	if err == nil {
		common.SuccessResp(c)
		return
	}
	common.ErrorResp(c, err, -1)
}

func Delete(c *gin.Context) {
	var field form.ID
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	if field.ID == 1 {
		common.ErrorResp(c, errors.New("the admin cannot delete!"), -1)
		return
	}

	if err := db.AdminDeleteByID(nil, field.ID); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}
	common.SuccessResp(c)
}
