package app

import (
	"fmt"
	"html/template"
	"io/fs"
	"net/http"
	"strings"

	// "time"

	"github.com/gin-contrib/gzip"
	"github.com/gin-contrib/sessions"
	"github.com/gin-contrib/sessions/cookie"
	"github.com/gin-gonic/gin"

	"aiprobe/embed"
	"aiprobe/internal/app/handles/install"
	"aiprobe/internal/app/middleware"
	"aiprobe/internal/conf"

	api_logs "aiprobe/internal/app/handles/api/logs"
	api_tools "aiprobe/internal/app/handles/api/tools"
	backend "aiprobe/internal/app/handles/backend"
	backend_ad "aiprobe/internal/app/handles/backend/ad"
	backend_admin "aiprobe/internal/app/handles/backend/admin"
	backend_cluster "aiprobe/internal/app/handles/backend/cluster"
	backend_log "aiprobe/internal/app/handles/backend/log"
	backend_system "aiprobe/internal/app/handles/backend/system"

	frontend "aiprobe/internal/app/handles/frontend"
)

func initTemp(r *gin.Engine) {
	defer func() {
		if r := recover(); r != nil {
		}
	}()

	// Define template functions
	funcMap := template.FuncMap{
		"safe": func(str string) template.HTML {
			return template.HTML(str)
		},
		// Cache-busting token exposed as a function for templates
		"BuildCommit": func() string {
			return conf.BuildCommit
		},
		"HasPrefix": func(s, prefix string) bool {
			return strings.HasPrefix(s, prefix)
		},
		//是子菜单或当前菜单
		"IsSubOrEq": func(base, menu string) bool {
			if base == menu {
				return true
			}
			endp := strings.Replace(base, menu, "", 1)
			endp = strings.TrimPrefix(endp, "/")
			return !strings.Contains(endp, "/")
		},
		"Contains": func(s, substr string) bool {
			return strings.Contains(s, substr)
		},
	}

	// Build template set with directory-aware names (e.g., "install/index.tmpl")
	// so that we can reference templates across multiple directories explicitly.
	tpl := template.New("").Delims("{[", "]}").Funcs(funcMap)

	for _, name := range embed.TemplatesAllNames("templates") {
		// Trim the leading "templates/" so template names are like "install/index.tmpl"
		short := strings.TrimPrefix(name, "templates/")
		content, err := embed.Templates.ReadFile(name)
		if err != nil {
			continue
		}
		if _, err := tpl.New(short).Parse(string(content)); err != nil {
			fmt.Printf("failed to parse template %s: %v\n", short, err)
			continue
		}
	}

	r.SetHTMLTemplate(tpl)
}

