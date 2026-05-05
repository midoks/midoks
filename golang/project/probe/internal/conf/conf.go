package conf

import (
	"os"
	"path/filepath"
	"strconv"
	"strings"

	"github.com/goccy/go-yaml"
	"github.com/pkg/errors"

	"aiprobe/embed"
)

var appConfig AppConfig

type AppConfig struct {
	AppName   string         `yaml:"app_name"`
	BrandName string         `yaml:"brand_name"`
	RunUser   string         `yaml:"run_user"`
	RunMode   string         `yaml:"run_mode"`
	General   GeneralConfig  `yaml:"general"`
	Log       LogConfig      `yaml:"log"`
	Session   SessionConfig  `yaml:"session"`
	Web       WebConfig      `yaml:"web"`
	Database  DatabaseConfig `yaml:"database"`
	Security  SecurityConfig `yaml:"security"`
}

type AppConfigCustom struct {
	AppName   string         `yaml:"app_name"`
	BrandName string         `yaml:"brand_name"`
	RunUser   string         `yaml:"run_user"`
	RunMode   string         `yaml:"run_mode"`
	Log       LogConfig      `yaml:"log"`
	Session   SessionConfig  `yaml:"session"`
	Web       WebConfig      `yaml:"web"`
	Security  SecurityConfig `yaml:"security"`
	Database  DatabaseConfig `yaml:"database"`
}

type GeneralConfig struct {
	MenuFile string `yaml:"menu_file"`
}

type LogConfig struct {
	Format   string `yaml:"format"`
	RootPath string `yaml:"root_path"`
}

type SessionConfig struct {
	Provider       string `yaml:"provider"`
	ProviderConfig string `yaml:"provider_config"`
	CookieName     string `yaml:"cookie_name"`
	CookieSecure   bool   `yaml:"cookie_secure"`
	GCInterval     int64  `yaml:"gc_interval"`
	MaxLifeTime    int64  `yaml:"max_life_time"`
	CSRFCookieName string `yaml:"csrf_cookie_name"`
}

type WebConfig struct {
	HTTPAddr   string `yaml:"http_addr"`
	HTTPPort   int    `yaml:"http_port"`
	AdminPath  string `yaml:"admin_path"`
	EnableGzip bool   `yaml:"enable_gzip"`
}

