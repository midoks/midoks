package form

type ClusterSshCreate struct {
	ID             int64  `form:"id"`
	Name           string `form:"name"`
	Method         string `form:"method"`
	Username       string `form:"username"`
	Password       string `form:"password"`
	Privatekey     string `form:"privatekey"`
	PrivatekeyPass string `form:"privatekey_pass"`
	Mark           string `form:"mark"`
}
