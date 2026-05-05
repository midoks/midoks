package model

type Cluster struct {
	ID         int64  `json:"id" gorm:"primaryKey"`                             // unique key
	Name       string `json:"name" gorm:"unique" binding:"required"`            // name
	Num        int64  `json:"num"`                                              // node num
	NumOnline  int64  `json:"num_online"`                                       // node num online
	TimeZone   string `json:"time_zone" gorm:"COMMENT:'时区'"`                    // time_zone
	UniqueID   string `json:"unique_id" gorm:"unique;index" binding:"required"` // unique_id
	CreateTime int64  `json:"create_time"`                                      // create_time
	UpdateTime int64  `json:"update_time"`                                      // update_time
}