type DatabaseConfig struct {
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

type SecurityConfig struct {
	InstallLock             bool   `yaml:"install_lock"`
	SecretKey               string `yaml:"secret_key"`
	LoginRememberDays       int    `yaml:"login_remember_days"`
	CookieRememberName      string `yaml:"cookie_remember_name"`
	CookieUsername          string `yaml:"cookie_username"`
	CookieSecure            bool   `yaml:"cookie_secure"`
	EnableLoginStatusCookie bool   `yaml:"enable_login_status_cookie"`
	LoginStatusCookieName   string `yaml:"login_status_cookie_name"`
}

func ReadConf() error {
	data, err := embed.Conf.ReadFile("conf/app.yaml")
	if err != nil {
		return errors.Wrap(err, "read file 'conf/app.yaml'")
	}

	err = yaml.Unmarshal(data, &appConfig)
	if err != nil {
		return errors.Wrap(err, "parse 'conf/app.yaml'")
	}
	return nil
}

func InstallConf(data map[string]string) error {
	err := ReadConf()
	if err != nil {
		return err
	}

	err = renderSection()
	if err != nil {
		return err
	}

	customConf := filepath.Join(CustomDir(), "conf", "app.yaml")

	if !isExist(filepath.Dir(customConf)) {
		err = os.MkdirAll(filepath.Dir(customConf), os.ModePerm)
		if err != nil {
			return errors.Wrap(err, "MkdirAll")
		}
	}

	appConfig.AppName = App.Name
	appConfig.BrandName = App.BrandName
	appConfig.RunUser = App.RunUser
	appConfig.RunMode = "prod"

	appConfig.Log.RootPath = Log.RootPath
	admin_path := "mgo"
	appConfig.Web.AdminPath = admin_path

	if strings.EqualFold(data["type"], "mysql") {
		appConfig.Database.Type = "mysql"
		appConfig.Database.Hostname = data["hostname"]
		hostport, _ := strconv.ParseInt(data["hostport"], 10, 64)
		appConfig.Database.Hostport = hostport
		appConfig.Database.Name = data["dbname"]
		appConfig.Database.User = data["username"]
		appConfig.Database.Password = data["password"]
		appConfig.Database.TablePrefix = data["table_prefix"]
	} else if strings.EqualFold(data["type"], "sqlite3") {
		appConfig.Database.Type = "sqlite3"
		appConfig.Database.Path = data["dbpath"]
		appConfig.Database.TablePrefix = data["table_prefix"]
	}

	appConfig.Security.InstallLock = true
	appConfig.Security.SecretKey = randString(32)

	saveConfig := AppConfigCustom{
		AppName:   appConfig.AppName,
		BrandName: appConfig.BrandName,
		RunUser:   appConfig.RunUser,
		RunMode:   appConfig.RunMode,
		Log:       appConfig.Log,
		Session:   appConfig.Session,
		Web:       appConfig.Web,
		Security:  appConfig.Security,
		Database:  appConfig.Database,
	}

	yamlData, err := yaml.Marshal(saveConfig)
	if err != nil {
		return errors.Wrap(err, "marshal yaml config")
	}

	if err := os.WriteFile(customConf, yamlData, os.ModePerm); err != nil {
		return errors.Wrap(err, "write custom config file")
	}

	err = InitConf("")
	if err != nil {
		return err
	}
	return nil
}

func InitConf(customConf string) error {
	data, err := embed.Conf.ReadFile("conf/app.yaml")
	if err != nil {
		return errors.Wrap(err, "read embedded config")
	}

	err = yaml.Unmarshal(data, &appConfig)
	if err != nil {
		return errors.Wrap(err, "parse 'conf/app.yaml'")
	}

	if customConf == "" {
		customConf = filepath.Join(CustomDir(), "conf", "app.yaml")
	} else {
		customConf, err = filepath.Abs(customConf)
		if err != nil {
			return errors.Wrap(err, "get absolute path")
		}
	}
	CustomConf = customConf

	if isFile(customConf) {
		customData, err := os.ReadFile(customConf)
		if err != nil {
			return errors.Wrapf(err, "read custom config %q", customConf)
		}

		err = yaml.Unmarshal(customData, &appConfig)
		if err != nil {
			return errors.Wrapf(err, "parse custom config %q", customConf)
		}
	}

	err = renderSection()
	if err != nil {
		return err
	}

	return nil
}

func renderSection() error {
	App.Name = appConfig.AppName
	App.BrandName = appConfig.BrandName
	App.RunUser = appConfig.RunUser
	App.RunMode = appConfig.RunMode

	General.MenuFile = appConfig.General.MenuFile

	Web.HTTPAddr = appConfig.Web.HTTPAddr
	Web.HTTPPort = appConfig.Web.HTTPPort
	Web.AdminPath = appConfig.Web.AdminPath
	Web.EnableGzip = appConfig.Web.EnableGzip

	Log.Format = appConfig.Log.Format
	Log.RootPath = appConfig.Log.RootPath

	Database.Type = appConfig.Database.Type
	Database.Path = appConfig.Database.Path
	Database.DSN = appConfig.Database.DSN
	Database.TablePrefix = appConfig.Database.TablePrefix
	Database.Hostname = appConfig.Database.Hostname
	Database.Hostport = appConfig.Database.Hostport
	Database.Name = appConfig.Database.Name
	Database.User = appConfig.Database.User
	Database.Password = appConfig.Database.Password
	Database.SSLMode = appConfig.Database.SSLMode

	Security.InstallLock = appConfig.Security.InstallLock
	Security.SecretKey = appConfig.Security.SecretKey
	Security.LoginRememberDays = appConfig.Security.LoginRememberDays
	Security.CookieRememberName = appConfig.Security.CookieRememberName
	Security.CookieUsername = appConfig.Security.CookieUsername
	Security.CookieSecure = appConfig.Security.CookieSecure
	Security.EnableLoginStatusCookie = appConfig.Security.EnableLoginStatusCookie
	Security.LoginStatusCookieName = appConfig.Security.LoginStatusCookieName

	Session.Provider = appConfig.Session.Provider
	Session.ProviderConfig = appConfig.Session.ProviderConfig
	Session.CookieName = appConfig.Session.CookieName
	Session.CookieSecure = appConfig.Session.CookieSecure
	Session.GCInterval = appConfig.Session.GCInterval
	Session.MaxLifeTime = appConfig.Session.MaxLifeTime
	Session.CSRFCookieName = appConfig.Session.CSRFCookieName

	return nil
}
