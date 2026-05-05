package conf

// "net/url"
// "os"

// CustomConf returns the absolute path of custom configuration file that is used.
var CustomConf string

// Build time and commit information.
//
// ⚠️ WARNING: should only be set by "-ldflags".
var (
	BuildTime   string
	BuildCommit string
)

var (
	App struct {
		Version string `yaml:"-"`

		Name      string `yaml:"app_name"`
		BrandName string `yaml:"brand_name"`
		RunUser   string `yaml:"run_user"`
		RunMode   string `yaml:"run_mode"`
		Debug     bool   `yaml:"debug"`
	}

	General struct {
		MenuFile string `yaml:"menu_file"`
	}

	Log struct {
		Format   string `yaml:"format"`
		RootPath string `yaml:"root_path"`
	}

	Cache struct {
		Adapter  string `yaml:"adapter"`
		Interval int    `yaml:"interval"`
		Host     string `yaml:"host"`
	}

	Database struct {
		Type        string `yaml:"type"`
		Path        string `yaml:"path"`
		DSN         string `yaml:"dsn"`
		TablePrefix string `yaml:"table_prefix"`
		Hostname    string `yaml:"hostname"`
		Hostport    int64  `yaml:"hostport"`
		Name        string `yaml:"name"`
		User        string `yaml:"user"`
		Password    string `yaml:"password"`
		SSLMode     string `yaml:"ssl_mode"`
	}

	Web struct {
		HTTPAddr   string `yaml:"http_addr"`
		HTTPPort   int    `yaml:"http_port"`
		AdminPath  string `yaml:"admin_path"`
		EnableGzip bool   `yaml:"enable_gzip"`
	}

	Session struct {
		Provider       string `yaml:"provider"`
		ProviderConfig string `yaml:"provider_config"`
		CookieName     string `yaml:"cookie_name"`
		CookieSecure   bool   `yaml:"cookie_secure"`
		GCInterval     int64  `yaml:"gc_interval"`
		MaxLifeTime    int64  `yaml:"max_life_time"`
		CSRFCookieName string `yaml:"csrf_cookie_name"`
	}

	Security struct {
		InstallLock             bool   `yaml:"install_lock"`
		SecretKey               string `yaml:"secret_key"`
		LoginRememberDays       int    `yaml:"login_remember_days"`
		CookieRememberName      string `yaml:"cookie_remember_name"`
		CookieUsername          string `yaml:"cookie_username"`
		CookieSecure            bool   `yaml:"cookie_secure"`
		EnableLoginStatusCookie bool   `yaml:"enable_login_status_cookie"`
		LoginStatusCookieName   string `yaml:"login_status_cookie_name"`
	}
)
