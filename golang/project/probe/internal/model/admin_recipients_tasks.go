package model

type AdminRecipientsTasks struct {
	ID          int64  `json:"id" gorm:"primaryKey"` // unique key
	Day         string `json:"day"`                  // day
	IsPrimary   int64  `json:"is_primary"`           // is_primary
	RecipientID string `json:"recipient_id"`         // recipient_id
	Subject     string `json:"subject"`              // subject
	Body        string `json:"body"`                 // body
	State       int    `json:"state"`                // state
	Status      bool   `json:"status"`               // status
	CreateTime  int64  `json:"create_time"`          // create_time
	UpdateTime  int64  `json:"update_time"`          // update_time
}
