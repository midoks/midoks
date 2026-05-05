package form

type AdminRecipientsGroup struct {
	ID     int64  `form:"id"`
	Name   string `form:"name"`
	Status bool   `form:"status"`
}
