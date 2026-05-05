package model

type DbNode struct {
	ID         int64  `json:"id" gorm:"primaryKey"` // unique key
	Name       string `json:"name"`                 // name
	Host       string `json:"host"`                 // host
	Port       int64  `json:"port"`                 // port
	Dbname     string `json:"dbname"`               // dbname
	DbType     string `json:"db_type"`              // db_type: mysql/postgresql/sqlite3
	Username   string `json:"username"`             // username
	Password   string `json:"password"`             // password
	Order      int64  `json:"order"`                // order
	Weigth     int64  `json:"weigth"`               // weigth
	Status     bool   `json:"status"`               // status
	Mark       string `json:"mark"`                 // mark
	CreateTime int64  `json:"create_time"`          // create_time
	UpdateTime int64  `json:"update_time"`          // update_time
}
