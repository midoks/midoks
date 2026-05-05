package form

type AdForm struct {
	Title     string `form:"title" binding:"required"`
	Position  string `form:"position" binding:"required"`
	Content   string `form:"content"`
	Link      string `form:"link"`
	ImageURL  string `form:"image_url"`
	Status    bool   `form:"status"`
	Sort      int    `form:"sort"`
	StartTime int64  `form:"start_time"`
	EndTime   int64  `form:"end_time"`
}

type AdListForm struct {
	Page     int    `form:"page"`
	Size     int    `form:"size"`
	Position string `form:"position"`
}
