package logs

import (
	"bytes"
	"fmt"
	"io"
	"time"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/app/form"
	"aiprobe/internal/db"
)

func DebugInfo(c *gin.Context) {
	// 获取原始POST数据
	body, err := io.ReadAll(c.Request.Body)
	if err != nil {
		fmt.Println("Error reading body:", err)
	} else {
		fmt.Println("Raw POST data:", string(body))
		c.Request.Body = io.NopCloser(bytes.NewReader(body)) // 重置请求体，以便后续绑定
	}
}

// 上报日志
func LogsAdd(c *gin.Context) {

	api_header := c.Request.Header
	unique_id := api_header.Get("X-Node-Id")
	secret := api_header.Get("X-Secret")

	node_data, err := db.GetClusterNodeByUniqueIdAndSecret(unique_id, secret)
	if err != nil {
		common.ErrorResp(c, err, -1)
		return
	}

	fmt.Println(node_data)
	// DebugInfo(c)

	// node_model := &model.ClusterNodeLogs{
	// 	Name:       field.Name,
	// 	CreateTime: time.Now().Unix(),
	// }

	// if err := db.GetDb().Create(node_model).Error; err != nil {
	// 	common.ErrorResp(c, err, -1)
	// 	return
	// }

	var field form.ApiLogs
	if err := c.ShouldBind(&field); err != nil {
		fmt.Println("err:", err)
		common.ErrorResp(c, err, -1)
		return
	}

	now := time.Now().Unix()
	fmt.Println("field:", now, field)
	common.SuccessResp(c)
}
