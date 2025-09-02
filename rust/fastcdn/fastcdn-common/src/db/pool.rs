use crate::config::ConfigDb;
use crate::db::query_builder::{DeleteBuilder, InsertBuilder, QueryBuilder, UpdateBuilder};
use lazy_static::lazy_static;
use serde_json::Value;
use sqlx::{Column, MySqlPool, Row, TypeInfo};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref INSTANCE: Arc<Mutex<Option<Arc<Manager>>>> = Arc::new(Mutex::new(None));
}

#[derive(Debug)]
pub struct Manager {
    pub pool: Arc<MySqlPool>,
    pub table_prefix: String,
}

impl Manager {
    pub async fn instance() -> Result<Arc<Self>, Box<dyn std::error::Error>> {
        // 首先检查是否已经初始化
        {
            let instance = INSTANCE.lock().unwrap();
            if let Some(manager) = instance.as_ref() {
                return Ok(manager.clone());
            }
        } // MutexGuard 在这里被释放

        // 如果没有初始化，准备初始化数据
        let database_url = {
            let db_instance = ConfigDb::Db::instance()?;
            let config = db_instance.lock().unwrap();
            format!(
                "mysql://{}:{}@{}/{}",
                config.user, config.password, config.host, config.database
            )
        }; // MutexGuard 在这里被释放

        let pool = MySqlPool::connect(&database_url).await?;
        let manager = Arc::new(Manager {
            pool: Arc::new(pool),
            table_prefix: "fastcdn_".to_string(),
        });

        {
            let mut instance = INSTANCE.lock().unwrap();
            if instance.is_none() {
                *instance = Some(manager.clone());
            }
            Ok(manager)
        }
    }

    /// 获取数据库连接池
    pub fn get_pool(&self) -> Arc<MySqlPool> {
        self.pool.clone()
    }

    /// 获取表前缀
    pub fn get_table_prefix(&self) -> &str {
        &self.table_prefix
    }

    /// 获取带前缀的表名
    pub fn get_table_name(&self, table: &str) -> String {
        if self.table_prefix.is_empty() {
            table.to_string()
        } else {
            format!("{}{}", self.table_prefix, table)
        }
    }

    /// 创建带指定前缀的新Manager实例
    pub fn with_prefix(&self, prefix: &str) -> Manager {
        Manager {
            pool: self.pool.clone(),
            table_prefix: prefix.to_string(),
        }
    }

    /// 测试数据库连接
    pub async fn test_connection(&self) -> Result<(), Box<dyn std::error::Error>> {
        let row: (i32,) = sqlx::query_as("SELECT 1")
            .fetch_one(self.pool.as_ref())
            .await?;
        println!("✓ 数据库连接测试成功，返回值: {}", row.0);
        Ok(())
    }

    /// 数据库迁移
    pub async fn migrate(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 这里可以添加数据库迁移逻辑
        Ok(())
    }

    /// 执行 SQL 语句
    pub async fn exec(&self, sql: &str) -> Result<(), sqlx::Error> {
        sqlx::query(sql).execute(self.pool.as_ref()).await?;
        Ok(())
    }

    /// 创建表的 SQL 语句
    pub async fn create_sql(&self, sql: &str) -> Result<(), sqlx::Error> {
        sqlx::query(sql).execute(self.pool.as_ref()).await?;
        Ok(())
    }

    // ==================== CRUD 基本操作方法 ====================

    /// 插入数据 (Create)
    ///
    /// # 参数
    /// * `table` - 表名
    /// * `data` - 要插入的数据，格式为 {"column1": "value1", "column2": "value2"}
    ///
    /// # 返回
    /// 返回插入记录的 ID
    pub async fn insert(
        &self,
        table: &str,
        data: &HashMap<String, Value>,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let table_name = self.get_table_name(table);
        let columns: Vec<String> = data.keys().cloned().collect();
        let placeholders: Vec<String> = (0..columns.len()).map(|_| "?".to_string()).collect();

        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name,
            columns.join(", "),
            placeholders.join(", ")
        );

