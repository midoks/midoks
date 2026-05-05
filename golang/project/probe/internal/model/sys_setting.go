package model

import (
	"encoding/json"
)

type SysSetting struct {
	ID         int64  `json:"id" gorm:"primaryKey"` // unique key
	Code       string `json:"code" gorm:"unique"`   // code
	Uid        int64  `json:"uid"`                  // uid
	Value      string `json:"value"`                // value
	UpdateTime int64  `json:"update_time"`          // update_time
	CreateTime int64  `json:"create_time"`          // create_time
}

type SysSettingLogValue struct {
	AllowedManualDelete   bool   `json:"allowed_manual_delete"`    // allowed manual delete
	AllowedManual         bool   `json:"allowed_manual"`           // allowed manual
	SaveDay               int64  `json:"save_day"`                 // save day
	MaxCapacityLimit      int64  `json:"max_capacity_limit"`       // max capacity limit
	MaxCapacityUnit       string `json:"max_capacity_unit"`        // max capacity unit
	AllowedModClearConfig bool   `json:"allowed_mod_clear_config"` // allowed mod clear config
}

type SysSettingAdminUIValue struct {
	DomainName  string `json:"domain_name"`  // domain_name
	ProductName string `json:"product_name"` // product_name
	SystemName  string `json:"system_name"`  // system_name
}

type SysSettingWebUIValue struct {
	Name     string `json:"name"`     // name
	Subtitle string `json:"subtitle"` // subtitle
}

type SysSettingDbConfValue struct {
	LogSaveDays int64 `json:"log_save_days"` // log_save_days
}

func (a *SysSetting) SetLogValue(p SysSettingLogValue) error {
	b, err := json.Marshal(p)
	if err != nil {
		return err
	}
	a.Value = string(b)
	return nil
}

func (a *SysSetting) GetLogValue() (SysSettingLogValue, error) {
	var p SysSettingLogValue
	if a.Value == "" {
		return p, nil
	}

	err := json.Unmarshal([]byte(a.Value), &p)
	return p, err
}

func (a *SysSetting) GetAdminUIValue() (SysSettingAdminUIValue, error) {
	var p SysSettingAdminUIValue
	if a.Value == "" {
		return p, nil
	}

	err := json.Unmarshal([]byte(a.Value), &p)
	return p, err
}

func (a *SysSetting) SetAdminUIValue(p SysSettingAdminUIValue) error {
	b, err := json.Marshal(p)
	if err != nil {
		return err
	}
	a.Value = string(b)
	return nil
}

func (a *SysSetting) GetWebUIValue() (SysSettingWebUIValue, error) {
	var p SysSettingWebUIValue
	if a.Value == "" {
		return p, nil
	}

	err := json.Unmarshal([]byte(a.Value), &p)
	return p, err
}

func (a *SysSetting) SetWebUIValue(p SysSettingWebUIValue) error {
	b, err := json.Marshal(p)
	if err != nil {
		return err
	}
	a.Value = string(b)
	return nil
}

func (a *SysSetting) GetDbConfValue() (SysSettingDbConfValue, error) {
	var p SysSettingDbConfValue
	if a.Value == "" {
		return p, nil
	}

	err := json.Unmarshal([]byte(a.Value), &p)
	return p, err
}

func (a *SysSetting) SetDbConfValue(p SysSettingDbConfValue) error {
	b, err := json.Marshal(p)
	if err != nil {
		return err
	}
	a.Value = string(b)
	return nil
}
