package model

type ClusterGroup struct {
	ID         int64  `json:"id" gorm:"primaryKey"`                        // unique key
	ClusterId  int64  `json:"cluster_id" gorm:"unique" binding:"required"` // cid
	Name       string `json:"name" gorm:"unique" binding:"required"`       // name
	CreateTime int64  `json:"create_time"`                                 // create_time
	UpdateTime int64  `json:"update_time"`                                 // update_time
}
