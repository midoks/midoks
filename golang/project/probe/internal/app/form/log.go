package form

type LogClean struct {
	Clean string `form:"clean"`
	Day   int64  `form:"day"`
}

type LogSetting struct {
	AllowedManualDelete   bool   `form:"allowed_manual_delete"`    // allowed manual delete
	AllowedManual         bool   `form:"allowed_manual"`           // allowed manual
	SaveDay               int64  `form:"save_day"`                 // save day
	MaxCapacityLimit      int64  `form:"max_capacity_limit"`       // max capacity limit
	MaxCapacityUnit       string `form:"max_capacity_unit"`        // max capacity unit
	AllowedModClearConfig bool   `form:"allowed_mod_clear_config"` // allow mod clear config
}
