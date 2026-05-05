package form

type SubMenu struct {
	Number int64  `form:"number"`
	Name   string `form:"name"`
	Link   string `form:"link"`
	Icon   string `form:"icon"`
}

type SubSettingMenu struct {
	Number int64  `form:"number"`
	Name   string `form:"name"`
	Link   string `form:"link"`
	Type   string `form:"type"`
}

type Page struct {
	Page  int `form:"page"`
	Limit int `form:"limit"`
}

type ID struct {
	ID int64 `form:"id"`
}

type DatabaseCommon struct {
	TableName string `form:"table_name"`
}
