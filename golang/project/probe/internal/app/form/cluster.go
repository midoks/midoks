package form

type ClusterCreate struct {
	Name string `form:"name"`
}

type ClusterSubMenu struct {
	Number int64  `form:"number"`
	Name   string `form:"name"`
	Link   string `form:"link"`
}

type ClusterGroupAdd struct {
	ID        string `form:"id"`
	Name      string `form:"name"`
	ClusterID int64  `form:"cluster_id"`
}

type ClusterNodeDelete struct {
	NodeID int64 `form:"node_id"`
}

type ClusterNodeList struct {
	Page
	ClusterID int64 `form:"cluster_id"`
}
