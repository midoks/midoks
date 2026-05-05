package form

type AdminRecipients struct {
	ID          int64   `form:"id" json:"id"`
	AdminID     int64   `form:"admin_id" json:"admin_id"`
	MediaID     int64   `form:"media_id" json:"media_id"`
	GroupID     int64   `form:"group_id" json:"group_id"`
	RecipientID string  `form:"recipient_id" json:"recipient_id"`
	ClustersID  []int64 `form:"cluster_ids" json:"cluster_ids"`
	Mark        string  `form:"mark" json:"mark"`
	Status      bool    `form:"status" json:"status"`
}
