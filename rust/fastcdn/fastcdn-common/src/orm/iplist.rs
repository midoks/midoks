use crate::{db::pool, utils};

pub async fn count() -> Result<i64, Box<dyn std::error::Error>> {
    let db = pool::Manager::instance().await?;
    let results = db.count("ip_lists", None).await?;
    Ok(results)
}

pub async fn add(
    name: &str,
    stype: &str,
    code: &str,
    is_public: u8,
    is_global: u8,
) -> Result<u64, Box<dyn std::error::Error>> {
    let time_unix = utils::time::now_unix();
    let db = pool::Manager::instance().await?;
    let mut data = std::collections::HashMap::new();
    data.insert(
        "name".to_string(),
        serde_json::Value::String(name.to_string()),
    );
    data.insert(
        "type".to_string(),
        serde_json::Value::String(stype.to_string()),
    );
    data.insert(
        "code".to_string(),
        serde_json::Value::String(code.to_string()),
    );
    data.insert(
        "is_public".to_string(),
        serde_json::Value::Number(serde_json::Number::from(is_public)),
    );
    data.insert(
        "is_global".to_string(),
        serde_json::Value::Number(serde_json::Number::from(is_global)),
    );
    data.insert(
        "created_at".to_string(),
        serde_json::Value::String(time_unix),
    );

    let id = db.insert("ip_lists", &data).await?;
    Ok(id)
}
