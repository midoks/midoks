package form

type DbNodeAdd struct {
	ID       int64  `form:"id"`
	Name     string `form:"name"`
	Host     string `form:"host"`
	Port     int    `form:"port"`
	Dbname   string `form:"dbname"`
	DbType   string `form:"db_type"`
	Username string `form:"username"`
	Password string `form:"password"`
	Status   bool   `form:"status"`
}
