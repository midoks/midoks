use crate::db::pool;

pub async fn count() -> Result<i64, Box<dyn std::error::Error>> {
    let db = pool::Manager::instance().await?;
    let results = db.count("node_clusters", None).await?;
    Ok(results)
}

//获取默认集群ID
pub async fn get_default_id() {}

pub async fn add(
    name: &str,
    use_all_api_nodes: bool,
    state: bool,
    unique_id: &str,
    secret: &str,
    dns: &str,
    dns_name: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
    let db = pool::Manager::instance().await?;
    let mut data = std::collections::HashMap::new();
    data.insert(
        "name".to_string(),
        serde_json::Value::String(name.to_string()),
    );
    data.insert(
        "use_all_api_nodes".to_string(),
        serde_json::Value::Bool(use_all_api_nodes),
    );
    data.insert(
        "unique_id".to_string(),
        serde_json::Value::String(unique_id.to_string()),
    );
    data.insert(
        "secret".to_string(),
        serde_json::Value::String(secret.to_string()),
    );
    data.insert(
        "dns".to_string(),
        serde_json::Value::String(dns.to_string()),
    );
    data.insert(
        "dns_name".to_string(),
        serde_json::Value::String(dns_name.to_string()),
    );
    data.insert("state".to_string(), serde_json::Value::Bool(state));

    let id = db.insert("node_clusters", &data).await?;
    Ok(id)
}
