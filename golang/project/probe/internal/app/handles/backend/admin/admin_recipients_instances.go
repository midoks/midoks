package admin

import (
	"context"
	"errors"
	"fmt"
	"net/http"
	"strconv"
	"time"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/app/form"
	"aiprobe/internal/db"
	"aiprobe/internal/model"
	"aiprobe/internal/notify"
)

func RecipientsInstances(c *gin.Context) {
	data := common.CommonVer(c)
	data["submenu"] = GetRecipientsSubMenu()
	c.HTML(http.StatusOK, "backend/admin/recipients/instances.tmpl", data)
}

func RecipientsInstancesAdd(c *gin.Context) {
	data := common.CommonVer(c)
	c.HTML(http.StatusOK, "backend/admin/recipients/instances_add.tmpl", data)
}

func RecipientsInstancesDetails(c *gin.Context) {
	id := c.Query("id")
	idInt, _ := strconv.ParseInt(id, 10, 64)
	recipient_data, _ := db.GetAdminRecipientsInstancesByID(idInt)

	data := common.CommonVer(c)
	data["id"] = id
	data["Data"] = recipient_data
	c.HTML(http.StatusOK, "backend/admin/recipients/instances_details.tmpl", data)
}

func RecipientsInstancesUpdate(c *gin.Context) {
	id := c.Query("id")
	idInt, _ := strconv.ParseInt(id, 10, 64)
	recipient_data, _ := db.GetAdminRecipientsInstancesByID(idInt)

	data := common.CommonVer(c)
	data["id"] = id
	data["Data"] = recipient_data
	c.HTML(http.StatusOK, "backend/admin/recipients/instances_update.tmpl", data)
}

func RecipientsInstancesTest(c *gin.Context) {
	id := c.Query("id")
	idInt, _ := strconv.ParseInt(id, 10, 64)
	recipient_data, _ := db.GetAdminRecipientsInstancesByID(idInt)

	data := common.CommonVer(c)
	data["id"] = id
	data["Data"] = recipient_data

	c.HTML(http.StatusOK, "backend/admin/recipients/instances_test.tmpl", data)
}

func PostRecipientsInstancesTest(c *gin.Context) {
	var field form.AdminRecipientsInstancesTest
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	if field.ID < 1 {
		common.ErrorResp(c, errors.New("请求异常!"), -1)
		return
	}

	recipient_data, _ := db.GetAdminRecipientsInstancesByID(field.ID)
	if recipient_data.MediaType == "telegram" {
		ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
		defer cancel()
		tp, _ := recipient_data.GetTelegramParams()
		notify_test, err := notify.NewNotification(tp.Token, field.SendID, recipient_data.GetTelegramProxy(), true)
		if err != nil {
			common.ErrorResp(c, err, -1)
			return
		}
		err = notify_test.Send(ctx, field.Title, field.Content)
		if err != nil {
			common.ErrorResp(c, err, -2)
			return
		}
	}
	common.SuccessResp(c)
}

func PostRecipientsInstancesAdd(c *gin.Context) {
	var field form.AdminRecipientsInstances
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	common_data := &model.AdminMediaInstance{
		Name:       field.Name,
		MediaType:  field.MediaType,
		Status:     field.Status,
		Mark:       field.Mark,
		HashLife:   field.HashLife,
		UpdateTime: time.Now().Unix(),
	}

	if field.MediaType == "telegram" {
		common_data.SetTelegramParams(model.AdminMediaTelegramParams{
			Token:               field.Token,
			SendID:              fmt.Sprintf("%d", field.SendID),
			TelegramProxyScheme: field.TelegramProxyScheme,
			TelegramProxyValue:  field.TelegramProxyValue,
		})
	}

	if field.MediaType == "email" {
		common_data.SetEmailParams(model.AdminMediaEmailParams{
			Smtp:     field.EmailSmtp,
			Username: field.EmailUsername,
			Password: field.EmailPassword,
			From:     field.EmailFrom,
		})
	}

	if field.MediaType == "webhook" {
		common_data.SetWebhookParams(model.AdminMediaWebhookParams{
			Url:    field.WebhookUrl,
			Method: field.WebhookMethod,
		})
	}

	common_data.SetRate(model.AdminMediaRateParams{
		Count:   field.Count,
		Minutes: field.Minutes,
	})

	if field.ID > 0 {
		if err := db.GetDb().Model(&model.AdminMediaInstance{}).Where("id = ?", field.ID).Updates(common_data).Error; err != nil {
			common.ErrorResp(c, err, -1)
			return
		}
		common.SuccessResp(c)
		return
	}
	common_data.Status = true
	common_data.CreateTime = time.Now().Unix()
	if err := db.GetDb().Create(common_data).Error; err != nil {
		common.ErrorResp(c, err, -1)
		return
	}
	common.SuccessResp(c)
}

func RecipientsInstancesList(c *gin.Context) {
	result, count, _ := db.GetAdminRecipientsInstancesList(1, 10)
	common.SuccessLayuiResp(c, count, "ok", result)
}

func RecipientsInstancesDelete(c *gin.Context) {
	var field form.ID
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	err := db.AdminRecipientsInstancesDeleteByID(nil, field.ID)
	if err == nil {
		common.SuccessResp(c)
		return
	}
	common.ErrorResp(c, err, -1)
}
