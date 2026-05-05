package model

type ClusterNodeIpaddr struct {
	ID             int64  `json:"id" gorm:"primaryKey"`                                 // unique key
	NodeID         int64  `json:"node_id" gorm:"uniqueIndex:idx_node_ip"`               // cluster_id
	Description    string `json:"description"`                                          // description
	Ip             string `json:"ip" gorm:"uniqueIndex:idx_node_ip" binding:"required"` // ip
	CanAccess      bool   `json:"can_access" binding:"required"`                        // can_access
	CanHealthCheck bool   `json:"can_health_check" binding:"required"`                  // can_health_check
	IsHealthy      bool   `json:"is_healthy" gorm:"index"`                              // is_healthy
	IsOn           bool   `json:"is_on" gorm:"index"`                                   // is_on
	IsUp           bool   `json:"is_up" gorm:"index"`                                   // is_up
	Order          int    `json:"order" gorm:"index" default:"0"`                       // order
	IsDeleted      int    `json:"is_deleted" gorm:"default:0"`                          // is_deleted
	CreateTime     int64  `json:"create_time"`                                          // create_time
	UpdateTime     int64  `json:"update_time"`                                          // update_time
}
