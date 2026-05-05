package model

type AdminMediaGroup struct {
	ID         int64  `json:"id" gorm:"primaryKey"`                  // unique key
	Name       string `json:"name" gorm:"unique" binding:"required"` // name
	Status     bool   `json:"status"`                                // status
	CreateTime int64  `json:"create_time"`                           // create_time
	UpdateTime int64  `json:"update_time"`                           // update_time
}
