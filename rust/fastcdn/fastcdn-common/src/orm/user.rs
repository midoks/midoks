use crate::{db::pool, utils};

pub async fn count() -> Result<i64, Box<dyn std::error::Error>> {
    let db = pool::Manager::instance().await?;
    let results = db.count("users", None).await?;
    Ok(results)
}
