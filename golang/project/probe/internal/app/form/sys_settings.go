package form

type SettingProfile struct {
	Name string `form:"name"`
}

type SettingLogin struct {
	Name      string `form:"name"`
	Password  string `form:"password"`
	Password2 string `form:"password2"`
}

type SettingAdminUI struct {
	ProductName string `form:"product_name" binding:"required"`
	SystemName  string `form:"system_name" binding:"required"`
}

type SettingWebUI struct {
	Name     string `form:"name"`
	Subtitle string `form:"subtitle"`
}

type SettingDbConf struct {
	LogSaveDays int64 `form:"log_save_days"`
}
