use crate::db::pool;

pub async fn find_enabled_api_node_id_with_addr(
    protocol: &str,
    host: &str,
    port: u16,
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
        "port".to_string(),
        serde_json::Value::Number(serde_json::Number::from(port)),
    );

    let addr = serde_json::to_string(&data)?;
    let table_name = db.get_table_name("api_nodes");
    let query = db
        .query_builder(&table_name)
        .select(&["id"])
        .where_with_param("JSON_CONTAINS(access_addrs, :addr)", &addr);
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
    role: &str,
    node_id: &str,
    secret: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
    let db = pool::Manager::instance().await?;
    let mut data = std::collections::HashMap::new();
    data.insert(
        "role".to_string(),
        serde_json::Value::String(role.to_string()),
    );
    data.insert(
        "node_id".to_string(),
        serde_json::Value::String(node_id.to_string()),
    );
    data.insert(
        "secret".to_string(),
        serde_json::Value::String(secret.to_string()),
    );

    let id = db.insert("api_nodes", &data).await?;
    Ok(id)
}
