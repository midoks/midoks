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

#[warn(dead_code)]
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
                    let embed_fields = &embed_table.fields;
                    for embed_field in embed_fields {
                        if let Some(column) = table_info.find_column(&embed_field.name).await {
                            if !column.eq_definition(&embed_field.definition).await {
                                let cmd = format!(
                                    "ALTER TABLE {} MODIFY `{}` {}",
                                    embed_table.name, embed_field.name, embed_field.definition
                                );
                                let _ = db.exec(&cmd).await;
                            }
                        } else {
                            let cmd = format!(
                                "ALTER TABLE {} ADD `{}` {}",
                                embed_table.name, embed_field.name, embed_field.definition
                            );
                            let _ = db.exec(&cmd).await;
                        }
                    }

                    // 对比索引 +
                    let embed_indexes = &embed_table.indexes;
                    for embed_index in embed_indexes {
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
                }
            }
        }
    }

    Ok(())
}
