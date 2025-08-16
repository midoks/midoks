use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use serde_json::Value; // 添加这行导入
use fastcdn_common::db::dump::DumpSql; // 添加这行导入

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

pub async fn is_tables_exists(tables: &[Value], name: &str) -> bool {
    let name_lower = name.to_lowercase();
    for table_info in tables {
        if let Some(table_name) = table_info.get("table_name") {
            if let Some(table_name_str) = table_name.as_str() {
                if table_name_str.to_lowercase() == name_lower {
                    return true;
                }
            }
        }
    }
    false
}

/// 守护进程管理器
pub struct Setup {
    log_path: String,
}

pub async fn install_db() -> Result<(), Box<dyn std::error::Error>> {
    // 从嵌入的文件中获取 install.json
    let install_json_file = DbFiles::get("install.json").ok_or("install.json file not found")?;

    // 将字节数据转换为字符串
    let install_json_str = std::str::from_utf8(&install_json_file.data)?;
    let install_config: InstallConfig = serde_json::from_str(install_json_str)?;

    let db = fastcdn_common::db::pool::Manager::new().await?;
    let dump_sql = db.dump().await?;
    println!("dump_sql:{:?}", dump_sql);

    // 遍历所有表
    for table in &install_config.tables {
        // println!("表名: {}", table.name);
        // println!("引擎: {}", table.engine);
        // println!("字符集: {}", table.charset);
        // println!("字段数量: {}", table.fields.len());
        // println!("索引数量: {}", table.indexes.len());

        // 将 dump_sql 转换为数组切片
        let tables_array = if let Some(array) = dump_sql.as_array() {
            array.as_slice()
        } else {
            &[]
        };

        if !is_tables_exists(tables_array, &table.name).await {
            db.create_sql(&table.definition).await?;
        } else {
            for table_info in tables_array {
                if let Some(create_statement) = table_info.get("create_statement") {
                    println!("local_sql:{}", create_statement);
                    println!("creat_sql:{:?}", table.definition);

                    if let Some(create_statement_str) = create_statement.as_str() {
                        if table.definition != create_statement_str {
                            println!("not ok!!!!");
                        }
                    } else {
                        println!("create_statement is not a string");
                    }
                }
            }
        }
    }

    Ok(())
}
