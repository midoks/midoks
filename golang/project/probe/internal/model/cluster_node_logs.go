package model

type ClusterNodeLogs struct {
	ID          int64  `json:"id" gorm:"primaryKey"`                    // unique key
	Day         string `json:"day" gorm:"index"`                        // day
	Description string `json:"description"`                             // description
	NodeID      int64  `json:"node_id" gorm:"index" binding:"required"` // node_id
	Level       string `json:"level"`                                   // level
	Tag         string `json:"tag" gorm:"size:255;COMMENT:'标签'"`        // tag
	IsFixed     int    `json:"is_fixed"`                                // is_fixed
	IsRead      int    `json:"is_read"`                                 // is_read
	Hash        string `json:"hash" gorm:"size:32;COMMENT:'信息内容Hash'"`  // hash
	Count       int    `json:"count" gorm:"COMMENT:'重复次数'"`             // count
	NodeTime    int64  `json:"node_time"`                               // node_time
	CreateTime  int64  `json:"create_time"`                             // create_time
}
