use crate::{db::pool, utils};

pub async fn gen_unique_id() -> Result<String, Box<dyn std::error::Error>> {
    let unique_id = utils::rand::hex_string(32);
    let db = pool::Manager::instance().await?;
    let table_name = db.get_table_name("api_nodes");
    let query = db
        .query_builder(&table_name)
        .where_eq("unique_id ", &unique_id);
    let results = db.query_with_builder(query).await?;
    println!("{:?}", results);
    Ok(unique_id)
}

pub async fn find_enabled_api_node_id_with_addr(
    protocol: &str,
    host: &str,
    port: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
    let db = pool::Manager::instance().await?;

    let mut data = std::collections::HashMap::new();
    data.insert(
        "protocol".to_string(),
        serde_json::Value::String(protocol.to_string()),
    );
    data.insert(
        "host".to_string(),
        serde_json::Value::String(host.to_string()),
    );
    data.insert(
        "port_range".to_string(),
        serde_json::Value::String(port.to_string()),
    );

    let addr = serde_json::to_string(&data)?;
    let table_name = db.get_table_name("api_nodes");
    let query = db
        .query_builder(&table_name)
        .select(&["id"])
        .where_with_param("JSON_CONTAINS(access_addrs, ?)", &addr);
    let results = db.query_with_builder(query).await?;
    if let Some(first_result) = results.first() {
        if let Some(id_value) = first_result.get("id") {
            if let Some(id) = id_value.as_u64() {
                return Ok(id);
            }
        }
    }
    Ok(0u64)
}

pub async fn add(
    name: &str,
    description: &str,
    // http_json: &str,
    // https_json: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
    let unique_id = gen_unique_id().await?;
    println!("unique_id:{:?}", unique_id);
    println!("name:{:?}", name);
    println!("description:{:?}", description);
    // let db = pool::Manager::instance().await?;
    // let mut data = std::collections::HashMap::new();
    // data.insert(
    //     "role".to_string(),
    //     serde_json::Value::String(role.to_string()),
    // );
    // data.insert(
    //     "node_id".to_string(),
    //     serde_json::Value::String(node_id.to_string()),
    // );
    // data.insert(
    //     "secret".to_string(),
    //     serde_json::Value::String(secret.to_string()),
    // );

    // let id = db.insert("api_nodes", &data).await?;
    Ok(0)
}
