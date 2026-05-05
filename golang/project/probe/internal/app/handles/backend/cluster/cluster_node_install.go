package cluster

import (
	"errors"
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"strconv"
	"sync"
	"time"

	"github.com/gin-gonic/gin"

	"aiprobe/internal/app/common"
	"aiprobe/internal/app/form"
	"aiprobe/internal/db"
	"aiprobe/internal/model"
	"aiprobe/internal/ssh"
)

// 安装状态结构体
type InstallStatus struct {
	NodeID    int64     `json:"node_id"`
	Status    string    `json:"status"`   // pending, running, success, failed
	Progress  int       `json:"progress"` // 0-100
	Message   string    `json:"message"`
	StartTime time.Time `json:"start_time"`
	EndTime   time.Time `json:"end_time"`
}

// 安装状态管理
var (
	installStatusMap   = make(map[int64]*InstallStatus)
	installStatusMutex sync.RWMutex
)

// 获取安装状态
func getInstallStatus(nodeID int64) *InstallStatus {
	installStatusMutex.RLock()
	defer installStatusMutex.RUnlock()
	return installStatusMap[nodeID]
}

// 设置安装状态
func setInstallStatus(nodeID int64, status string, progress int, message string) {
	installStatusMutex.Lock()
	defer installStatusMutex.Unlock()

	if _, exists := installStatusMap[nodeID]; !exists {
		installStatusMap[nodeID] = &InstallStatus{
			NodeID:    nodeID,
			StartTime: time.Now(),
		}
	}

	installStatusMap[nodeID].Status = status
	installStatusMap[nodeID].Progress = progress
	installStatusMap[nodeID].Message = message

	if status == "success" || status == "failed" {
		installStatusMap[nodeID].EndTime = time.Now()
	}
}

func NodeInstall(c *gin.Context) {
	node_id := c.Query("node_id")
	node_idint, _ := strconv.ParseInt(node_id, 10, 64)

	data := common.CommonVer(c)
	data["submenu"] = GetNodeSubMenu()
	data["node_id"] = node_id
	data["cluster_id"] = c.Query("cluster_id")

	if node_id != "" {
		node_data, _ := db.GetClusterNodeByID(node_idint)
		data["Data"] = node_data

		node_ssh_data, _ := db.GetClusterNodeLoginByNodeID(node_idint)
		data["SshData"] = node_ssh_data

	}

	c.HTML(http.StatusOK, "backend/cluster/node/install.tmpl", data)
}

func PostNodeInstallUpdateStatus(c *gin.Context) {
	var field form.ClusterNodeUpdateStatus
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}
	if field.ID > 0 {
		if err := db.ClusterNodeInstallDone(field.ID, field.IsInstalled); err != nil {
			common.ErrorResp(c, err, -1)
			return
		}
		common.SuccessResp(c)
		return
	}
	common.ErrorResp(c, errors.New("node_id error?"), -1)
}

// 开始安装 - 触发安装上传（异步）
func PostNodeInstallDone(c *gin.Context) {
	var field form.ClusterNodeDone
	if err := c.ShouldBind(&field); err != nil {
		common.ErrorResp(c, err, -1)
		return
	}
	if field.ID <= 0 {
		common.ErrorResp(c, errors.New("node_id error?"), -1)
		return
	}

	// 检查节点是否存在
	_, err := db.GetClusterNodeByID(field.ID)
	if err != nil {
		common.ErrorResp(c, errors.New("node not found"), -1)
		return
	}

	// 检查是否正在安装
	status := getInstallStatus(field.ID)
	if status != nil && status.Status == "running" {
		common.ErrorResp(c, errors.New("installation is already running"), -1)
		return
	}

	// 设置初始状态
	setInstallStatus(field.ID, "running", 0, "开始安装...")

	// 异步执行安装
	go func(nodeID int64) {
		defer func() {
			if r := recover(); r != nil {
				setInstallStatus(nodeID, "failed", 0, fmt.Sprintf("安装过程发生异常: %v", r))
			}
		}()

		// 执行安装过程
		executeInstallation(nodeID)
	}(field.ID)

	// 返回成功，告知客户端安装已开始
	common.SuccessResp(c)
}

