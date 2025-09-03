use crate::{constant, db::pool, option, orm::api_token, utils};

pub async fn gen_unique_id() -> Result<String, Box<dyn std::error::Error>> {
    let db = pool::Manager::instance().await?;
    let table_name = db.get_table_name("api_nodes");

    loop {
        let unique_id = utils::rand::hex_string(32);
        let query = db
            .query_builder(&table_name)
            .where_eq("unique_id ", &unique_id);
        let results = db.query_with_builder(query).await?;
        if results.len() == 0 {
            return Ok(unique_id);
        }
    }
}

pub async fn find_enabled_api_node_id_with_addr(
    protocol: &str,
    host: &str,
    port: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
    let db = pool::Manager::instance().await?;

    let addr = option::network_address::NetworkAddressConfig {
        protocal: protocol.to_string(),
        host: host.to_string(),
        port_range: port.to_string(),
        min_port: 0,
        max_port: 0,
        host_has_variables: false,
    };

    let addr = serde_json::to_string(&[addr])?;
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

pub async fn create(
    is_on: Option<u8>,
    name: Option<&str>,
    description: Option<&str>,
    http: Option<&str>,
    https: Option<&str>,
    access_addrs: Option<&str>,
    unique_id: &str,
    secret: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
    let db = pool::Manager::instance().await?;
    let mut data = std::collections::HashMap::new();
    data.insert(
        "is_on".to_string(),
        serde_json::Value::String(is_on.map_or("0".to_string(), |v| v.to_string())),
    );
    data.insert(
        "name".to_string(),
        serde_json::Value::String(name.unwrap_or("").to_string()),
    );
    data.insert(
        "description".to_string(),
        serde_json::Value::String(description.unwrap_or("").to_string()),
    );
    data.insert(
        "http".to_string(),
        serde_json::Value::String(http.unwrap_or("").to_string()),
    );
    data.insert(
        "https".to_string(),
        serde_json::Value::String(https.unwrap_or("").to_string()),
    );

    data.insert(
        "access_addrs".to_string(),
        serde_json::Value::String(access_addrs.unwrap_or("").to_string()),
    );
    data.insert(
        "unique_id".to_string(),
        serde_json::Value::String(unique_id.to_string()),
    );
    data.insert(
        "secret".to_string(),
        serde_json::Value::String(secret.to_string()),
    );
    let id = db.insert("api_nodes", &data).await?;
    Ok(id)
}

pub async fn add(
    name: &str,
    description: &str,
    http_json: &str,
    https_json: &str,
    access_addrs: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
    let unique_id = gen_unique_id().await?;
    let secret = utils::rand::string(32);

    api_token::add(constant::node_rule::API, &unique_id, &secret).await?;

    let id = create(
        Some(1),
        Some(name),
        Some(description),
        Some(http_json),
        Some(https_json),
        Some(access_addrs),
        &unique_id,
        &secret,
    )
    .await?;
    println!("unique_id:{:?}", unique_id);
    println!("secret:{:?}", secret);
    Ok(id)
}
