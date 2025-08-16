use crate::db::pool;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::Row;
use std::collections::HashMap;

/// 服务器配置结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TableColumns {
    pub name: String,
    pub data_type: String,
    pub column_type: String,
    pub nullable: bool,
    pub default: String,
    pub key: String,
    pub extra: String,
    pub comment: String,
}

/// 数据库导出 trait
pub trait DumpSql {
    async fn dump(&self) -> Result<JsonValue, Box<dyn std::error::Error>>;
    async fn table_names(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn find_full_table(
        &self,
        table_name: &str,
    ) -> Result<JsonValue, Box<dyn std::error::Error>>;
}

impl DumpSql for pool::Manager {
    /// 导出数据库所有表的结构信息
    ///
    /// # 返回
    /// 返回包含所有表信息的 JSON 数组，每个表包含：
    /// - table_name: 表名
    /// - engine: 表引擎
    /// - create_statement: 创建表的 SQL 语句
    /// - columns: 字段信息数组
    /// - indexes: 索引信息数组
    async fn dump(&self) -> Result<JsonValue, Box<dyn std::error::Error>> {
        let mut tables_info = Vec::new();

        // 1. 获取所有表名
        let table_names = self.table_names().await?;

        // 2. 遍历每个表，获取完整的表信息
        for table_name in table_names {
            let table_info = self.find_full_table(&table_name).await?;
            tables_info.push(table_info);
        }

        Ok(JsonValue::Array(tables_info))
    }

    /// 获取数据库中所有表名
    ///
    /// # 返回
    /// 返回包含所有表名的字符串向量
    async fn table_names(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
    async fn find_full_table(
        &self,
        table_name: &str,
    ) -> Result<JsonValue, Box<dyn std::error::Error>> {
        let mut result = serde_json::json!({
            "table_name": table_name,
            "create_statement": "",
            "columns": [],
            "fields": [],
            "indexes": [],
            "partitions": []
        });

        // 获取表的创建语句
        let create_query = format!("SHOW CREATE TABLE `{}`", table_name);
        let create_row = sqlx::query(&create_query)
            .fetch_one(self.pool.as_ref())
            .await?;

        let create_statement: String = create_row.try_get(1)?; // 第二列是 Create Table
        result["create_statement"] = JsonValue::String(create_statement);

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
        let mut f = Vec::new(); // 添加这行来定义变量 f
        for row in &column_rows {
            let column_infos = TableColumns {
                name: row.try_get::<String, _>("COLUMN_NAME")?,
                data_type: row.try_get::<String, _>("DATA_TYPE")?,
                column_type: row.try_get::<String, _>("COLUMN_TYPE")?,
                nullable: row.try_get::<String, _>("IS_NULLABLE")? == "YES",
                default: row
                    .try_get::<Option<String>, _>("COLUMN_DEFAULT")?
                    .unwrap_or_default(),
                key: row.try_get::<String, _>("COLUMN_KEY")?,
                extra: row.try_get::<String, _>("EXTRA")?,
                comment: row.try_get::<String, _>("COLUMN_COMMENT")?,
            };
            columns.push(serde_json::to_value(column_infos.clone())?); // 转换为 JSON 值
            f.push(serde_json::to_value(column_infos)?); // 修复：转换为 JSON 值
        }
        result["columns"] = JsonValue::Array(columns);

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

        let mut indexes_map: HashMap<String, JsonValue> = HashMap::new();
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

        let indexes: Vec<JsonValue> = indexes_map.into_values().collect();
        result["indexes"] = JsonValue::Array(indexes);

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
        result["partitions"] = JsonValue::Array(partitions);

        Ok(result)
    }
}
