package model

type Ad struct {
	ID         int64  `json:"id" gorm:"primaryKey"`
	Title      string `json:"title" gorm:"size:255;not null"`
	Position   string `json:"position" gorm:"size:50;not null"`
	Content    string `json:"content" gorm:"type:text"`
	Link       string `json:"link" gorm:"size:500"`
	ImageURL   string `json:"image_url" gorm:"size:500"`
	Status     bool   `json:"status" gorm:"default:true"`
	Sort       int    `json:"sort" gorm:"default:0"`
	StartTime  int64  `json:"start_time"`
	EndTime    int64  `json:"end_time"`
	CreateTime int64  `json:"create_time"`
	UpdateTime int64  `json:"update_time"`
}

func (Ad) TableName() string {
	return "ad"
}