// 后台/backstage
func initRuoteAdmin(r *gin.Engine) {
	backstage := r.Group(conf.Web.AdminPath)
	backstage.Use(middleware.CheckInstalled())
	backstage.GET("/login", backend.LoginPage)
	backstage.POST("/login", backend.PostLogin)
	backstage.GET("/logout", backend.LoginOut)

	backstage_admin := backstage.Group("")
	backstage_admin.Use(middleware.CheckInstalled(), middleware.AuthRequired())

	// 控制台
	backstage_admin.GET("/console", backend.ConsoleIndex)

	// 管理员
	backstage_admin.GET("", backend_admin.Home)
	backstage_admin.GET("/index", backend_admin.Home)
	backstage_admin.GET("/admin/index", backend_admin.Home)

	backstage_admin.GET("/admin/add", backend_admin.Add)
	backstage_admin.POST("/admin/add", backend_admin.PostAdd)
	backstage_admin.GET("/admin/list", backend_admin.List)
	backstage_admin.GET("/admin/details", backend_admin.Details)
	backstage_admin.GET("/admin/update", backend_admin.Update)
	backstage_admin.POST("/admin/delete", backend_admin.Delete)
	backstage_admin.POST("/admin/trigger_status", backend_admin.AdminTriggerStatus)

	// 管理员 - 通知
	backstage_admin.GET("/admin/recipients", backend_admin.Recipients)
	backstage_admin.GET("/admin/recipients/list", backend_admin.RecipientsList)
	backstage_admin.POST("/admin/recipients/delete", backend_admin.RecipientsDelete)
	backstage_admin.GET("/admin/recipients/add", backend_admin.RecipientsAdd)
	backstage_admin.POST("/admin/recipients/add", backend_admin.PostRecipientsAdd)
	backstage_admin.GET("/admin/recipients/groups", backend_admin.RecipientsGroups)
	backstage_admin.GET("/admin/recipients/groups/list", backend_admin.RecipientsGroupsList)
	backstage_admin.GET("/admin/recipients/groups/select", backend_admin.RecipientsGroupsSelect)
	backstage_admin.GET("/admin/recipients/groups/add", backend_admin.RecipientsGroupsAdd)
	backstage_admin.POST("/admin/recipients/groups/add", backend_admin.PostRecipientsGroupsAdd)
	backstage_admin.POST("/admin/recipients/groups/delete", backend_admin.PostRecipientsGroupsDelete)
	backstage_admin.GET("/admin/recipients/instances", backend_admin.RecipientsInstances)
	backstage_admin.GET("/admin/recipients/instances/list", backend_admin.RecipientsInstancesList)
	backstage_admin.GET("/admin/recipients/instances/add", backend_admin.RecipientsInstancesAdd)
	backstage_admin.POST("/admin/recipients/instances/add", backend_admin.PostRecipientsInstancesAdd)
	backstage_admin.GET("/admin/recipients/instances/details", backend_admin.RecipientsInstancesDetails)
	backstage_admin.GET("/admin/recipients/instances/update", backend_admin.RecipientsInstancesUpdate)
	backstage_admin.GET("/admin/recipients/instances/test", backend_admin.RecipientsInstancesTest)
	backstage_admin.POST("/admin/recipients/instances/test", backend_admin.PostRecipientsInstancesTest)
	backstage_admin.POST("/admin/recipients/instances/delete", backend_admin.RecipientsInstancesDelete)

	backstage_admin.GET("/admin/recipients/recipients/details", backend_admin.RecipientsRecipientsDetails)
	backstage_admin.GET("/admin/recipients/recipients/update", backend_admin.RecipientsRecipientsUpdate)
	backstage_admin.GET("/admin/recipients/recipients/test", backend_admin.RecipientsRecipientsTest)

	backstage_admin.GET("/admin/recipients/tasks", backend_admin.RecipientsTasks)
	backstage_admin.GET("/admin/recipients/logs", backend_admin.RecipientsLogs)

	// 广告管理
	backstage_admin.GET("/ad/index", backend_ad.Home)
	backstage_admin.GET("/ad/list", backend_ad.List)
	backstage_admin.GET("/ad/add", backend_ad.Add)
	backstage_admin.POST("/ad/add", backend_ad.PostAdd)
	backstage_admin.GET("/ad/update", backend_ad.Update)
	backstage_admin.POST("/ad/update", backend_ad.PostUpdate)
	backstage_admin.POST("/ad/delete", backend_ad.Delete)
	backstage_admin.POST("/ad/trigger_status", backend_ad.TriggerStatus)

	// 边缘节点
	backstage_admin.GET("/clusters", backend_cluster.Home)
	backstage_admin.POST("/clusters/create", backend_cluster.PostCreate)
	backstage_admin.GET("/clusters/list", backend_cluster.List)
	backstage_admin.POST("/clusters/delete", backend_cluster.Delete)

	backstage_admin.GET("/clusters/cluster/boards", backend_cluster.ClusterBoards)
	backstage_admin.GET("/clusters/cluster/list", backend_cluster.ClusterList)

	// 边缘节点 - 节点
	backstage_admin.GET("/clusters/node", backend_cluster.Node)
	backstage_admin.GET("/clusters/node/list", backend_cluster.NodeList)
	backstage_admin.GET("/clusters/node/boards", backend_cluster.NodeBoards)
	backstage_admin.GET("/clusters/node/details", backend_cluster.NodeBoards)
	backstage_admin.GET("/clusters/node/install", backend_cluster.NodeInstall)
	backstage_admin.POST("/clusters/node/install_update_status", backend_cluster.PostNodeInstallUpdateStatus)
	backstage_admin.POST("/clusters/node/doinstall", backend_cluster.PostNodeInstallDone)
	backstage_admin.GET("/clusters/node/install_status", backend_cluster.GetNodeInstallStatus)
	backstage_admin.GET("/clusters/node/logs", backend_cluster.NodeLogs)
	backstage_admin.GET("/clusters/node/logs/list", backend_cluster.NodeLogsList)
	backstage_admin.GET("/clusters/node/settings", backend_cluster.NodeSettings)
	backstage_admin.POST("/clusters/node/settings", backend_cluster.PostNodeSettings)
	backstage_admin.GET("/clusters/node/settings/ssh", backend_cluster.NodeSettingsSsh)
	backstage_admin.POST("/clusters/node_login_add", backend_cluster.PostNodeLoginAdd)

	backstage_admin.GET("/clusters/ipaddr", backend_cluster.Node)
	backstage_admin.GET("/clusters/create", backend_cluster.Create)
	backstage_admin.GET("/clusters/select/ip", backend_cluster.SelectIp)
	backstage_admin.GET("/clusters/select/region", backend_cluster.SelectRegion)
	backstage_admin.GET("/clusters/select/groups", backend_cluster.SelectGroups)
	backstage_admin.GET("/clusters/select/ssh", backend_cluster.SelectSsh)
	backstage_admin.GET("/clusters/cluster/create_node", backend_cluster.CreateNode)
	backstage_admin.POST("/clusters/cluster/create_node", backend_cluster.PostCreateNode)
	backstage_admin.POST("/clusters/cluster/delete_node", backend_cluster.PostDeleteNode)

	// 边缘节点 - 分组
	backstage_admin.GET("/clusters/cluster/groups", backend_cluster.ClusterGroups)
	backstage_admin.GET("/clusters/cluster/groups/add", backend_cluster.ClusterGroupsAdd)
	backstage_admin.GET("/clusters/cluster/groups/list", backend_cluster.ClusterGroupsList)
	backstage_admin.POST("/clusters/cluster/groups/add", backend_cluster.PostClusterGroupsAdd)
	backstage_admin.POST("/clusters/cluster/groups/delete", backend_cluster.ClusterGroupsDelete)

	backstage_admin.GET("/clusters/cluster/install", backend_cluster.ClusterInstall)
	backstage_admin.GET("/clusters/cluster/upgrade", backend_cluster.ClusterUpgrade)

	// 边缘节点 - 设置
	backstage_admin.GET("/clusters/cluster/settings", backend_cluster.ClusterSettings)
	backstage_admin.POST("/clusters/cluster/settings", backend_cluster.PostClusterSettings)
	backstage_admin.GET("/clusters/cluster/settings/health", backend_cluster.ClusterSettingsHealth)

	backstage_admin.GET("/clusters/cluster/delete", backend_cluster.ClusterDelete)

	// 边缘节点 - 区域设置
	backstage_admin.GET("/clusters/regions", backend_cluster.ClusterRegions)
	backstage_admin.GET("/clusters/regions/list", backend_cluster.ClusterRegionsList)
	backstage_admin.GET("/clusters/regions/add", backend_cluster.ClusterRegionsAdd)
	backstage_admin.GET("/clusters/regions/nodes", backend_cluster.ClusterRegionsNodes)
	backstage_admin.POST("/clusters/regions/add", backend_cluster.PostClusterRegionsNodesAdd)
	backstage_admin.POST("/clusters/regions/delete", backend_cluster.ClusterRegionsDelete)
	backstage_admin.POST("/clusters/regions/trigger_status", backend_cluster.ClusterRegionsTriggerStatus)

	//边缘节点 - 认证
	backstage_admin.GET("/clusters/ssh", backend_cluster.ClusterSsh)
	backstage_admin.GET("/clusters/ssh/list", backend_cluster.ClusterSshList)
	backstage_admin.GET("/clusters/ssh/select_list", backend_cluster.ClusterSshSelectList)
	backstage_admin.GET("/clusters/ssh/details", backend_cluster.ClusterSshDetails)
	backstage_admin.GET("/clusters/ssh/test", backend_cluster.ClusterSshTest)
	backstage_admin.GET("/clusters/ssh/add", backend_cluster.ClusterSshAdd)
	backstage_admin.GET("/clusters/ssh/node_pop_add", backend_cluster.ClusterNodeSshPopAdd)
	backstage_admin.GET("/clusters/ssh/create", backend_cluster.ClusterSshCreate)
	backstage_admin.POST("/clusters/ssh/create", backend_cluster.PostClusterSshCreate)
	backstage_admin.GET("/clusters/ssh/update", backend_cluster.ClusterSshUpdate)

	// 日志审计
	backstage_admin.GET("/log", backend_log.Home)
	backstage_admin.GET("/log/list", backend_log.List)
	backstage_admin.GET("/log/settings", backend_log.Settings)
	backstage_admin.POST("/log/settings", backend_log.PostSettting)
	backstage_admin.GET("/log/clean", backend_log.Clean)
	backstage_admin.POST("/log/clean", backend_log.PostLogClean)
	backstage_admin.POST("/log/delete", backend_log.Delete)

	// 系统设置
	backstage_admin.GET("/system/settings", backend_system.Home)
	backstage_admin.POST("/system/settings/home", backend_system.PostHome)

	backstage_admin.GET("/system/settings/web", backend_system.Web)
	backstage_admin.POST("/system/settings/web", backend_system.PostWeb)

	backstage_admin.GET("/system/settings/profile", backend_system.Profile)
	backstage_admin.POST("/system/settings/profile", backend_system.PostProfile)

	backstage_admin.GET("/system/settings/login", backend_system.Login)
	backstage_admin.POST("/system/settings/login", backend_system.PostLogin)
	backstage_admin.GET("/system/settings/login/logs", backend_system.LoginLogs)
	backstage_admin.GET("/system/settings/login/logs/list", backend_system.LoginLogsList)

	backstage_admin.GET("/system/database", backend_system.Database)
	backstage_admin.GET("/system/database/index", backend_system.Database)
	// backstage_admin.GET("/system/database/update", backend_system.DatabaseUpdate)
	backstage_admin.GET("/system/database/cleans", backend_system.DatabaseClean)
	backstage_admin.GET("/system/database/list", backend_system.DatabaseList)
	backstage_admin.POST("/system/database/clean", backend_system.PostDatabaseClean)
	backstage_admin.POST("/system/database/delete", backend_system.PostDatabaseDelete)

	backstage_admin.GET("/system/database/clean_setting", backend_system.DatabaseCleanSetting)
	backstage_admin.POST("/system/database/clean_setting", backend_system.PostDatabaseCleanSetting)

	backstage_admin.GET("/system/db", backend_system.Db)
	backstage_admin.GET("/system/db/list", backend_system.DbNodeList)
	backstage_admin.GET("/system/db/add", backend_system.DbNodeAdd)
	backstage_admin.POST("/system/db/add", backend_system.PostDbNodeAdd)
	backstage_admin.GET("/system/db/details", backend_system.DbNodeDetails)
	backstage_admin.GET("/system/db/clean", backend_system.DbNodeClean)
	backstage_admin.GET("/system/db/logs", backend_system.DbNodeLogs)
	backstage_admin.GET("/system/db/update", backend_system.DbNodeUpdate)
}

