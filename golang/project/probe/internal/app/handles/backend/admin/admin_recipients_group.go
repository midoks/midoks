package admin

import (
	// "encoding/json"
	"net/http"
	"strconv"
	"time"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/app/form"
	"aiprobe/internal/db"
	"aiprobe/internal/model"
)

func RecipientsGroups(c *gin.Context) {
	data := common.CommonVer(c)
	data["submenu"] = GetRecipientsSubMenu()
	c.HTML(http.StatusOK, "backend/admin/recipients/groups.tmpl", data)
}

func RecipientsGroupsSelect(c *gin.Context) {
	data := common.CommonVer(c)

	group_list, _, _ := db.GetAdminRecipientsGroupList(1, 100)
	data["groups_list"] = group_list
	c.HTML(http.StatusOK, "backend/admin/recipients/groups_select.tmpl", data)
}

func RecipientsGroupsAdd(c *gin.Context) {

	id := c.Query("id")
	idInt, _ := strconv.ParseInt(id, 10, 64)

	data := common.CommonVer(c)
	ga_data, err := db.GetAdminRecipientsGroupByID(idInt)
	if err == nil {
		data["Data"] = ga_data
	}
	c.HTML(http.StatusOK, "backend/admin/recipients/groups_add.tmpl", data)
}

func RecipientsGroupsList(c *gin.Context) {
	var field form.Page
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	result, count, _ := db.GetAdminRecipientsGroupList(field.Page, field.Limit)
	common.SuccessLayuiResp(c, count, "ok", result)
}

func PostRecipientsGroupsAdd(c *gin.Context) {
	var field form.AdminRecipientsGroup
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	if field.ID > 0 {
		update_data := &model.AdminMediaGroup{
			Name:       field.Name,
			Status:     field.Status,
			UpdateTime: time.Now().Unix(),
		}

		if err := db.GetDb().Model(&model.AdminMediaGroup{}).Where("id = ?", field.ID).Updates(update_data).Error; err != nil {
			common.ErrorResp(c, err, -1)
			return
		}
		common.SuccessResp(c)
		return
	}

	add_data := &model.AdminMediaGroup{
		Name:       field.Name,
		Status:     field.Status,
		CreateTime: time.Now().Unix(),
		UpdateTime: time.Now().Unix(),
	}

	if err := db.GetDb().Create(add_data).Error; err != nil {
		common.ErrorResp(c, err, -1)
		return
	}
	common.SuccessResp(c)
}

func PostRecipientsGroupsDelete(c *gin.Context) {
	var field form.ID
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	if err := db.AdminRecipientsGroupDelete(nil, field.ID); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}
	common.SuccessResp(c)
}
