use crate::{db::pool, utils};

pub async fn count() -> Result<i64, Box<dyn std::error::Error>> {
    let db = pool::Manager::instance().await?;
    let results = db.count("users", None).await?;
    Ok(results)
}

pub async fn add(
    username: &str,
    password: &str,
    fullname: &str,
    is_on: u8,
    state: u8,
    cluster_id: u64,
) -> Result<u64, Box<dyn std::error::Error>> {
    let db = pool::Manager::instance().await?;
    let time_unix = utils::time::now_unix();
    let mut data = std::collections::HashMap::new();
    data.insert(
        "username".to_string(),
        serde_json::Value::String(username.to_string()),
    );
    data.insert(
        "password".to_string(),
        serde_json::Value::String(password.to_string()),
    );
    data.insert(
        "fullname".to_string(),
        serde_json::Value::String(fullname.to_string()),
    );
    data.insert(
        "is_on".to_string(),
        serde_json::Value::Number(serde_json::Number::from(is_on)),
    );
    data.insert(
        "state".to_string(),
        serde_json::Value::Number(serde_json::Number::from(state)),
    );
    data.insert(
        "cluster_id".to_string(),
        serde_json::Value::Number(serde_json::Number::from(cluster_id)),
    );
    data.insert(
        "created_at".to_string(),
        serde_json::Value::String(time_unix),
    );

    let id = db.insert("users", &data).await?;
    Ok(id)
}
