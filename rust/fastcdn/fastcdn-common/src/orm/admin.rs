use crate::{db::pool, utils};

pub async fn count() -> Result<i64, Box<dyn std::error::Error>> {
    let db = pool::Manager::instance().await?;
    let results = db.count("admin", None).await?;
    Ok(results)
}

pub async fn check_admin_password(
    username: &str,
    password: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
    let db = pool::Manager::instance().await?;
    let table_name = db.get_table_name("admin");
    let query = db
        .query_builder(&table_name)
        .select(&["id", "password", "salt"])
        .limit(1)
        .where_eq("username", username);
    let results = db.query_with_builder(query).await?;
    if results.len() > 0 {
        // 先克隆一份结果，避免多次可变借用
        let result = &results[0];

        let salt_opt = result.get("salt");
        let password_opt = result.get("password");
        let id_opt = result.get("id");

        if let (Some(salt), Some(passdb), Some(id)) = (salt_opt, password_opt, id_opt) {
            if let (Some(salt_str), Some(password_str)) = (salt.as_str(), passdb.as_str()) {
                // 计算密码的MD5值
                let password_md5 = utils::common::md5_string(password);
                let password_salt = format!("{}.{}", password_md5, salt_str);
                let hash_pw = utils::common::md5_string(&password_salt);

                if hash_pw == password_str {
                    if let Some(id_u64) = id.as_u64() {
                        return Ok(id_u64);
                    }
                }
            }
        }
    }
    Ok(0u64)
}

pub async fn update_admin_password(
    id: u64,
    password: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let db = pool::Manager::instance().await?;

    let time_unix = utils::time::now_unix();
    let salt = utils::rand::string(5);
    let password_md5 = utils::common::md5_string(&password);
    let password_salt = format!("{}.{}", password_md5, salt);
    let hash_pw = utils::common::md5_string(&password_salt);

    let update = db
        .update_builder("admin")
        .set_str("password", &hash_pw)
        .set_str("salt", &salt)
        .set_str("updated_at", &time_unix)
        .where_id(id);
    let affected = db.update_with_builder(update).await?;

    Ok(affected > 0)
}

pub async fn find_admin_id_with_username(
    username: &str,
) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let db = pool::Manager::instance().await?;
    let table_name = db.get_table_name("admin");
    let query = db
        .query_builder(&table_name)
        .select(&["id"])
        .limit(1)
        .where_eq("username", username);
    let results = db.query_with_builder(query).await?;
    Ok(results)
}

pub async fn add(
    username: &str,
    password: &str,
    fullname: &str,
    is_on: bool,
    is_super: bool,
    can_login: bool,
    theme: &str,
    lang: &str,
    state: bool,
) -> Result<u64, Box<dyn std::error::Error>> {
    let db = pool::Manager::instance().await?;

    let salt = utils::rand::string(5);

    let password_md5 = utils::common::md5_string(&password);
    let password_salt = format!("{}.{}", password_md5, salt);
    let hash_pw = utils::common::md5_string(&password_salt);

    let time_unix = utils::time::now_unix();
    let mut data = std::collections::HashMap::new();
    data.insert(
        "username".to_string(),
        serde_json::Value::String(username.to_string()),
    );
    data.insert("password".to_string(), serde_json::Value::String(hash_pw));
    data.insert("salt".to_string(), serde_json::Value::String(salt));
    data.insert(
        "fullname".to_string(),
        serde_json::Value::String(fullname.to_string()),
    );
    data.insert("is_on".to_string(), serde_json::Value::Bool(is_on));
    data.insert("is_super".to_string(), serde_json::Value::Bool(is_super));
    data.insert("can_login".to_string(), serde_json::Value::Bool(can_login));
    data.insert("state".to_string(), serde_json::Value::Bool(state));
    data.insert(
        "theme".to_string(),
        serde_json::Value::String(theme.to_string()),
    );
    data.insert(
        "lang".to_string(),
        serde_json::Value::String(lang.to_string()),
    );
    data.insert(
        "created_at".to_string(),
        serde_json::Value::String(time_unix.clone()),
    );
    data.insert(
        "updated_at".to_string(),
        serde_json::Value::String(time_unix),
    );

    let id = db.insert("admin", &data).await?;
    Ok(id)
}
