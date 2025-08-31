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