func initRuoteInstall(r *gin.Engine) {
	installGroup := r.Group("/install")
	installGroup.Use(middleware.CheckInstalledAfter())
	installGroup.GET("/index", install.HomePage)
	installGroup.POST("/step1", install.PostInstallStep1)
	installGroup.POST("/dbtest", install.MyDbtest)
}

func initRuoteFrontend(r *gin.Engine) {
	api := r.Group("/api")
	api.POST("/logs", api_logs.LogsAdd)

	frontendApi := r.Group("/api/frontend")
	frontendApi.POST("/ping", api_tools.Ping)
	frontendApi.POST("/tcping", api_tools.Tcping)
	frontendApi.POST("/speedtest", api_tools.Speedtest)
	frontendApi.POST("/traceroute", api_tools.Traceroute)
	frontendApi.POST("/dns", api_tools.DNSQuery)
	frontendApi.POST("/findping", api_tools.FindPing)
	frontendApi.GET("/network/info", api_tools.NetworkInfo)
	frontendApi.POST("/batch", api_tools.BatchCheck)

	r.GET("/ping", func(c *gin.Context) {
		c.String(200, "pong")
	})

	frontendGroup := r.Group("/frontend")
	frontendGroup.Use(middleware.CheckInstalled())
	frontendGroup.GET("/ping", frontend.Ping)
	frontendGroup.GET("/tcping", frontend.Tcping)
	frontendGroup.GET("/speedtest", frontend.Speedtest)
	frontendGroup.GET("/traceroute", frontend.Traceroute)
	frontendGroup.GET("/dns", frontend.DNS)
	frontendGroup.GET("/findping", frontend.FindPing)
	frontendGroup.GET("/network", frontend.Network)
	frontendGroup.GET("/batch", frontend.Batch)
	frontendGroup.GET("/help", frontend.Help)
	frontendGroup.GET("/settings", frontend.Settings)

	r.Use(middleware.CheckInstalled()).GET("/", frontend.Home)
}

