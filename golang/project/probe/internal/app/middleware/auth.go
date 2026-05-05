package middleware

import (
	"fmt"
	"net/http"
	"strconv"
	"strings"

	"github.com/gin-contrib/sessions"
	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/conf"
	"aiprobe/internal/db"
)

func AuthRequired() gin.HandlerFunc {
	return func(c *gin.Context) {
		session := sessions.Default(c)
		uid := session.Get("user_id")
		// fmt.Println(uid)

		var idint int64
		switch v := uid.(type) {
		case int64:
			idint = v
		case int:
			idint = int64(v)
		case uint:
			idint = int64(v)
		case uint64:
			idint = int64(v)
		case string:
			// optional: parse string id
			// ignore error, keep 0 when invalid
			if n, err := strconv.ParseInt(v, 10, 64); err == nil {
				idint = n
			}
		}

		// Require valid admin id; session flag is not authoritative
		if idint < 1 {
			path := conf.Web.AdminPath
			if !strings.HasPrefix(path, "/") {
				path = "/" + path
			}
			c.Redirect(http.StatusFound, fmt.Sprintf("%s/login", path))
			c.Abort()
			return
		}

		// Permission check by menu subapi
		// Build full path helper
		adminPath := conf.Web.AdminPath
		full := func(p string) string {
			if adminPath != "" {
				if !strings.HasPrefix(p, "/") {
					p = "/" + p
				}
				return "/" + adminPath + p
			}
			return p
		}

		menus := common.GetMenus()

		// Build known sets
		apiPaths := map[string]bool{}
		var walk func(ms []common.MenuConf)
		walk = func(ms []common.MenuConf) {
			for _, m := range ms {
				for _, s := range m.SubApi {
					if s.Path != "" {
						apiPaths[full(s.Path)] = true
					}
				}
				if len(m.Children) > 0 {
					walk(m.Children)
				}
			}
		}

		// Super admin bypass
		isSuper := false
		if u, err := db.GetAdminByID(idint); err == nil {

			isSuper = u.SuperAdmin
			if !isSuper && u.ID != 1 {
				// First derive permitted menu tree from auth codes, so that children under authorized parent are kept
				allowedCodes := common.ParseAuthCodes(u.Auth)
				allowedMenus := common.FilterMenusByCodes(menus, allowedCodes)

				// Build allowed path set from filtered menus
				allowed := map[string]bool{}
				var collectAllowed func(ms []common.MenuConf)
				collectAllowed = func(ms []common.MenuConf) {
					for _, m := range ms {
						if m.Path != "" {
							allowed[full(m.Path)] = true
						}
						for _, s := range m.SubApi {
							if s.Path != "" {
								allowed[full(s.Path)] = true
							}
						}
						if len(m.Children) > 0 {
							collectAllowed(m.Children)
						}
					}
				}
				collectAllowed(allowedMenus)
				walk(allowedMenus)

				reqPath := c.Request.URL.Path
				method := strings.ToUpper(c.Request.Method)
				if method != "POST" {
					c.Next()
					return
				}
				if allowed[reqPath] {
					c.Next()
					return
				}

				if !apiPaths[reqPath] {
					common.ErrorStrResp(c, "no permission", -1)
					return
				}
				// If path unknown, allow pass-through
				c.Next()
				return
			}
		}

		// super admin or unknown admin; allow
		c.Next()
	}
}
