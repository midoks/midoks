use crate::config::ConfigDb;
use serde_json::Value;
use sqlx::{Column, MySqlPool, Row, TypeInfo}; // 添加 TypeInfo trait
use std::collections::HashMap; // 添加 HashMap 导入
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

        // println!(
        //     "正在连接数据库: {}",
        //     database_url.replace(&config.password, "***")
        // );

        // 创建连接池
        let pool = MySqlPool::connect(&database_url).await?;

        // println!("✓ 数据库连接成功");

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
        Ok(()) // 修复：Ok() -> Ok(())
    }

    /// 数据库迁移
    pub async fn migrate(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 这里可以添加数据库迁移逻辑
        Ok(()) // 修复：Ok() -> Ok(())
    }

    /// 执行 SQL 语句
    pub async fn execute_sql(&self, sql: &str) -> Result<(), sqlx::Error> {
        sqlx::query(sql).execute(self.pool.as_ref()).await?;
        Ok(()) // 修复：Ok() -> Ok(())
    }

    /// 创建表的 SQL 语句
    pub async fn create_sql(&self, sql: &str) -> Result<(), sqlx::Error> {
        sqlx::query(sql).execute(self.pool.as_ref()).await?;
        Ok(()) // 修复：Ok() -> Ok(())
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
        let columns: Vec<String> = data.keys().cloned().collect();
        let placeholders: Vec<String> = (0..columns.len()).map(|_| "?".to_string()).collect();

        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table,
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
        let cols = columns
            .map(|c| c.join(", "))
            .unwrap_or_else(|| "*".to_string());
        let where_clause = condition
            .map(|c| format!(" WHERE {}", c))
            .unwrap_or_default();

        let sql = format!("SELECT {} FROM {}{}", cols, table, where_clause);

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
                    "INT" | "BIGINT" | "SMALLINT" | "TINYINT" => {
                        let val: Option<i64> = row.try_get(i).unwrap_or(None);
                        val.map(|v| Value::Number(serde_json::Number::from(v)))
                            .unwrap_or(Value::Null)
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
                    "BOOLEAN" | "BOOL" => {
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
        if let Value::Object(map) = data {
            let set_clauses: Vec<String> = map.keys().map(|k| format!("{} = ?", k)).collect();

            let sql = format!(
                "UPDATE {} SET {} WHERE {}",
                table,
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
        let sql = format!("DELETE FROM {} WHERE {}", table, condition);

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
        let where_clause = condition
            .map(|c| format!(" WHERE {}", c))
            .unwrap_or_default();
        let sql = format!("SELECT COUNT(*) FROM {}{}", table, where_clause);

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
    // 删除这里的多余右大括号 }

    /// 导出数据库所有表的结构信息
    ///
    /// # 返回
    /// 返回包含所有表信息的 JSON 数组，每个表包含：
    /// - table_name: 表名
    /// - engine: 表引擎
    /// - create_statement: 创建表的 SQL 语句
    /// - columns: 字段信息数组
    /// - indexes: 索引信息数组
    pub async fn dump(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let mut tables_info = Vec::new();

        // 1. 获取所有表名
        let tables_query = "SELECT TABLE_NAME, ENGINE FROM information_schema.TABLES WHERE TABLE_SCHEMA = DATABASE() AND TABLE_TYPE = 'BASE TABLE'";
        let table_rows = sqlx::query(tables_query)
            .fetch_all(self.pool.as_ref())
            .await?;

        for table_row in table_rows {
            let table_name: String = table_row.try_get("TABLE_NAME")?;
            let engine: Option<String> = table_row.try_get("ENGINE").unwrap_or(None);

            let mut table_info = serde_json::Map::new();
            table_info.insert("table_name".to_string(), Value::String(table_name.clone()));
            table_info.insert(
                "engine".to_string(),
                engine.map(Value::String).unwrap_or(Value::Null),
            );

            // 2. 获取表的创建语句
            let create_query = format!("SHOW CREATE TABLE `{}`", table_name);
            let create_row = sqlx::query(&create_query)
                .fetch_one(self.pool.as_ref())
                .await?;

            let create_statement: String = create_row.try_get(1)?; // 第二列是 Create Table
            table_info.insert(
                "create_statement".to_string(),
                Value::String(create_statement),
            );

            // 3. 获取字段信息
            let columns_query = r#"
                SELECT 
                    COLUMN_NAME,
                    DATA_TYPE,
                    IS_NULLABLE,
                    COLUMN_DEFAULT,
                    COLUMN_TYPE,
                    COLUMN_KEY,
                    EXTRA,
                    COLUMN_COMMENT
                FROM information_schema.COLUMNS 
                WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = ?
                ORDER BY ORDINAL_POSITION
            "#;

            let column_rows = sqlx::query(columns_query)
                .bind(&table_name)
                .fetch_all(self.pool.as_ref())
                .await?;

            let mut columns = Vec::new();
            for column_row in column_rows {
                let mut column_info = serde_json::Map::new();

                column_info.insert(
                    "name".to_string(),
                    Value::String(column_row.try_get::<String, _>("COLUMN_NAME")?),
                );
                column_info.insert(
                    "data_type".to_string(),
                    Value::String(column_row.try_get::<String, _>("DATA_TYPE")?),
                );
                column_info.insert(
                    "is_nullable".to_string(),
                    Value::String(column_row.try_get::<String, _>("IS_NULLABLE")?),
                );

                let default_value: Option<String> =
                    column_row.try_get("COLUMN_DEFAULT").unwrap_or(None);
                column_info.insert(
                    "default_value".to_string(),
                    default_value.map(Value::String).unwrap_or(Value::Null),
                );

                column_info.insert(
                    "column_type".to_string(),
                    Value::String(column_row.try_get::<String, _>("COLUMN_TYPE")?),
                );
                column_info.insert(
                    "column_key".to_string(),
                    Value::String(column_row.try_get::<String, _>("COLUMN_KEY")?),
                );
                column_info.insert(
                    "extra".to_string(),
                    Value::String(column_row.try_get::<String, _>("EXTRA")?),
                );

                let comment: Option<String> = column_row.try_get("COLUMN_COMMENT").unwrap_or(None);
                column_info.insert(
                    "comment".to_string(),
                    comment.map(Value::String).unwrap_or(Value::Null),
                );

                columns.push(Value::Object(column_info));
            }

            table_info.insert("columns".to_string(), Value::Array(columns));

            // 4. 获取索引信息
            let indexes_query = r#"
                SELECT 
                    INDEX_NAME,
                    COLUMN_NAME,
                    NON_UNIQUE,
                    INDEX_TYPE,
                    SEQ_IN_INDEX,
                    COLLATION,
                    CARDINALITY,
                    SUB_PART,
                    PACKED,
                    NULLABLE,
                    INDEX_COMMENT
                FROM information_schema.STATISTICS 
                WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = ?
                ORDER BY INDEX_NAME, SEQ_IN_INDEX
            "#;

            let index_rows = sqlx::query(indexes_query)
                .bind(&table_name)
                .fetch_all(self.pool.as_ref())
                .await?;

            // 按索引名分组
            let mut indexes_map: std::collections::HashMap<String, Value> = std::collections::HashMap::new();

            for index_row in index_rows {
                let index_name: String = index_row.try_get("INDEX_NAME")?;
                let column_name: String = index_row.try_get("COLUMN_NAME")?;
                let non_unique: i32 = index_row.try_get("NON_UNIQUE")?;
                let index_type: String = index_row.try_get("INDEX_TYPE")?;
                let seq_in_index: u32 = index_row.try_get("SEQ_IN_INDEX")?;
                let collation: Option<String> = index_row.try_get("COLLATION").unwrap_or(None);

                if !indexes_map.contains_key(&index_name) {
                    indexes_map.insert(
                        index_name.clone(),
                        serde_json::json!({
                            "name": index_name,
                            "unique": non_unique == 0,
                            "type": index_type,
                            "columns": []
                        }),
                    );
                }

                if let Some(index_info) = indexes_map.get_mut(&index_name) {
                    if let Some(columns_array) = index_info["columns"].as_array_mut() {
                        let column_info = serde_json::json!({
                            "name": column_name,
                            "seq_in_index": seq_in_index,
                            "collation": collation
                        });
                        columns_array.push(column_info);
                    }
                }
            }

            // 转换索引信息为最终格式
            let mut indexes = Vec::new();
            for (index_name, index_value) in indexes_map {
                let mut index_info = serde_json::Map::new();
                index_info.insert("index_name".to_string(), Value::String(index_name));

                // 从 index_value 中提取信息
                if let Some(non_unique) = index_value.get("unique") {
                    index_info.insert("non_unique".to_string(), Value::Bool(!non_unique.as_bool().unwrap_or(false)));
                }
                if let Some(index_type) = index_value.get("type") {
                    index_info.insert("index_type".to_string(), index_type.clone());
                }

                // 提取列信息
                if let Some(columns_array) = index_value.get("columns").and_then(|v| v.as_array()) {
                    let index_columns: Vec<Value> = columns_array
                        .iter()
                        .map(|col| {
                            let mut column_info = serde_json::Map::new();
                            if let Some(name) = col.get("name") {
                                column_info.insert("column_name".to_string(), name.clone());
                            }
                            if let Some(seq) = col.get("seq_in_index") {
                                column_info.insert("seq_in_index".to_string(), seq.clone());
                            }
                            if let Some(collation) = col.get("collation") {
                                column_info.insert("collation".to_string(), collation.clone());
                            }
                            Value::Object(column_info)
                        })
                        .collect();

                    index_info.insert("columns".to_string(), Value::Array(index_columns));
                }

                indexes.push(Value::Object(index_info));
            }

            table_info.insert("indexes".to_string(), Value::Array(indexes));

            tables_info.push(Value::Object(table_info));
        }

        Ok(Value::Array(tables_info))
    }

    /// 获取数据库中所有表名
    ///
    /// # 返回
    /// 返回包含所有表名的字符串向量
    pub async fn table_names(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let query = "SHOW TABLES";
        let rows = sqlx::query(query).fetch_all(self.pool.as_ref()).await?;

        let mut table_names = Vec::new();
        for row in rows {
            let table_name: String = row.try_get(0)?;
            table_names.push(table_name);
        }

        Ok(table_names)
    }

    /// 获取表的完整信息，包括表结构、分区和索引信息
    pub async fn find_full_table(
        &self,
        table_name: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let mut result = serde_json::json!({
            "table_name": table_name,
            "create_statement": "",
            "columns": [],
            "indexes": [],
            "partitions": []
        });

        // 获取表的创建语句
        let create_query = format!("SHOW CREATE TABLE `{}`", table_name);
        let create_row = sqlx::query(&create_query)
            .fetch_one(self.pool.as_ref())
            .await?;

        let create_statement: String = create_row.try_get(1)?; // 第二列是 Create Table
        result["create_statement"] = Value::String(create_statement);

        // 获取表结构信息
        let columns_query = "SELECT 
            COLUMN_NAME,
            DATA_TYPE,
            IS_NULLABLE,
            COLUMN_DEFAULT,
            COLUMN_TYPE,
            COLUMN_KEY,
            EXTRA,
            COLUMN_COMMENT
        FROM INFORMATION_SCHEMA.COLUMNS 
        WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = ?
        ORDER BY ORDINAL_POSITION";

        let column_rows = sqlx::query(columns_query)
            .bind(table_name)
            .fetch_all(self.pool.as_ref())
            .await?;

        let mut columns = Vec::new();
        for row in column_rows {
            let column_info = serde_json::json!({
                "name": row.try_get::<String, _>("COLUMN_NAME")?,
                "data_type": row.try_get::<String, _>("DATA_TYPE")?,
                "column_type": row.try_get::<String, _>("COLUMN_TYPE")?,
                "nullable": row.try_get::<String, _>("IS_NULLABLE")? == "YES",
                "default": row.try_get::<Option<String>, _>("COLUMN_DEFAULT")?,
                "key": row.try_get::<String, _>("COLUMN_KEY")?,
                "extra": row.try_get::<String, _>("EXTRA")?,
                "comment": row.try_get::<String, _>("COLUMN_COMMENT")?
            });
            columns.push(column_info);
        }
        result["columns"] = Value::Array(columns);

        // 获取索引信息
        let indexes_query = "SELECT 
            INDEX_NAME,
            COLUMN_NAME,
            NON_UNIQUE,
            INDEX_TYPE,
            SEQ_IN_INDEX,
            COLLATION
        FROM INFORMATION_SCHEMA.STATISTICS 
        WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = ?
        ORDER BY INDEX_NAME, SEQ_IN_INDEX";

        let index_rows = sqlx::query(indexes_query)
            .bind(table_name)
            .fetch_all(self.pool.as_ref())
            .await?;

        let mut indexes_map: HashMap<String, Value> = HashMap::new();
        for row in index_rows {
            let index_name: String = row.try_get("INDEX_NAME")?;
            let column_name: String = row.try_get("COLUMN_NAME")?;
            let non_unique: i32 = row.try_get("NON_UNIQUE")?;
            let index_type: String = row.try_get("INDEX_TYPE")?;
            let seq_in_index: u32 = row.try_get("SEQ_IN_INDEX")?;
            let collation: Option<String> = row.try_get("COLLATION").unwrap_or(None);

            if !indexes_map.contains_key(&index_name) {
                indexes_map.insert(
                    index_name.clone(),
                    serde_json::json!({
                        "name": index_name,
                        "unique": non_unique == 0,
                        "type": index_type,
                        "columns": []
                    }),
                );
            }

            if let Some(index_info) = indexes_map.get_mut(&index_name) {
                if let Some(columns_array) = index_info["columns"].as_array_mut() {
                    let column_info = serde_json::json!({
                        "name": column_name,
                        "seq_in_index": seq_in_index,
                        "collation": collation
                    });
                    columns_array.push(column_info);
                }
            }
        }

        let indexes: Vec<Value> = indexes_map.into_values().collect();
        result["indexes"] = Value::Array(indexes);

        // 获取分区信息
        let partitions_query = "SELECT 
            PARTITION_NAME,
            PARTITION_METHOD,
            PARTITION_EXPRESSION,
            PARTITION_DESCRIPTION,
            TABLE_ROWS,
            DATA_LENGTH,
            INDEX_LENGTH
        FROM INFORMATION_SCHEMA.PARTITIONS 
        WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = ? AND PARTITION_NAME IS NOT NULL";

        let partition_rows = sqlx::query(partitions_query)
            .bind(table_name)
            .fetch_all(self.pool.as_ref())
            .await?;

        let mut partitions = Vec::new();
        for row in partition_rows {
            let partition_info = serde_json::json!({
                "name": row.try_get::<Option<String>, _>("PARTITION_NAME")?,
                "method": row.try_get::<Option<String>, _>("PARTITION_METHOD")?,
                "expression": row.try_get::<Option<String>, _>("PARTITION_EXPRESSION")?,
                "description": row.try_get::<Option<String>, _>("PARTITION_DESCRIPTION")?,
                "rows": row.try_get::<Option<i64>, _>("TABLE_ROWS")?,
                "data_length": row.try_get::<Option<i64>, _>("DATA_LENGTH")?,
                "index_length": row.try_get::<Option<i64>, _>("INDEX_LENGTH")?
            });
            partitions.push(partition_info);
        }
        result["partitions"] = Value::Array(partitions);

        Ok(result)
    }
}
