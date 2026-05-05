package model

import (
	"encoding/json"
	"strconv"
)

type ClusterNodeLogin struct {
	ID         int64  `json:"id" gorm:"primaryKey"`        // unique key
	Name       string `json:"name"`                        // name
	NodeID     int64  `json:"node_id" gorm:"unique;index"` // node_id
	Params     string `json:"ip"`                          // params
	Status     bool   `json:"status" gorm:"index"`         // status
	CreateTime int64  `json:"create_time"`                 // create_time
	UpdateTime int64  `json:"update_time"`                 // update_time
}

type ClusterNodeLoginParams struct {
	Host  string `json:"host"`
	Port  int    `json:"port"`
	SshID int64  `json:"ssh_id"`
}

func (p *ClusterNodeLoginParams) UnmarshalJSON(b []byte) error {
	var raw map[string]json.RawMessage
	if err := json.Unmarshal(b, &raw); err != nil {
		return err
	}
	_ = json.Unmarshal(raw["host"], &p.Host)

	if v, ok := raw["port"]; ok {
		var portInt int
		if err := json.Unmarshal(v, &portInt); err == nil {
			p.Port = portInt
		} else {
			var portStr string
			if err2 := json.Unmarshal(v, &portStr); err2 == nil && portStr != "" {
				if n, err3 := strconv.Atoi(portStr); err3 == nil {
					p.Port = n
				}
			}
		}
	}

	if v, ok := raw["ssh_id"]; ok {
		var idInt int64
		if err := json.Unmarshal(v, &idInt); err == nil {
			p.SshID = idInt
		} else {
			var idStr string
			if err2 := json.Unmarshal(v, &idStr); err2 == nil && idStr != "" {
				if n, err3 := strconv.ParseInt(idStr, 10, 64); err3 == nil {
					p.SshID = n
				}
			}
		}
	}
	return nil
}

func (a *ClusterNodeLogin) SetParams(p ClusterNodeLoginParams) error {
	b, err := json.Marshal(p)
	if err != nil {
		return err
	}
	a.Params = string(b)
	return nil
}

func (a *ClusterNodeLogin) GetParams() (ClusterNodeLoginParams, error) {
	var p ClusterNodeLoginParams
	if a.Params == "" {
		return p, nil
	}
	err := json.Unmarshal([]byte(a.Params), &p)
	return p, err
}
