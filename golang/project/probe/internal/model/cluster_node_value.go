package model

type ClusterNodeValue struct {
	ID         int64  `json:"id" gorm:"primaryKey"`    // unique key
	ClusterID  int64  `json:"cluster_id" gorm:"index"` // cluster_id
	NodeID     int64  `json:"node_id" gorm:"index"`    // node_id
	Item       string `json:"item"`                    // item
	Day        int64  `json:"day"`                     // day
	Hour       int64  `json:"hour"`                    // hour
	Minute     string `json:"minute"`                  // minute
	Value      string `json:"value"`                   // value
	CreateTime int64  `json:"create_time"`             // create_time
}
