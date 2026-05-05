package form

type ClusterCreateNode struct {
	Ip              string `form:"ip"`
	Name            string `form:"name"`
	ClusterID       int64  `form:"cluster_id"`
	IpAddressesJson string `form:"ip_addresses_json"`
}

type ClusterNodeQuery struct {
	ID int64 `form:"id"`
	Page
}

type ClusterNodeDone struct {
	ID int64 `form:"id"`
}

type ClusterNodeUpdateStatus struct {
	ID          int64 `form:"id"`
	IsInstalled bool  `form:"is_installed"`
}

type ClusterNodeIpAddr struct {
	Ip             string `form:"ip" json:"ip"`
	CanAccess      bool   `form:"can_access" json:"can_access"`
	CanHealthCheck bool   `form:"can_health_check" json:"can_health_check"`
	IsOn           bool   `form:"is_on" json:"is_on"`
	Description    string `form:"description" json:"description"`
}

type ClusterNodeSettings struct {
	ID              int64  `form:"id"`
	Name            string `form:"name"`
	IpAddressesJson string `form:"ip_addresses_json"`
}

type ClusterNodeLoginAdd struct {
	ID     int64  `form:"id"`
	Host   string `form:"host"`
	Port   int    `form:"port"`
	SshID  int64  `form:"ssh_id"`
	NodeID int64  `form:"node_id"`
}
