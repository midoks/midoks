package model

type AdminLogs struct {
	ID         int64  `json:"id" gorm:"primaryKey"`                // unique key
	Ip         string `json:"ip" gorm:"unique" binding:"required"` // ip
	UserAgent  string `json:"user_agent"`                          // user_agent
	AdminId    int64  `json:"admin_id"`                            // admin_id
	CreateTime int64  `json:"create_time"`                         // create_time
	UpdateTime int64  `json:"update_time"`                         // update_time
}
