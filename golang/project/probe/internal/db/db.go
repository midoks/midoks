package db

import (
	"fmt"
	stdlog "log"
	"os"
	"path/filepath"
	"strings"
	"time"

	log "github.com/sirupsen/logrus"
	"gorm.io/driver/mysql"
	"gorm.io/driver/postgres"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
	"gorm.io/gorm/schema"
	_ "modernc.org/sqlite"

	"aiprobe/internal/conf"
	"aiprobe/internal/model"
	utils "aiprobe/internal/utils"
)

var db *gorm.DB

func GetDb() *gorm.DB {
	return db
}

func InitDb() {
	logLevel := logger.Silent
	newLogger := logger.New(
		stdlog.New(os.Stdout, "\r\n", stdlog.LstdFlags), // io writer
		logger.Config{
			SlowThreshold:             time.Second, // slow SQL threshold
			LogLevel:                  logLevel,    // Log level
			IgnoreRecordNotFoundError: true,
			Colorful:                  true, // disable colorful printing
		},
	)

	database := conf.Database
	prefix := database.TablePrefix
	if prefix == "" {
		prefix = "mgo_"
	}

	gormConfig := &gorm.Config{
		NamingStrategy: schema.NamingStrategy{
			TablePrefix: prefix,
		},
		Logger: newLogger,
		// performance optimization: enable prepared statement cache
		PrepareStmt: true,
		// performance optimization: disable auto ping
		DisableForeignKeyConstraintWhenMigrating: true,
	}

	var dB *gorm.DB
	var err error

	switch database.Type {
	case "sqlite3":
		{
			if !(strings.HasSuffix(database.Path, ".db") && len(database.Path) > 3) {
				log.Fatalf("db name error.")
			}

			if strings.HasPrefix(database.Path, "/") {
				dB, err = gorm.Open(sqlite.Open(fmt.Sprintf("%s?_journal=WAL&_vacuum=incremental", database.Path)), gormConfig)
			} else {
				conf_path := conf.WorkDir()
				custom_dir := fmt.Sprintf("%s/custom", conf_path)

				db_file := fmt.Sprintf("%s/%s", custom_dir, database.Path)
				db_dir := filepath.Dir(db_file)

				if !utils.IsExist(db_dir) {
					os.MkdirAll(db_dir, os.ModePerm)
				}

				dB, err = gorm.Open(sqlite.Open(fmt.Sprintf("%s?_journal=WAL&_vacuum=incremental", db_file)), gormConfig)
			}

		}
	case "mysql":
		{
			dsn := database.DSN
			if dsn == "" {
				//[username[:password]@][protocol[(address)]]/dbname[?param1=value1&...&paramN=valueN]
				dsn = fmt.Sprintf("%s:%s@tcp(%s:%d)/%s?charset=utf8mb4&parseTime=True&loc=Local&tls=%s",
					database.User, database.Password, database.Hostname, database.Hostport, database.Name, database.SSLMode)
			}
			dB, err = gorm.Open(mysql.Open(dsn), gormConfig)
		}
	case "postgres":
		{
			dsn := database.DSN
			if dsn == "" {
				dsn = fmt.Sprintf("host=%s user=%s password=%s dbname=%s port=%d sslmode=%s TimeZone=Asia/Shanghai",
					database.Hostname, database.User, database.Password, database.Name, database.Hostport, database.SSLMode)
			}
			dB, err = gorm.Open(postgres.Open(dsn), gormConfig)
		}
	default:
		log.Fatalf("not supported database type: %s", database.Type)
		return
	}

	if err != nil {
		log.Fatalf("failed to connect database:%s", err.Error())
	}

	Init(dB)
}

func Init(d *gorm.DB) {
	// assign to package-level DB
	db = d
	// performance optimization: configure connection pool using performance config
	sqlDb, err := d.DB()
	if err == nil {
		// Use performance-optimized connection pool settings
		// MaxIdleConns: number of connections retained in idle pool
		// MaxOpenConns: maximum number of open connections to the database
		// ConnMaxLifetime: maximum amount of time a connection may be reused
		defaultMaxIdleConns := 25                  // Increased from 10 for better concurrency
		defaultMaxOpenConns := 100                 // Sufficient for most applications
		defaultConnMaxLifetime := time.Hour * 2    // 2 hours to prevent connection leaks
		defaultConnMaxIdleTime := time.Minute * 10 // 10 minutes idle timeout

		sqlDb.SetMaxIdleConns(defaultMaxIdleConns)
		sqlDb.SetMaxOpenConns(defaultMaxOpenConns)
		sqlDb.SetConnMaxLifetime(defaultConnMaxLifetime)
		sqlDb.SetConnMaxIdleTime(defaultConnMaxIdleTime)

		// Test connection to ensure pool is working
		if err := sqlDb.Ping(); err != nil {
			log.Warnf("Database connection test failed: %v", err)
		}
	}

	err = AutoMigrate(
		new(model.SysSetting),
		new(model.Admin),
		new(model.AdminRecipients),
		new(model.AdminRecipientsClusterRelated),
		new(model.AdminMediaInstance),
		new(model.AdminMediaGroup),
		new(model.AdminRecipientsTasks),
		new(model.Cluster),
		new(model.ClusterGroup),
		new(model.ClusterRegion),
		new(model.ClusterNode),
		new(model.ClusterNodeLogs),
		new(model.ClusterNodeLogin),
		new(model.ClusterNodeIpaddr),
		new(model.ClusterNodeValue),
		new(model.ClusterSsh),
		new(model.Log),
		new(model.DbNode),
		new(model.User),
		new(model.Ad),
	)
	if err != nil {
		log.Fatalf("failed migrate database: %s", err.Error())
	}
}

func AutoMigrate(dst ...interface{}) error {
	var err error
	if conf.Database.Type == "mysql" {
		err = db.Set("gorm:table_options", "ENGINE=InnoDB CHARSET=utf8mb4").AutoMigrate(dst...)
	} else {
		err = db.AutoMigrate(dst...)
	}
	return err
}

func CheckDbConnnect(data map[string]string) error {
	switch strings.ToLower(data["type"]) {
	case "mysql":
		dsn := fmt.Sprintf("%s:%s@tcp(%s:%s)/%s?charset=utf8mb4&parseTime=True&loc=Local&tls=false", data["username"], data["password"], data["hostname"], data["hostport"], data["dbname"])
		_, err := gorm.Open(mysql.Open(dsn), &gorm.Config{})
		if err != nil {
			return err
		}
	case "sqlite3":
		dbPath := data["dbpath"]
		if dbPath == "" {
			dbPath = "data/mgo.db"
		}

		// 确保目录存在
		dbDir := filepath.Dir(dbPath)
		if !utils.IsExist(dbDir) {
			if err := os.MkdirAll(dbDir, os.ModePerm); err != nil {
				return err
			}
		}

		dsn := dbPath
		_, err := gorm.Open(sqlite.Open(dsn), &gorm.Config{})
		if err != nil {
			return err
		}
	case "postgres":
		dsn := fmt.Sprintf("host=%s user=%s password=%s dbname=%s port=%s sslmode=disable", data["hostname"], data["username"], data["password"], data["dbname"], data["hostport"])
		_, err := gorm.Open(postgres.Open(dsn), &gorm.Config{})
		if err != nil {
			return err
		}
	default:
		return fmt.Errorf("不支持的数据库类型: %s", data["type"])
	}
	return nil
}
