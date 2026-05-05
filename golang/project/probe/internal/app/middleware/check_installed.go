package middleware

import (
	"github.com/gin-gonic/gin"

	"aiprobe/internal/conf"
)

func CheckInstalled() gin.HandlerFunc {
	return func(c *gin.Context) {
		if !conf.Security.InstallLock {
			c.Redirect(302, "/install/index")
			c.Abort()
			return
		}
		c.Next()
	}
}
