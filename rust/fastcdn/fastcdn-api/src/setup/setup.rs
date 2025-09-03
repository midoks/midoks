use fastcdn_common::{db::dump::TableInfo, option::network_address::NetworkAddressConfig};
use lazy_static::lazy_static;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

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

lazy_static! {
    static ref INSTANCE: Arc<Mutex<Option<Arc<Setup>>>> = Arc::new(Mutex::new(None));
}

/// 守护进程管理器
#[derive(Debug)]
pub struct Setup {}

impl Setup {
    pub async fn instance() -> Result<Arc<Self>, Box<dyn std::error::Error>> {
        {
            let instance = INSTANCE.lock().unwrap();
            if let Some(setup) = instance.as_ref() {
                return Ok(setup.clone());
            }
        }
        {
            let setup = Arc::new(Setup {});
            let mut instance = INSTANCE.lock().unwrap();
            if instance.is_none() {
                *instance = Some(setup.clone());
            }
            Ok(setup)
        }
    }

    pub async fn install(
        &self,
        protocol: &str,
        host: &str,
        port: u16,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if protocol.is_empty() {
            return Err("protocol cannot be empty.".into());
        }

        if protocol != "http" && protocol != "https" {
            return Err("protocol only supports http and https!".into());
        }

        if host.is_empty() {
            return Err("host cannot be empty.".into());
        }

        // 初始化安装创建数据库
        self.install_db().await?;
        self.check_data().await?;

        let api_token_data =
            fastcdn_common::orm::api_token::find_enabled_token_with_role("admin").await?;
        if api_token_data.is_empty() {
            return Err("can not find admin node token, please run the setup again".into());
        }

        let api_node_id = fastcdn_common::orm::api_node::find_enabled_id_with_addr(
            protocol,
            host,
            &port.to_string(),
        )
        .await?;

        println!("api_node_id:{:?}", api_node_id);
        if api_node_id == 0 {
            let addr = NetworkAddressConfig {
                protocal: protocol.to_string(),
                host: host.to_string(),
                port_range: port.to_string(),
                min_port: 0,
                max_port: 0,
                host_has_variables: false,
            };
            println!("{:?}", addr);

            let access_addrs = serde_json::to_string(&[addr])?;

            let http_json = "{}";
            if protocol == "http" {}

            let https_json = "{}";

            println!("http_json:{:?}", http_json);
            println!("https_json:{:?}", https_json);

            fastcdn_common::orm::api_node::add(
                "默认API节点",
                "这是默认创建的第一个API节点",
                http_json,
                https_json,
                &access_addrs,
            )
            .await?;
        }

        let data = fastcdn_common::orm::api_node::find_enabled_with_id(api_node_id).await?;
        let api_config = fastcdn_common::config::api::Api {
            node_id: data[0]
                .get("unique_id")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            secret: data[0]
                .get("secret")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
        };

        api_config.write()?;
        Ok(())
    }

    pub async fn check_iplist(&self) -> Result<(), Box<dyn std::error::Error>> {
        let nums = fastcdn_common::orm::iplist::count().await?;
        if nums < 2 {
            fastcdn_common::orm::iplist::add("黑名单", "black", "black", 1, 1).await?;
            fastcdn_common::orm::iplist::add("白名单", "white", "white", 1, 1).await?;
        }
        Ok(())
    }

    pub async fn check_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.check_admin_node().await?;
        self.check_user_node().await?;
        self.check_cluster().await?;
        self.check_user().await?;
        self.check_iplist().await?;
        self.check_dns().await?;
        self.check_api().await?;
        self.check_version().await?;
        Ok(())
    }

    pub async fn check_user(&self) -> Result<(), Box<dyn std::error::Error>> {
        let user_nums = fastcdn_common::orm::user::count().await?;
        if user_nums > 1 {
            return Ok(());
        }
        Ok(())
    }

    pub async fn check_admin_node(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.check_api_tokens("admin").await;
        Ok(())
    }

    pub async fn check_user_node(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.check_api_tokens("user").await;
        Ok(())
    }

    pub async fn check_dns(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.check_api_tokens("dns").await;
        Ok(())
    }

    pub async fn check_api(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.check_api_tokens("api").await;
        Ok(())
    }

    pub async fn check_version(&self) -> Result<(), Box<dyn std::error::Error>> {
        let fastcdn_version = env!("CARGO_PKG_VERSION");
        let _ = fastcdn_common::orm::version::update(fastcdn_version).await?;
        Ok(())
    }

    pub async fn check_cluster(&self) -> Result<(), Box<dyn std::error::Error>> {
        let nums = fastcdn_common::orm::node_cluster::count().await?;
        if nums < 1 {
            let option = fastcdn_common::option::cluster_dns::default();
            let option_str = serde_json::to_string(&option);
            let default_dns_name = format!("g{}.cdn", fastcdn_common::utils::rand::hex_string(6));

            let node_id = fastcdn_common::utils::rand::hex_string(32);
            let secret = fastcdn_common::utils::rand::string(32);

            fastcdn_common::orm::node_cluster::add(
                "默认集群",
                true,
                true,
                &node_id,
                &secret,
                &option_str?,
                &default_dns_name,
            )
            .await?;

            let cluster_id =
                fastcdn_common::orm::api_token::add("cluster", &node_id, &secret).await?;

            let username = format!("g{}.cdn", fastcdn_common::utils::rand::hex_string(6));
            let pass_rnd = fastcdn_common::utils::rand::hex_string(32);
            fastcdn_common::orm::user::add(&username, &pass_rnd, "默认用户", 1, 1, cluster_id)
                .await?;
        }

        Ok(())
    }

    pub async fn check_api_tokens(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let rows = fastcdn_common::orm::api_token::get_by_role(name).await?;
        if rows.len() == 0 {
            let node_id = fastcdn_common::utils::rand::hex_string(32);
            let secret = fastcdn_common::utils::rand::string(32);

            match fastcdn_common::orm::api_token::add(&name, &node_id, &secret).await {
                Ok(_id) => {
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    pub async fn install_db(&self) -> Result<(), Box<dyn std::error::Error>> {
        let install_embed_file =
            DbFiles::get("install.json").ok_or("install.json file not found")?;
        // 将字节数据转换为字符串
        let install_json_str = std::str::from_utf8(&install_embed_file.data)?;
        let install_config: InstallConfig = serde_json::from_str(install_json_str)?;

        let db = fastcdn_common::db::pool::Manager::instance().await?;
        let dump_sql = db.dump().await?;

        // 遍历所有表
        for embed_table in &install_config.tables {
            let db_tables = dump_sql.as_slice();

            if !is_tables_exists(db_tables, &embed_table.name).await {
                db.create_sql(&embed_table.definition).await?;
            } else {
                for table_info in db_tables {
                    if table_info.eq_definition(&embed_table.definition).await {
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
                        //end
                    }
                }
            }
        }

        Ok(())
    }
}
