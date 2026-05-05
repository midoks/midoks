package model

type AdminRecipients struct {
	ID          int64  `json:"id" gorm:"primaryKey"` // unique key
	AdminID     int64  `json:"admin_id"`             // admin_id
	MediaID     int64  `json:"media_id"`             // media_id
	GroupID     int64  `json:"group_id"`             // group_id
	RecipientID string `json:"recipient_id"`         // recipient_id
	Mark        string `json:"mark"`                 // mark
	Status      bool   `json:"status"`               // status
	TimeFrom    string `json:"time_from"`            // time_from
	TimeTo      string `json:"time_to"`              // time_to
	CreateTime  int64  `json:"create_time"`          // create_time
	UpdateTime  int64  `json:"update_time"`          // update_time
}
