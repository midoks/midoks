use crate::db::pool;

pub async fn get_by_role(name: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let db = pool::Manager::instance().await?;
    let query = db
        .query_builder("api_tokens")
        .select(&["id", "node_id", "secret", "role"])
        .where_eq("role", name);
    let results = db.query(query).await?;
    Ok(results)
}

pub async fn find_enabled_token_with_role(
    name: &str,
) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let db = pool::Manager::instance().await?;

    let table_name = db.get_table_name("api_tokens");
    let query = db
        .query_builder(&table_name)
        .select(&["id", "node_id", "secret", "role"])
        .where_eq("state", "1")
        .where_eq("role", name);
    let results = db.query_with_builder(query).await?;
    Ok(results)
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

    data.insert(
        "state".to_string(),
        serde_json::Value::String("1".to_string()),
    );
    let id = db.insert("api_tokens", &data).await?;
    Ok(id)
    // Ok(0u64)
}

pub async fn simple_add(
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

    let id = db.insert("api_tokens", &data).await?;
    Ok(id)
}