        let mut query = sqlx::query(&sql);
        for column in &columns {
            if let Some(value) = data.get(column) {
                match value {
                    Value::String(s) => query = query.bind(s),
                    Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            query = query.bind(i);
                        } else if let Some(f) = n.as_f64() {
                            query = query.bind(f);
                        }
                    }
                    Value::Bool(b) => query = query.bind(b),
                    Value::Null => query = query.bind(Option::<String>::None),
                    _ => query = query.bind(value.to_string()),
                }
            }
        }

        let result = query.execute(&*self.pool).await?;
        Ok(result.last_insert_id())
    }

    /// 查询数据 (Read)
    ///
    /// # 参数
    /// * `table` - 表名
    /// * `columns` - 要查询的列，None 表示查询所有列
    /// * `condition` - WHERE 条件，None 表示无条件
    /// * `params` - 条件参数
    ///
    /// # 返回
    /// 返回查询结果的 JSON 数组
    pub async fn select(
        &self,
        table: &str,
        columns: Option<&[&str]>,
        condition: Option<&str>,
        params: Option<&[&str]>, // 简化为字符串切片
    ) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        let table_name = self.get_table_name(table);
        let cols = columns
            .map(|c| c.join(", "))
            .unwrap_or_else(|| "*".to_string());
        let where_clause = condition
            .map(|c| format!(" WHERE {}", c))
            .unwrap_or_default();

        let sql = format!("SELECT {} FROM {}{}", cols, table_name, where_clause);
        println!("select:{:?}", sql);
        let mut query = sqlx::query(&sql);

        // 绑定参数
        if let Some(params) = params {
            for param in params {
                query = query.bind(param);
            }
        }

        let rows = query.fetch_all(&*self.pool).await?;
        let mut results = Vec::new();

        for row in rows {
            let mut obj = serde_json::Map::new();

            // 遍历所有列
            for (i, column) in row.columns().iter().enumerate() {
                let column_name = column.name();

                // 根据列类型获取值
                let value = match column.type_info().name() {
                    "VARCHAR" | "TEXT" | "CHAR" => {
                        let val: Option<String> = row.try_get(i).unwrap_or(None);
                        val.map(Value::String).unwrap_or(Value::Null)
                    }
                    "INT" | "BIGINT" | "BIGINT UNSIGNED" | "SMALLINT" | "TINYINT" => {
                        // 首先尝试获取 u64 (适用于 UNSIGNED 类型)
                        if let Ok(Some(val)) = row.try_get::<Option<u64>, _>(i) {
                            Value::Number(serde_json::Number::from(val))
                        } else if let Ok(Some(val)) = row.try_get::<Option<i64>, _>(i) {
                            Value::Number(serde_json::Number::from(val))
                        } else {
                            Value::Null
                        }
                    }
                    "FLOAT" | "DOUBLE" | "DECIMAL" => {
                        let val: Option<f64> = row.try_get(i).unwrap_or(None);
                        val.map(|v| {
                            Value::Number(
                                serde_json::Number::from_f64(v)
                                    .unwrap_or(serde_json::Number::from(0)),
                            )
                        })
                        .unwrap_or(Value::Null)
                    }
                    "BOOLEAN" | "BOOL" | "TINYINT UNSIGNED" => {
                        let val: Option<bool> = row.try_get(i).unwrap_or(None);
                        val.map(Value::Bool).unwrap_or(Value::Null)
                    }
                    _ => {
                        // 对于其他类型，尝试作为字符串获取
                        let val: Option<String> = row.try_get(i).unwrap_or(None);
                        val.map(Value::String).unwrap_or(Value::Null)
                    }
                };

                obj.insert(column_name.to_string(), value);
            }

            results.push(Value::Object(obj));
        }

        Ok(results)
    }

    /// 简化的查询方法 - 根据 ID 查询单条记录
    pub async fn find_by_id(
        &self,
        table: &str,
        _id: u64,
    ) -> Result<Option<Value>, Box<dyn std::error::Error>> {
        let results = self
            .select(
                table,
                None,
                Some("id = ?"),
                None, // 这里简化处理，实际使用时需要正确绑定参数
            )
            .await?;

        Ok(results.into_iter().next())
    }

    /// 更新数据 (Update)
    ///
    /// # 参数
    /// * `table` - 表名
    /// * `data` - 要更新的数据
    /// * `condition` - WHERE 条件
    /// * `params` - 条件参数
    ///
    /// # 返回
    /// 返回受影响的行数
    pub async fn update(
        &self,
        table: &str,
        data: &Value,
        condition: &str,
        condition_params: &[&str],
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let table_name = self.get_table_name(table);
        if let Value::Object(map) = data {
            let set_clauses: Vec<String> = map.keys().map(|k| format!("{} = ?", k)).collect();

            let sql = format!(
                "UPDATE {} SET {} WHERE {}",
                table_name,
                set_clauses.join(", "),
                condition
            );

            let mut query = sqlx::query(&sql);

            // 绑定更新数据参数
            for (_, value) in map {
                match value {
                    Value::String(s) => query = query.bind(s),
                    Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            query = query.bind(i);
                        } else if let Some(f) = n.as_f64() {
                            query = query.bind(f);
                        }
                    }
                    Value::Bool(b) => query = query.bind(b),
                    Value::Null => query = query.bind(Option::<String>::None),
                    _ => query = query.bind(value.to_string()),
                }
            }

            // 绑定条件参数
            for param in condition_params {
                query = query.bind(param);
            }

            let result = query.execute(self.pool.as_ref()).await?;
            Ok(result.rows_affected())
        } else {
            Err("数据格式错误，必须是 JSON 对象".into())
        }
    }

    /// 根据 ID 更新记录
    pub async fn update_by_id(
        &self,
        table: &str,
        id: u64,
        data: &Value,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        self.update(table, data, "id = ?", &[&id.to_string()]).await
    }

    /// 删除数据 (Delete)
    ///
    /// # 参数
    /// * `table` - 表名
    /// * `condition` - WHERE 条件
    /// * `params` - 条件参数
    ///
    /// # 返回
    /// 返回受影响的行数
    pub async fn delete(
        &self,
        table: &str,
        condition: &str,
        params: &[&str],
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let table_name = self.get_table_name(table);
        let sql = format!("DELETE FROM {} WHERE {}", table_name, condition);

        let mut query = sqlx::query(&sql);

        // 绑定参数
        for param in params {
            query = query.bind(param);
        }

        let result = query.execute(self.pool.as_ref()).await?;
        Ok(result.rows_affected())
    }

    /// 根据 ID 删除记录
    pub async fn delete_by_id(
        &self,
        table: &str,
        id: u64,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        self.delete(table, "id = ?", &[&id.to_string()]).await
    }

    /// 统计记录数
    pub async fn count(
        &self,
        table: &str,
        condition: Option<&str>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let table_name = self.get_table_name(table);
        let where_clause = condition
            .map(|c| format!(" WHERE {}", c))
            .unwrap_or_default();
        let sql = format!("SELECT COUNT(*) FROM {}{}", table_name, where_clause);

        let row: (i64,) = sqlx::query_as(&sql).fetch_one(self.pool.as_ref()).await?;

        Ok(row.0)
    }

    /// 检查记录是否存在
    pub async fn exists(
        &self,
        table: &str,
        condition: &str,
        params: &[&str],
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sql = format!("SELECT 1 FROM {} WHERE {} LIMIT 1", table, condition);

        let mut query = sqlx::query(&sql);

        // 绑定参数
        for param in params {
            query = query.bind(param);
        }

        let result = query.fetch_optional(self.pool.as_ref()).await?;
        Ok(result.is_some())
    }

    // ========== 查询构建器方法 ==========

    /// 使用查询构建器进行查询
    pub async fn query(
        &self,
        builder: QueryBuilder,
    ) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        let sql = builder.build_sql();
        let params = builder.get_params();

        let mut query = sqlx::query(&sql);
        for param in params {
            query = query.bind(param);
        }
        let rows = query.fetch_all(&*self.pool).await?;

        let mut results = Vec::new();
        for row in rows {
            let mut obj = serde_json::Map::new();
            for (i, column) in row.columns().iter().enumerate() {
                let column_name = column.name();
                let type_info = column.type_info();
                let type_name = type_info.name();

                let value = match type_name {
                    "INT" | "BIGINT" | "SMALLINT" | "TINYINT" | "BIGINT UNSIGNED" => {
                        if let Ok(val) = row.try_get::<u64, _>(i) {
                            Value::Number(serde_json::Number::from(val))
                        } else if let Ok(val) = row.try_get::<i64, _>(i) {
                            Value::Number(serde_json::Number::from(val))
                        } else {
                            Value::Null
                        }
                    }
                    "BOOLEAN" | "TINYINT UNSIGNED" => {
                        if let Ok(val) = row.try_get::<bool, _>(i) {
                            Value::Bool(val)
                        } else {
                            Value::Null
                        }
                    }
                    "FLOAT" | "DOUBLE" | "DECIMAL" => {
                        if let Ok(val) = row.try_get::<f64, _>(i) {
                            Value::Number(
                                serde_json::Number::from_f64(val)
                                    .unwrap_or(serde_json::Number::from(0)),
                            )
                        } else {
                            Value::Null
                        }
                    }
                    _ => {
                        if let Ok(val) = row.try_get::<String, _>(i) {
                            Value::String(val)
                        } else {
                            Value::Null
                        }
                    }
                };
                obj.insert(column_name.to_string(), value);
            }
            results.push(Value::Object(obj));
        }
        Ok(results)
    }

    /// 使用插入构建器进行插入
    pub async fn insert_with_builder(
        &self,
        builder: InsertBuilder,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        self.insert(builder.get_table(), builder.get_data()).await
    }

    /// 使用更新构建器进行更新
    pub async fn update_with_builder(
        &self,
        builder: UpdateBuilder,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let data_json = Value::Object(
            builder
                .get_data()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        );

        let condition = builder.get_conditions();
        let params: Vec<&str> = builder.get_params().iter().map(|s| s.as_str()).collect();

        self.update(builder.get_table(), &data_json, &condition, &params)
            .await
    }

    /// 使用删除构建器进行删除
    pub async fn delete_with_builder(
        &self,
        builder: DeleteBuilder,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let condition = builder.get_conditions();
        let params: Vec<&str> = builder.get_params().iter().map(|s| s.as_str()).collect();

        self.delete(builder.get_table(), &condition, &params).await
    }

    /// 使用查询构建器进行查询
    pub async fn query_with_builder(
        &self,
        builder: QueryBuilder,
    ) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        self.query(builder).await
    }

    /// 创建查询构建器
    pub fn query_builder(&self, table: &str) -> QueryBuilder {
        QueryBuilder::new(&table)
    }

    /// 创建插入构建器
    pub fn insert_builder(&self, table: &str) -> InsertBuilder {
        InsertBuilder::new(&table)
    }

    /// 创建更新构建器
    pub fn update_builder(&self, table: &str) -> UpdateBuilder {
        UpdateBuilder::new(&table)
    }

    /// 创建删除构建器
    pub fn delete_builder(&self, table: &str) -> DeleteBuilder {
        DeleteBuilder::new(&table)
    }
}
