package backend

import (
	// "fmt"
	"errors"
	"net/http"

	"github.com/gin-contrib/sessions"
	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/conf"
	"aiprobe/internal/db"
	// "aiprobe/internal/op"
)

type LoginReq struct {
	Username string `json:"username" binding:"required"`
	Password string `json:"password" binding:"required"`
}

func HomePage(c *gin.Context) {
	data := common.CommonVer(c)
	c.HTML(http.StatusOK, "backend/index.tmpl", data)
}

func LoginPage(c *gin.Context) {
	data := common.CommonVer(c)
	c.HTML(http.StatusOK, "backend/login.tmpl", data)
}

func LoginOut(c *gin.Context) {
	session := sessions.Default(c)
	session.Clear()
	session.Save()
	c.Redirect(http.StatusFound, "/"+conf.Web.AdminPath+"/login")
}

func PostLogin(c *gin.Context) {
	var req LoginReq
	if err := c.ShouldBind(&req); err != nil {
		common.ErrorResp(c, err, 400)
		return
	}
	loginHash(c, &req)
}

func loginHash(c *gin.Context, req *LoginReq) {
	// check username
	user, err := db.GetAdminByName(req.Username)
	if err != nil {
		common.ErrorResp(c, err, 400)
		return
	}
	// validate password hash
	if err := user.ValidatePwdStaticHash(req.Password); err != nil {
		common.ErrorResp(c, errors.New("认证失败"), 400)
		return
	}

	// 保存用户信息到 session
	session := sessions.Default(c)
	session.Set("user_id", user.ID)
	session.Set("username", user.Username)
	session.Set("logged_in", true)

	if err := session.Save(); err != nil {
		c.JSON(500, gin.H{"error": "磁盘异常!"})
		return
	}

	common.SuccessResp(c, gin.H{"message": "登录成功", "user": user})
}
