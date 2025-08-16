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

pub async fn is_exists(tables: &[String], name: &str) -> bool {
    let name_lower = name.to_lowercase();
    tables.iter().any(|s| s.to_lowercase() == name_lower)
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

    // 解析 JSON
    let install_config: InstallConfig = serde_json::from_str(install_json_str)?;

    println!("解析成功！找到 {} 个表定义", install_config.tables.len());

    let db = fastcdn_common::db::pool::Manager::new().await?;
    let tables = db.table_names().await?;

    // 遍历所有表
    for table in &install_config.tables {
        println!("\n表名: {}", table.name);
        println!("引擎: {}", table.engine);
        println!("字符集: {}", table.charset);
        println!("字段数量: {}", table.fields.len());
        println!("索引数量: {}", table.indexes.len());

        // // 打印字段信息
        // println!("字段列表:");
        // for field in &table.fields {
        //     println!("  - {}: {}", field.name, field.definition);
        // }

        // // 打印索引信息
        // println!("索引列表:");
        // for index in &table.indexes {
        //     println!("  - {}: {}", index.name, index.definition);
        // }

        let info = db.find_full_table(&table.name).await?;
        if let Some(create_statement) = info.get("create_statement") {
            println!("local_sql:{:?}", create_statement);
            println!("create_sql:{:?}", table.definition);
        } else {
            println!("create_statement not found in info");
        }

        if !is_exists(&tables, &table.name).await {
            db.create_sql(&table.definition).await?;

            let info = db.find_full_table(&table.name).await?;
            if let Some(create_statement) = info.get("create_statement") {
                println!("local_sql:{:?}", create_statement);
                println!("create_sql:{:?}", table.definition);
            } else {
                println!("create_statement not found in info");
            }
        }
    }

    Ok(())
}
