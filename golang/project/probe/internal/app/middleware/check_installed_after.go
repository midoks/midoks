package middleware

import (
	"github.com/gin-gonic/gin"

	"aiprobe/internal/conf"
)

func CheckInstalledAfter() gin.HandlerFunc {
	return func(c *gin.Context) {
		if conf.Security.InstallLock {
			c.Redirect(302, "/")
			// c.String(200, "Installed")
			c.Abort()
			return
		}
		c.Next()
	}
}
