use fastcdn_common::db::dump::TableInfo;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};

#[derive(RustEmbed, Debug)]
#[folder = "src/setup/db_files/"]
struct DbFiles;

#[derive(Debug, Deserialize, Serialize)]
struct InstallConfig {
    tables: Vec<Table>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Table {
    name: String,
    engine: String,
    charset: String,
    definition: String,
    fields: Vec<Field>,
    indexes: Vec<Index>,
    records: Vec<serde_json::Value>, // 使用 Value 类型处理动态记录
}

impl Table {
    pub async fn find_column(&self, name: &str) -> Option<&Field> {
        for field in &self.fields {
            if field.name == name {
                return Some(field);
            }
        }
        None
    }

    pub async fn find_index(&self, name: &str) -> Option<&Index> {
        for idx in &self.indexes {
            if idx.name == name {
                return Some(idx);
            }
        }
        None
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Field {
    name: String,
    definition: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Index {
    name: String,
    definition: String,
}

pub async fn is_exists(tables: &[String], name: &str) -> bool {
    let name_lower = name.to_lowercase();
    tables.iter().any(|s| s.to_lowercase() == name_lower)
}

// 修改函数签名，使用 TableInfo 而不是 Value
pub async fn is_tables_exists(tables: &[TableInfo], name: &str) -> bool {
    let name_lower = name.to_lowercase();
    for table_info in tables {
        if table_info.table_name.to_lowercase() == name_lower {
            return true;
        }
    }
    false
}

/// 守护进程管理器
pub struct Setup {
    log_path: String,
}

pub async fn install_db() -> Result<(), Box<dyn std::error::Error>> {
    let install_embed_file = DbFiles::get("install.json").ok_or("install.json file not found")?;
    // 将字节数据转换为字符串
    let install_json_str = std::str::from_utf8(&install_embed_file.data)?;
    let install_config: InstallConfig = serde_json::from_str(install_json_str)?;

    let db = fastcdn_common::db::pool::Manager::new().await?;
    let dump_sql = db.dump().await?;
    // println!("dump_sql:{:?}", dump_sql);

    // 遍历所有表
    for embed_table in &install_config.tables {
        let db_tables = dump_sql.as_slice();

        if !is_tables_exists(db_tables, &embed_table.name).await {
            db.create_sql(&embed_table.definition).await?;
        } else {
            for table_info in db_tables {
                let create_statement = &table_info.create_statement;
                if embed_table.definition != *create_statement {
                    // 对比字段 +
                    for embed_field in &embed_table.fields {
                        if let Some(column) = table_info.find_column(&embed_field.name).await {
                            if !column.eq_definition(&embed_field.definition).await {
                                let cmd = format!(
                                    "ALTER TABLE {} MODIFY `{}` {}",
                                    embed_table.name, embed_field.name, embed_field.definition
                                );

                                match db.exec(&cmd).await {
                                    Ok(_) => {}
                                    Err(e) => {
                                        eprintln!("modify table index command: {}", e);
                                    }
                                }
                            }
                        } else {
                            let cmd = format!(
                                "ALTER TABLE {} ADD `{}` {}",
                                embed_table.name, embed_field.name, embed_field.definition
                            );
                            match db.exec(&cmd).await {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("add table index command: {}", e);
                                }
                            }
                        }
                    }

                    // 对比索引 +
                    for embed_index in &embed_table.indexes {
                        if let Some(index) = table_info.find_index(&embed_index.name).await {
                            if index.definition().await != embed_index.definition {
                                let drop_index = format!(
                                    "ALTER TABLE {} DROP KEY {}",
                                    embed_table.name, embed_index.definition
                                );
                                match db.exec(&drop_index).await {
                                    Ok(_) => {
                                        let add_index = format!(
                                            "ALTER TABLE {} ADD {}",
                                            embed_table.name, embed_index.definition
                                        );
                                        match db.exec(&add_index).await {
                                            Ok(_) => {}
                                            Err(e) => {
                                                eprintln!("add table index command: {}", e);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("drop table index command: {}", e);
                                    }
                                }
                            }
                        } else {
                            let add_index = format!(
                                "ALTER TABLE {} ADD {}",
                                embed_table.name, embed_index.definition
                            );
                            match db.exec(&add_index).await {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("add table index command: {}", e);
                                }
                            }
                        }
                    }

                    // 字段对比 -
                    for table_col in &table_info.columns {
                        if let Some(_col) = embed_table.find_column(&table_col.name).await {
                        } else {
                            let cmd = format!(
                                "ALTER TABLE {} DROP COLUMN `{}`",
                                embed_table.name, table_col.name
                            );
                            match db.exec(&cmd).await {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("drop table column command: {}", e);
                                }
                            }
                        }
                    }

                    // 索引对比 -
                    for table_idx in &table_info.indexes {
                        if let Some(_idx) = embed_table.find_index(&table_idx.name).await {
                        } else {
                            let cmd = format!(
                                "ALTER TABLE {} DROP KEY `{}`",
                                embed_table.name, table_idx.name
                            );
                            match db.exec(&cmd).await {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("drop table index command: {}", e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