func initRuote(r *gin.Engine) {
	defer func() {
		if r := recover(); r != nil {
		}
	}()
	// Static files from embedded filesystem subdir "static"
	staticFS, err := fs.Sub(embed.Static, "static")
	if err == nil {
		r.StaticFS("/static", http.FS(staticFS))
	}

	initRuoteAdmin(r)
	initRuoteInstall(r)
	initRuoteFrontend(r)
}

func Run() {
	if conf.App.RunMode == "prod" {
		gin.SetMode(gin.ReleaseMode)
	}

	r := gin.New()

	// 初始化 session 存储
	store := cookie.NewStore([]byte(conf.Security.SecretKey))
	// 设置 cookie 选项
	store.Options(sessions.Options{
		Path:     "/",
		MaxAge:   int(conf.Session.MaxLifeTime),
		HttpOnly: true,
		Secure:   conf.Session.CookieSecure,
		SameSite: http.SameSiteLaxMode,
	})
	r.Use(sessions.Sessions(conf.Session.CookieName, store))

	if conf.Web.EnableGzip {
		r.Use(gzip.Gzip(gzip.DefaultCompression))
	}

	r.Use(gin.Recovery())
	r.SetTrustedProxies(nil)

	initTemp(r)
	initRuote(r)
	// fmt.Println("conf.Web.HTTPPort:", conf.Web.HTTPPort)
	r.Run(fmt.Sprintf(":%d", conf.Web.HTTPPort))
}
