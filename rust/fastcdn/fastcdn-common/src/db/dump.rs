use crate::db::pool;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::collections::HashMap;

/// 数据库表-字段信息数据结构体
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

/// 数据库表-索引信息数据结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TableIndexColumns {
    pub collation: String,
    pub name: String,
    pub seq_in_index: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TableIndexes {
    pub name: String,
    pub index_type: String,
    pub unique: bool,
    pub columns: Vec<TableIndexColumns>,
}

/// 数据库表-分区信息数据结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TablePartitions {
    pub name: String,
    pub method: String,
    pub expression: String,
    pub description: String,
    pub rows: i64,
    pub data_length: i64,
    pub index_length: i64,
}

/// 数据库表信息数据结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TableInfo {
    pub table_name: String,
    pub create_statement: String,
    pub columns: Vec<TableColumns>,
    pub indexes: Vec<TableIndexes>,
    pub partitions: Vec<TablePartitions>,
}

pub trait Find {
    async fn find_column(&self, name: &str) -> Option<&TableColumns>;
    async fn find_index(&self, name: &str) -> Option<&TableIndexes>;
    async fn find_partition(&self, name: &str) -> Option<&TablePartitions>;
}

impl Find for TableInfo {
    async fn find_column(&self, name: &str) -> Option<&TableColumns> {
        let columns = &self.columns;
        for col in columns {
            if col.name == name {
                return Some(col);
            }
        }
        None
    }

    async fn find_index(&self, name: &str) -> Option<&TableIndexes> {
        let indexes = &self.indexes;
        for idx in indexes {
            if idx.name == name {
                return Some(idx);
            }
        }
        None
    }

    async fn find_partition(&self, name: &str) -> Option<&TablePartitions> {
        let part = &self.partitions;
        for pt in part {
            if pt.name == name {
                return Some(pt);
            }
        }
        None
    }
}

/// 导出数据库表结构 trait
pub trait Dump {
    async fn dump(&self) -> Result<Vec<TableInfo>, Box<dyn std::error::Error>>;
    async fn table_names(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn find_full_table(
        &self,
        table_name: &str,
    ) -> Result<TableInfo, Box<dyn std::error::Error>>;
}

impl Dump for pool::Manager {
    /// 导出数据库所有表的结构信息
    ///
    /// # 返回
    /// 返回包含所有表信息的 JSON 数组，每个表包含：
    /// - table_name: 表名
    /// - engine: 表引擎
    /// - create_statement: 创建表的 SQL 语句
    /// - columns: 字段信息数组
    /// - indexes: 索引信息数组
    async fn dump(&self) -> Result<Vec<TableInfo>, Box<dyn std::error::Error>> {
        let mut tables_info = Vec::new();

        // 1. 获取所有表名
        let table_names = self.table_names().await?;

        // 2. 遍历每个表，获取完整的表信息
        for table_name in table_names {
            let table_info = self.find_full_table(&table_name).await?;
            tables_info.push(table_info);
        }

        Ok(tables_info)
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
    ) -> Result<TableInfo, Box<dyn std::error::Error>> {
        // 获取表的创建语句
        let create_query = format!("SHOW CREATE TABLE `{}`", table_name);
        let create_row = sqlx::query(&create_query)
            .fetch_one(self.pool.as_ref())
            .await?;

        let create_statement: String = create_row.try_get(1)?; // 第二列是 Create Table

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
        for row in &column_rows {
            let column_info = TableColumns {
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
            columns.push(column_info);
        }

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

        let mut indexes_map: HashMap<String, TableIndexes> = HashMap::new();
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
                    TableIndexes {
                        name: index_name.clone(),
                        unique: non_unique == 0,
                        index_type: index_type.clone(),
                        columns: Vec::new(),
                    },
                );
            }

            if let Some(index_info) = indexes_map.get_mut(&index_name) {
                let column_info = TableIndexColumns {
                    name: column_name,
                    seq_in_index,
                    collation: collation.unwrap_or_default(),
                };
                index_info.columns.push(column_info);
            }
        }

        let indexes: Vec<TableIndexes> = indexes_map.into_values().collect();

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
            let partition_info = TablePartitions {
                name: row
                    .try_get::<Option<String>, _>("PARTITION_NAME")?
                    .unwrap_or_default(),
                method: row
                    .try_get::<Option<String>, _>("PARTITION_METHOD")?
                    .unwrap_or_default(),
                expression: row
                    .try_get::<Option<String>, _>("PARTITION_EXPRESSION")?
                    .unwrap_or_default(),
                description: row
                    .try_get::<Option<String>, _>("PARTITION_DESCRIPTION")?
                    .unwrap_or_default(),
                rows: row.try_get::<Option<i64>, _>("TABLE_ROWS")?.unwrap_or(0),
                data_length: row.try_get::<Option<i64>, _>("DATA_LENGTH")?.unwrap_or(0),
                index_length: row.try_get::<Option<i64>, _>("INDEX_LENGTH")?.unwrap_or(0),
            };
            partitions.push(partition_info);
        }

        let result = TableInfo {
            table_name: table_name.to_string(),
            create_statement,
            columns,
            indexes,
            partitions,
        };

        Ok(result)
    }
}
