package entity

type ClusterSsh struct {
	ID       int64  `json:"id" gorm:"primaryKey"`                  // unique key
	Name     string `json:"name" gorm:"unique" binding:"required"` // name
	Method   string `json:"method"`                                // method
	Username string `json:"username"`                              // username
}

type ClusterSshEntityList struct {
	Sugguest []ClusterSsh `json:"sugguest"`
	List     []ClusterSsh `json:"list"`
}
