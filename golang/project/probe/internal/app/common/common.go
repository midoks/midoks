package common

import (
	"context"
	"encoding/json"

	// "fmt"
	"net/http"
	// "strings"

	"aiprobe/internal/conf"
	"aiprobe/internal/db"

	// utils "aiprobe/internal/utils"

	"github.com/gin-contrib/sessions"
	"github.com/gin-gonic/gin"
	log "github.com/sirupsen/logrus"
)

type Resp[T any] struct {
	Code int    `json:"code"`
	Msg  string `json:"msg"`
	Data T      `json:"data"`
}

type PageResp struct {
	Content interface{} `json:"content"`
	Total   int64       `json:"total"`
}

type LayuiResp[T any] struct {
	Code  int    `json:"code"`
	Count int64  `json:"count"`
	Msg   string `json:"msg"`
	Data  T      `json:"data"`
}

func ParseAdminId(login_uid interface{}) int64 {
	var admin_id int64
	switch v := login_uid.(type) {
	case int64:
		admin_id = v
	case int:
		admin_id = int64(v)
	case uint:
		admin_id = int64(v)
	case uint64:
		admin_id = int64(v)
	}
	return admin_id
}

func CommonVer(c *gin.Context) map[string]interface{} {
	data := map[string]interface{}{
		"title":   "Web",
		"version": conf.App.Version,
	}

	session := sessions.Default(c)
	username := session.Get("username")
	login_uid := session.Get("user_id")

	data["login_name"] = username
	admin_id := ParseAdminId(login_uid)
	data["login_uid"] = admin_id

	menus := GetMenus()
	if db.GetDb() != nil {
		if admin_data, err := db.GetAdminByID(admin_id); err == nil {
			data["login_data"] = admin_data
			if !admin_data.SuperAdmin && admin_data.ID != 1 {
				allowed := ParseAuthCodes(admin_data.Auth)
				menus = FilterMenusByCodes(menus, allowed)
			}
		}
	}

	data["admin_path"] = conf.Web.AdminPath
	data["Menus"] = menus
	data["CurrentPath"] = c.Request.URL.Path
	data["ActiveMenu"] = FindMenuCodeByPath(c.Request.URL.Path, conf.Web.AdminPath)

	if db.GetDb() != nil {
		setting_admin_ui_data, err := db.GetSysSettingByCode(db.SettingAdminUI)
		if err == nil {
			data["setting_admin_ui"] = setting_admin_ui_data
		}
	}
	return data
}

func ToJson(v interface{}) (d string) {
	rdata, _ := json.MarshalIndent(v, "", "  ")
	return string(rdata)
}

// ErrorResp is used to return error response
// @param l: if true, log error
func ErrorResp(c *gin.Context, err error, code int, l ...bool) {
	ErrorWithDataResp(c, err, code, nil, l...)
}

func hidePrivacy(msg string) string {
	// for _, r := range conf.PrivacyReg {
	// 	msg = r.ReplaceAllStringFunc(msg, func(s string) string {
	// 		return strings.Repeat("*", len(s))
	// 	})
	// }
	return msg
}

func ErrorWithDataResp(c *gin.Context, err error, code int, data interface{}, l ...bool) {
	if len(l) > 0 && l[0] {
		if conf.App.Debug {
			log.Errorf("%+v", err)
		} else {
			log.Errorf("%v", err)
		}
	}
	c.JSON(200, Resp[interface{}]{
		Code: code,
		Msg:  hidePrivacy(err.Error()),
		Data: data,
	})
	c.Abort()
}

func ErrorStrResp(c *gin.Context, str string, code int, l ...bool) {
	if len(l) != 0 && l[0] {
		log.Error(str)
	}
	c.JSON(200, Resp[interface{}]{
		Code: code,
		Msg:  hidePrivacy(str),
		Data: nil,
	})
	c.Abort()
}

func SuccessResp(c *gin.Context, data ...interface{}) {
	if len(data) == 0 {
		c.JSON(200, Resp[interface{}]{
			Code: 200,
			Msg:  "success",
			Data: nil,
		})
		return
	}
	c.JSON(200, Resp[interface{}]{
		Code: 200,
		Msg:  "success",
		Data: data[0],
	})
}

func SuccessLayuiMsgResp(c *gin.Context, msg string, data ...interface{}) {
	if len(data) == 0 {
		c.JSON(200, LayuiResp[interface{}]{
			Code: 0,
			Msg:  msg,
			Data: nil,
		})
		return
	}

	c.JSON(200, LayuiResp[interface{}]{
		Code: 0,
		Msg:  msg,
		Data: data[0],
	})
}

func SuccessLayuiResp(c *gin.Context, count int64, msg string, data ...interface{}) {
	if len(data) == 0 {
		c.JSON(200, LayuiResp[interface{}]{
			Code:  0,
			Count: count,
			Msg:   msg,
			Data:  nil,
		})
		return
	}

	c.JSON(200, LayuiResp[interface{}]{
		Code:  0,
		Count: count,
		Msg:   msg,
		Data:  data[0],
	})
}

func GetHttpReq(ctx context.Context) *http.Request {
	if c, ok := ctx.(*gin.Context); ok {
		return c.Request
	}
	return nil
}
