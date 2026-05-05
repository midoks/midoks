package admin

import (
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/db"
)

func RecipientsRecipientsDetails(c *gin.Context) {
	id := c.Query("id")
	idint, _ := strconv.ParseInt(id, 10, 64)
	recipient_data, _ := db.GetAdminRecipientsByID(idint)

	data := common.CommonVer(c)
	data["submenu"] = GetRecipientsSubMenu()

	data["id"] = id
	data["Data"] = recipient_data

	// 获取关联的集群列表
	clusterRelatedList, _ := db.GetAdminRecipientsClusterRelatedByRecipientID(idint)
	var clusterList []map[string]interface{}
	for _, related := range clusterRelatedList {
		cluster, _ := db.GetClusterByID(related.ClusterID)
		if cluster.ID > 0 {
			clusterList = append(clusterList, map[string]interface{}{
				"ID":   cluster.ID,
				"Name": cluster.Name,
			})
		}
	}
	data["ClusterList"] = clusterList

	c.HTML(http.StatusOK, "backend/admin/recipients/recipients_details.tmpl", data)
}

func RecipientsRecipientsUpdate(c *gin.Context) {
	id := c.Query("id")
	idint, _ := strconv.ParseInt(id, 10, 64)
	recipient_data, _ := db.GetAdminRecipientsByID(idint)

	data := common.CommonVer(c)
	data["id"] = id
	data["Data"] = recipient_data

	data["AdminID"] = recipient_data.AdminID
	data["MediaID"] = recipient_data.MediaID
	data["GroupID"] = recipient_data.GroupID

	cluster_list, _, _ := db.GetClusterList(1, 100)
	data["ClusterList"] = cluster_list

	admin_list, _, _ := db.GetAdminList(1, 100)
	data["AdminList"] = admin_list

	recipients_list, _, _ := db.GetAdminRecipientsInstancesList(1, 100)
	data["RecipientsList"] = recipients_list

	recipients_cluster_related_list, _ := db.GetAdminRecipientsClusterRelatedByRecipientID(idint)
	data["RecipientsClusterRelated"] = recipients_cluster_related_list

	c.HTML(http.StatusOK, "backend/admin/recipients/recipients_update.tmpl", data)
}

func RecipientsRecipientsTest(c *gin.Context) {
	id := c.Query("id")
	idint, _ := strconv.ParseInt(id, 10, 64)
	recipient_data, _ := db.GetAdminRecipientsByID(idint)

	data := common.CommonVer(c)
	data["id"] = id
	data["Data"] = recipient_data
	c.HTML(http.StatusOK, "backend/admin/recipients/recipients_test.tmpl", data)
}
