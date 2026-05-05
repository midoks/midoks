package form

type AdminAdd struct {
	ID         int64             `form:"id"`
	Username   string            `form:"username"`
	Password   string            `form:"password"`
	Password2  string            `form:"password2"`
	FullName   string            `form:"full_name"`
	AllowLogin bool              `form:"allow_login"`
	SuperAdmin bool              `form:"super_admin"`
	Status     bool              `form:"status"`
	Auth       map[string]string `form:"auth"`
}

type AdminEdit struct {
	ID       int64  `form:"id"`
	Username string `form:"username"`
	Tel      string `form:"Tel"`
	Email    string `form:"email"`
	Password string `form:"password"`
}
