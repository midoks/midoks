package form

type ApiLogs struct {
	Version   string `json:"version"`   // 请求版本信息
	Timestamp int64  `json:"timestamp"` // 时间戳

	// type:
	// sys		-> 系统信息
	// node 		-> 运行信息
	// request 	-> 应用请求
	Type string `json:"type"`

	Data string `json:"data"`
}
