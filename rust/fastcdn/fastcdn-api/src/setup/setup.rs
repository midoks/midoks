use fastcdn_common::db::dump::{Dump, Find, TableInfo};
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
    for table in &install_config.tables {
        // println!("表名: {}", table.name);
        // println!("引擎: {}", table.engine);
        // println!("字符集: {}", table.charset);
        // println!("字段数量: {}", table.fields.len());
        // println!("索引数量: {}", table.indexes.len());

        // dump_sql 现在是 Vec<TableInfo>，直接使用切片
        let tables_array = dump_sql.as_slice();

        if !is_tables_exists(tables_array, &table.name).await {
            db.create_sql(&table.definition).await?;
        } else {
            for table_info in tables_array {
                // 直接访问 create_statement 字段
                let create_statement = &table_info.create_statement;
                // let create_statement_jstr = serde_json::to_string(create_statement)?;
                // println!("local_sql:{}", create_statement);
                // println!("localjstr:{}", create_statement_jstr);
                // println!("creat_sql:{:?}", table.definition);

                if table.definition != *create_statement {
                    println!("not ok!!!!");

                    // 对比字段
                    let code_fields = &table.fields;
                    for field in code_fields {
                        // println!("{:?}", field);
                        if let Some(column) = table_info.find_column(&field.name).await {
                            println!("Found column: {:?}", column);

                            let sql_cmd = format!(
                                "ALTER TABLE {} MODIFY `{}` {}",
                                table.name, field.name, field.definition
                            );

                            let _ = db.exec(&sql_cmd).await;
                            println!("sql_cmd: {}", sql_cmd);
                        } else {
                            let sql_cmd = format!(
                                "ALTER TABLE {} ADD `{}` {}",
                                table.name, field.name, field.definition
                            );

                            let _ = db.exec(&sql_cmd).await;
                        }
                    }
                    // println!("{:?}", newTable);
                }
            }
        }
    }

    Ok(())
}
