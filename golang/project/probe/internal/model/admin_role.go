package model

type AdminRole struct {
	ID      int64 `json:"id" gorm:"primaryKey"`
	AdminID int64 `json:"admin_id" gorm:"index"`
	RoleID  int64 `json:"role_id" gorm:"index"`
}
