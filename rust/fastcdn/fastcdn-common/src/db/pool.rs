use crate::web::config::ConfigDb;
use sqlx::MySqlPool;
use std::sync::Arc;

/// 数据库连接管理器
pub struct Manager {
    pool: Arc<MySqlPool>,
}

impl Manager {
    /// 创建新的数据库管理器
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db_instance = ConfigDb::Db::instance()?;
        let config = db_instance.lock().unwrap();

        // 构建数据库连接字符串
        let database_url = format!(
            "mysql://{}:{}@{}/{}",
            config.user, config.password, config.host, config.database
        );

        println!(
            "正在连接数据库: {}",
            database_url.replace(&db_config.password, "***")
        );

        // 创建连接池
        let pool = MySqlPool::connect(&database_url).await?;

        println!("✓ 数据库连接成功");

        Ok(Manager {
            pool: Arc::new(pool),
        })
    }

    /// 获取数据库连接池
    pub fn get_pool(&self) -> Arc<MySqlPool> {
        self.pool.clone()
    }

    /// 测试数据库连接
    pub async fn test_connection(&self) -> Result<(), Box<dyn std::error::Error>> {
        let row: (i32,) = sqlx::query_as("SELECT 1")
            .fetch_one(self.pool.as_ref())
            .await?;

        println!("✓ 数据库连接测试成功，返回值: {}", row.0);
        Ok(())
    }

    /// 执行数据库迁移（如果需要）
    pub async fn migrate(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 这里可以添加数据库表创建或迁移逻辑
        println!("✓ 数据库迁移完成");
        Ok(())
    }
}
