package op

import (
	"errors"
	"fmt"

	"aiprobe/internal/conf"
	"aiprobe/internal/db"
)

var (
	tablePrefix string
	tableTypes  map[string]tableTypeInfo
)

func init() {
	tablePrefix = conf.Database.TablePrefix
	if tablePrefix == "" {
		tablePrefix = "mgo_"
	}

	tableTypes = map[string]tableTypeInfo{
		tablePrefix + "logs_": {Type: "监控日志", Actions: []string{"delete", "clean"}},
		tablePrefix + "logs":  {Type: "系统日志", Actions: []string{"clean"}},
	}
}

// TableInfo 表信息结构体
type TableInfo struct {
	TableName string   `json:"table_name"`
	Size      float64  `json:"size"`    // 单位：MB
	Type      string   `json:"type"`    // 表类型：监控日志、系统日志、其他
	Actions   []string `json:"actions"` // 可执行的操作
}

// 临时结构体，用于扫描 SQL 结果
type tableInfoTemp struct {
	TableName string  `json:"table_name"`
	Size      float64 `json:"size"` // 单位：MB
}

// tableTypeInfo 表类型信息
type tableTypeInfo struct {
	Type    string
	Actions []string
}

func contains(slice []string, item string) bool {
	for _, s := range slice {
		if s == item {
			return true
		}
	}
	return false
}

func DeleteTableByName(name string) error {
	if name == "" {
		return errors.New("table name is empty")
	}

	for prefix, typeInfo := range tableTypes {
		if len(name) >= len(prefix) && name[:len(prefix)] == prefix {
			if contains(typeInfo.Actions, "delete") {
				sql := fmt.Sprintf("DROP TABLE IF EXISTS %s", name)
				if err := db.GetDb().Exec(sql).Error; err != nil {
					return err
				}
				return nil
			}
			return errors.New("no delete permission")
		}
	}

	return errors.New("table not found")
}

func CleanTableByName(name string) error {
	if name == "" {
		return errors.New("table name is empty")
	}

	for prefix, typeInfo := range tableTypes {
		if len(name) >= len(prefix) && name[:len(prefix)] == prefix {
			if contains(typeInfo.Actions, "clean") {
				var sql string
				if conf.Database.Type == "sqlite3" {
					sql = fmt.Sprintf("DELETE FROM %s", name)
				} else {
					sql = fmt.Sprintf("TRUNCATE TABLE %s", name)
				}
				if err := db.GetDb().Exec(sql).Error; err != nil {
					return err
				}
				return nil
			}
			return errors.New("no clean permission")
		}
	}
	return errors.New("table not found")
}

// GetTableList 获取所有数据库表的名称和占用空间，并根据表名匹配规则添加类型和操作
func GetTableList() ([]TableInfo, error) {
	var tempTables []tableInfoTemp

	var query string
	if conf.Database.Type == "sqlite3" {
		// SQLite 使用 sqlite_master 表
		query = `
			SELECT 
				name as table_name,
				0 as size
			FROM 
				sqlite_master 
			WHERE 
				type = 'table'
			ORDER BY 
				name
		`
	} else {
		// MySQL 使用 information_schema.tables
		query = `
			SELECT
				table_name,
				ROUND((data_length + index_length) / 1024 / 1024, 2) as size
			FROM
				information_schema.tables
			WHERE
				table_schema = DATABASE()
			ORDER BY
				size DESC
		`
	}

	if err := db.GetDb().Raw(query).Scan(&tempTables).Error; err != nil {
		return nil, fmt.Errorf("获取表信息失败: %v", err)
	}

	var tables []TableInfo
	for _, temp := range tempTables {
		tableName := temp.TableName
		tableInfo := TableInfo{
			TableName: tableName,
			Size:      temp.Size,
		}

		found := false

		for prefix, typeInfo := range tableTypes {
			if len(tableName) >= len(prefix) && tableName[:len(prefix)] == prefix {
				tableInfo.Type = typeInfo.Type
				tableInfo.Actions = typeInfo.Actions
				found = true
				break
			}
		}

		if found {
			tables = append(tables, tableInfo)
		}
	}

	return tables, nil
}
