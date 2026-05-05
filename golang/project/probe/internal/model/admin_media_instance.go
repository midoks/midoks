package model

import (
	"encoding/json"
	"fmt"
)

type AdminMediaInstance struct {
	ID         int64  `json:"id" gorm:"primaryKey"`                  // unique key
	Name       string `json:"name" gorm:"unique" binding:"required"` // name
	MediaType  string `json:"media_type"`                            // media_type
	IsOn       string `json:"is_on"`                                 // is_on
	HashLife   int64  `json:"hash_life"`                             // hash_life
	Params     string `json:"params"`                                // params
	Rate       string `json:"rate"`                                  // rate
	Mark       string `json:"mark"`                                  // mark
	Status     bool   `json:"status"`                                // status
	CreateTime int64  `json:"create_time"`                           // create_time
	UpdateTime int64  `json:"update_time"`                           // update_time
}

type AdminMediaTelegramParams struct {
	Token               string `json:"token"`
	SendID              string `json:"send_id"`
	TelegramProxyScheme string `json:"telegram_proxy_scheme"`
	TelegramProxyValue  string `json:"telegram_proxy_value"`
}

type AdminMediaWebhookParams struct {
	Url    string `json:"url"`
	Method string `json:"method"`
}

type AdminMediaEmailParams struct {
	Smtp     string `json:"smtp"`
	Username string `json:"username"`
	Password string `json:"password"`
	From     string `json:"from"`
}

type AdminMediaRateParams struct {
	Count   int64 `json:"count"`
	Minutes int64 `json:"minutes"`
}

func (a *AdminMediaInstance) SetParamsFromMap(m map[string]interface{}) error {
	b, err := json.Marshal(m)
	if err != nil {
		return err
	}
	a.Params = string(b)
	return nil
}

func (a *AdminMediaInstance) GetParamsMap() (map[string]interface{}, error) {
	if a.Params == "" {
		return map[string]interface{}{}, nil
	}
	var m map[string]interface{}
	if err := json.Unmarshal([]byte(a.Params), &m); err != nil {
		return nil, err
	}
	return m, nil
}

func (a *AdminMediaInstance) SetTelegramParams(p AdminMediaTelegramParams) error {
	b, err := json.Marshal(p)
	if err != nil {
		return err
	}
	a.Params = string(b)
	return nil
}

func (a *AdminMediaInstance) GetTelegramParams() (AdminMediaTelegramParams, error) {
	var p AdminMediaTelegramParams
	if a.Params == "" {
		return p, nil
	}
	err := json.Unmarshal([]byte(a.Params), &p)
	return p, err
}

func (a *AdminMediaInstance) SetWebhookParams(p AdminMediaWebhookParams) error {
	b, err := json.Marshal(p)
	if err != nil {
		return err
	}
	a.Params = string(b)
	return nil
}

func (a *AdminMediaInstance) GetWebhookParams() (AdminMediaWebhookParams, error) {
	var p AdminMediaWebhookParams
	if a.Params == "" {
		return p, nil
	}
	err := json.Unmarshal([]byte(a.Params), &p)
	return p, err
}

func (a *AdminMediaInstance) SetEmailParams(p AdminMediaEmailParams) error {
	b, err := json.Marshal(p)
	if err != nil {
		return err
	}
	a.Params = string(b)
	return nil
}

func (a *AdminMediaInstance) GetEmailParams() (AdminMediaEmailParams, error) {
	var p AdminMediaEmailParams
	if a.Params == "" {
		return p, nil
	}
	err := json.Unmarshal([]byte(a.Params), &p)
	return p, err
}

func (a *AdminMediaInstance) SetRate(p AdminMediaRateParams) error {
	b, err := json.Marshal(p)
	if err != nil {
		return err
	}
	a.Rate = string(b)
	return nil
}

func (a *AdminMediaInstance) GetRate() (AdminMediaRateParams, error) {
	var p AdminMediaRateParams
	if a.Rate == "" {
		return p, nil
	}
	err := json.Unmarshal([]byte(a.Rate), &p)
	return p, err
}

func (a *AdminMediaInstance) GetTelegramProxy() string {
	if a.MediaType == "telegram" {
		var p AdminMediaTelegramParams
		err := json.Unmarshal([]byte(a.Params), &p)
		if err == nil {
			if p.TelegramProxyScheme == "socket5" {
				return fmt.Sprintf("socket5://%s", p.TelegramProxyValue)
			} else if p.TelegramProxyScheme == "https" {
				return fmt.Sprintf("https://%s", p.TelegramProxyValue)
			} else if p.TelegramProxyScheme == "http" {
				return fmt.Sprintf("http://%s", p.TelegramProxyValue)
			}
		}
	}
	return ""
}
