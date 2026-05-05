package db

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"gorm.io/driver/mysql"
	"gorm.io/driver/postgres"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"

	"aiprobe/internal/conf"
	"aiprobe/internal/model"
)

type DbInfo struct {
	Version string `json:"version"`
	Usage   string `json:"usage"`
}

func GetDbNodeInfo(node *model.DbNode) (*DbInfo, error) {
	info := &DbInfo{
		Version: "未知",
		Usage:   "未知",
	}

	// 如果没有设置数据库类型，尝试通过端口推断
	dbType := node.DbType
	if dbType == "" {
		// 根据端口推断数据库类型
		switch node.Port {
		case 3306:
			dbType = "mysql"
		case 5432:
			dbType = "postgres"
		default:
			// 默认尝试 MySQL
			dbType = "mysql"
		}
	}

	tempDb, err := getTempDbWithType(node, dbType)
	if err != nil {
		return info, err
	}
	defer func() {
		if sqlDb, _ := tempDb.DB(); sqlDb != nil {
			sqlDb.Close()
		}
	}()

	switch dbType {
	case "mysql":
		return getMysqlInfo(tempDb, info)
	case "postgres":
		return getPostgresInfo(tempDb, info)
	case "sqlite3":
		return getSqliteInfo(tempDb, info, node.Dbname)
	default:
		return info, fmt.Errorf("不支持的数据库类型: %s", dbType)
	}
}

func getTempDb(node *model.DbNode) (*gorm.DB, error) {
	return getTempDbWithType(node, node.DbType)
}

func getTempDbWithType(node *model.DbNode, dbType string) (*gorm.DB, error) {
	switch dbType {
	case "mysql":
		dsn := fmt.Sprintf("%s:%s@tcp(%s:%d)/%s?charset=utf8mb4&parseTime=True&loc=Local&tls=false",
			node.Username, node.Password, node.Host, node.Port, node.Dbname)
		return gorm.Open(mysql.Open(dsn), &gorm.Config{})

	case "postgres":
		dsn := fmt.Sprintf("host=%s user=%s password=%s dbname=%s port=%d sslmode=disable",
			node.Host, node.Username, node.Password, node.Dbname, node.Port)
		return gorm.Open(postgres.Open(dsn), &gorm.Config{})

	case "sqlite3":
		dbPath := node.Dbname
		if dbPath == "" {
			dbPath = "data/mgo.db"
		}
		if !strings.HasSuffix(dbPath, ".db") && len(dbPath) <= 3 {
			dbPath = dbPath + ".db"
		}
		if !strings.HasPrefix(dbPath, "/") {
			dbPath = filepath.Join(conf.WorkDir(), dbPath)
		}
		dbDir := filepath.Dir(dbPath)
		if !isDirExists(dbDir) {
			os.MkdirAll(dbDir, os.ModePerm)
		}
		return gorm.Open(sqlite.Open(dbPath), &gorm.Config{})

	default:
		return nil, fmt.Errorf("不支持的数据库类型: %s", dbType)
	}
}

func getMysqlInfo(tempDb *gorm.DB, info *DbInfo) (*DbInfo, error) {
	var version string
	if err := tempDb.Raw("SELECT VERSION()").Scan(&version).Error; err != nil {
		info.Version = "获取失败"
	} else {
		info.Version = version
	}

	// 使用指针类型处理可能的 NULL 值
	var usage *float64
	if err := tempDb.Raw(`SELECT ROUND(SUM(data_length + index_length) / 1024 / 1024, 2) FROM information_schema.tables WHERE table_schema = DATABASE()`).Scan(&usage).Error; err != nil {
		info.Usage = "获取失败"
	} else if usage == nil {
		info.Usage = "0MB"
	} else {
		info.Usage = fmt.Sprintf("%.2fMB", *usage)
	}

	return info, nil
}

func getPostgresInfo(tempDb *gorm.DB, info *DbInfo) (*DbInfo, error) {
	var version string
	if err := tempDb.Raw("SELECT VERSION()").Scan(&version).Error; err != nil {
		info.Version = "获取失败"
	} else {
		parts := strings.Fields(version)
		if len(parts) > 0 {
			info.Version = parts[0]
		} else {
			info.Version = "PostgreSQL"
		}
	}

	var usage float64
	if err := tempDb.Raw(`SELECT pg_database_size(current_database()) / 1024 / 1024`).Scan(&usage).Error; err != nil {
		info.Usage = "获取失败"
	} else {
		info.Usage = fmt.Sprintf("%.2fMB", usage)
	}

	return info, nil
}

func getSqliteInfo(tempDb *gorm.DB, info *DbInfo, dbPath string) (*DbInfo, error) {
	info.Version = "SQLite3"

	stat, err := os.Stat(dbPath)
	if err == nil {
		sizeMB := float64(stat.Size()) / 1024 / 1024
		if sizeMB > 1024 {
			info.Usage = fmt.Sprintf("%.2fGB", sizeMB/1024)
		} else {
			info.Usage = fmt.Sprintf("%.2fMB", sizeMB)
		}
	} else {
		var count int64
		if err := tempDb.Raw("SELECT COUNT(*) FROM sqlite_master WHERE type='table'").Scan(&count).Error; err == nil {
			info.Usage = fmt.Sprintf("%d张表", count)
		} else {
			info.Usage = "获取失败"
		}
	}

	return info, nil
}

func isDirExists(path string) bool {
	stat, err := os.Stat(path)
	return err == nil && stat.IsDir()
}