// 执行安装过程
func executeInstallation(nodeID int64) {
	// 获取节点SSH登录信息
	node_login_data, err := db.GetClusterNodeLoginByNodeID(nodeID)
	if err != nil {
		setInstallStatus(nodeID, "failed", 0, "节点SSH登录信息未找到")
		return
	}

	// 获取SSH参数
	login_params, err := node_login_data.GetParams()
	if err != nil {
		setInstallStatus(nodeID, "failed", 0, "获取SSH参数失败")
		return
	}

	// 如果使用了SSH认证，获取SSH认证信息
	var ssh_data *model.ClusterSsh
	if login_params.SshID > 0 {
		ssh_data, err = db.GetClusterSshByID(login_params.SshID)
		if err != nil {
			setInstallStatus(nodeID, "failed", 0, "获取SSH认证信息失败")
			return
		}
	}

	// 准备SSH配置
	ssh_config := ssh.Config{
		Host:    login_params.Host,
		Port:    login_params.Port,
		Timeout: 30 * time.Second,
	}

	if ssh_data != nil {
		ssh_config.User = ssh_data.Username
		ssh_config.Password = ssh_data.Password
		ssh_config.PrivateKeyPEM = []byte(ssh_data.Privatekey)
		ssh_config.PrivateKeyPass = []byte(ssh_data.PrivatekeyPass)
	} else {
		// 如果没有使用SSH认证，使用节点登录信息中的用户名和密码
		ssh_config.User = "root"
		ssh_config.Password = ""
	}

	// 更新状态
	setInstallStatus(nodeID, "running", 10, "正在连接SSH服务器...")

	// 创建SSH客户端
	ssh_client, err := ssh.New(ssh_config)
	if err != nil {
		setInstallStatus(nodeID, "failed", 0, fmt.Sprintf("连接SSH服务器失败: %v", err))
		return
	}
	defer ssh_client.Close()

	// 检测远程服务器架构
	setInstallStatus(nodeID, "running", 15, "正在检测服务器架构...")
	stdout, stderr, err := ssh_client.Run("uname -m")
	if err != nil {
		setInstallStatus(nodeID, "failed", 0, fmt.Sprintf("检测服务器架构失败: %v, stderr: %s", err, stderr))
		return
	}
	arch := string(stdout)
	arch = arch[:len(arch)-1]

	var archSuffix string
	switch arch {
	case "x86_64":
		archSuffix = "amd64"
	case "i386", "i686":
		archSuffix = "386"
	case "aarch64", "arm64":
		archSuffix = "arm64"
	default:
		archSuffix = "amd64"
	}

	// 上传文件
	appname := fmt.Sprintf("network_probe_v1.0_linux_%s.tar.gz", archSuffix)
	local_file := filepath.Join("deploy", "network_probe", appname)
	remote_dir := "/home/root/mgo_web"
	remote_file := filepath.Join(remote_dir, appname)

	// 检查本地文件是否存在
	if _, err = os.Stat(local_file); os.IsNotExist(err) {
		setInstallStatus(nodeID, "failed", 0, fmt.Sprintf("deploy/network_probe/network_probe_v1.0_linux_%s.tar.gz 文件不存在", archSuffix))
		return
	}

	// 检查远程目录是否存在，不存在则创建
	setInstallStatus(nodeID, "running", 18, "正在检查远程目录...")
	stdout, stderr, err = ssh_client.Run(fmt.Sprintf("mkdir -p %s", remote_dir))
	if err != nil {
		setInstallStatus(nodeID, "failed", 0, fmt.Sprintf("创建远程目录失败: %v, stderr: %s", err, stderr))
		return
	}

	// 更新状态
	setInstallStatus(nodeID, "running", 20, "正在上传文件到服务器...")

	// 上传文件
	err = ssh_client.Upload(local_file, remote_file, 0755, func(written int64, total int64) {
		progress := int(float64(written)/float64(total)*60) + 20 // 20-80%
		setInstallStatus(nodeID, "running", progress, fmt.Sprintf("正在上传文件: %.2f%%", float64(written)/float64(total)*100))
	})
	if err != nil {
		setInstallStatus(nodeID, "failed", 0, fmt.Sprintf("上传文件失败: %v", err))
		return
	}

	// 更新状态
	setInstallStatus(nodeID, "running", 80, "正在解压文件...")

	// 解压文件（覆盖已有文件）
	extract_cmd := fmt.Sprintf("cd %s && tar -xzf %s --overwrite", remote_dir, remote_file)
	stdout, stderr, err = ssh_client.Run(extract_cmd)
	if err != nil {
		setInstallStatus(nodeID, "failed", 0, fmt.Sprintf("解压文件失败: %v, stderr: %s", err, stderr))
		return
	}

	// 删除压缩包文件
	delete_cmd := fmt.Sprintf("rm -rf %s", remote_file)
	stdout, stderr, err = ssh_client.Run(delete_cmd)
	if err != nil {
		setInstallStatus(nodeID, "failed", 0, fmt.Sprintf("删除压缩包文件失败: %v, stderr: %s", err, stderr))
		return
	}

	// 更新状态
	setInstallStatus(nodeID, "running", 85, "正在执行安装命令...")

	// 在远程服务器上执行安装命令
	// 假设解压后的可执行文件名为 network_probe
	executable_path := filepath.Join(remote_dir, "network_probe")
	install_cmd := fmt.Sprintf("chmod +x %s && %s install", executable_path, executable_path)
	stdout, stderr, err = ssh_client.Run(install_cmd)
	if err != nil {
		setInstallStatus(nodeID, "failed", 0, fmt.Sprintf("执行安装命令失败: %v, stderr: %s", err, stderr))
		return
	}

	// 更新状态
	setInstallStatus(nodeID, "running", 90, "正在生成配置文件...")

	// 获取节点信息
	node_data, err := db.GetClusterNodeByID(nodeID)
	if err != nil {
		setInstallStatus(nodeID, "failed", 0, "获取节点信息失败")
		return
	}

	// 生成配置文件内容
	config_content := fmt.Sprintf(`rpc.endpoints: [ "http://127.0.0.1:8001" ]
nodeId: "%s"
secret: "%s"`, node_data.UniqueID, node_data.Secret)

	// 创建临时配置文件
	config_dir := "/tmp/mgo_install"
	err = os.MkdirAll(config_dir, 0755)
	if err != nil {
		setInstallStatus(nodeID, "failed", 0, fmt.Sprintf("创建临时目录失败: %v", err))
		return
	}

	local_config_file := filepath.Join(config_dir, "api_node.yaml")
	err = os.WriteFile(local_config_file, []byte(config_content), 0644)
	if err != nil {
		setInstallStatus(nodeID, "failed", 0, fmt.Sprintf("生成配置文件失败: %v", err))
		return
	}

	// 上传配置文件到远程服务器
	remote_config_dir := filepath.Join(remote_dir, "configs")
	remote_config_file := filepath.Join(remote_config_dir, "api_node.yaml")

	// 创建远程配置目录
	setInstallStatus(nodeID, "running", 92, "正在创建远程配置目录...")
	stdout, stderr, err = ssh_client.Run(fmt.Sprintf("mkdir -p %s", remote_config_dir))
	if err != nil {
		setInstallStatus(nodeID, "failed", 0, fmt.Sprintf("创建远程配置目录失败: %v, stderr: %s", err, stderr))
		return
	}

	// 上传配置文件
	setInstallStatus(nodeID, "running", 94, "正在上传配置文件...")
	err = ssh_client.Upload(local_config_file, remote_config_file, 0644, nil)
	if err != nil {
		setInstallStatus(nodeID, "failed", 0, fmt.Sprintf("上传配置文件失败: %v", err))
		return
	}

	// 清理临时文件
	os.Remove(local_config_file)

	// 更新状态
	setInstallStatus(nodeID, "running", 96, "正在配置节点...")

	// 检测防火墙类型并开放8080端口
	stdout, stderr, err = ssh_client.Run("which firewall-cmd")
	if err == nil && len(stdout) > 0 {
		open_port_cmd := "firewall-cmd --add-port=8080/tcp --permanent && firewall-cmd --reload"
		stdout, stderr, err = ssh_client.Run(open_port_cmd)
		if err != nil {
			setInstallStatus(nodeID, "failed", 0, fmt.Sprintf("开放8080端口失败: %v, stderr: %s", err, stderr))
			return
		}
	} else {
		open_port_cmd := "iptables -A INPUT -p tcp --dport 8080 -j ACCEPT"
		stdout, stderr, err = ssh_client.Run(open_port_cmd)
		if err != nil {
			setInstallStatus(nodeID, "failed", 0, fmt.Sprintf("开放8080端口失败: %v, stderr: %s", err, stderr))
			return
		}
	}

	fmt.Printf("Install output: %s\n", stdout)

	// 安装完成
	setInstallStatus(nodeID, "success", 100, "安装成功完成")

	db.ClusterNodeInstallDone(nodeID, true)

}

// 获取安装状态
func GetNodeInstallStatus(c *gin.Context) {
	nodeIDStr := c.Query("node_id")
	nodeID, err := strconv.ParseInt(nodeIDStr, 10, 64)
	if err != nil {
		common.ErrorResp(c, errors.New("invalid node_id"), -1)
		return
	}

	status := getInstallStatus(nodeID)
	if status == nil {
		// 如果没有安装记录，返回默认状态
		status = &InstallStatus{
			NodeID:   nodeID,
			Status:   "pending",
			Progress: 0,
			Message:  "未开始安装",
		}
	}

	common.SuccessResp(c, status)
}
