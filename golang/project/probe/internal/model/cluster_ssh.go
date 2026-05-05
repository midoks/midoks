package model

type ClusterSsh struct {
	ID             int64  `json:"id" gorm:"primaryKey"`                  // unique key
	Name           string `json:"name" gorm:"unique" binding:"required"` // name
	Method         string `json:"method"`                                // method
	Username       string `json:"username"`                              // username
	Password       string `json:"password"`                              // password
	Privatekey     string `json:"privatekey"`                            // private_key
	PrivatekeyPass string `json:"privatekey_pass"`                       // private_key_pass
	Mark           string `json:"mark"`                                  // mark
	CreateTime     int64  `json:"create_time"`                           // create_time
	UpdateTime     int64  `json:"update_time"`                           // update_time
}
