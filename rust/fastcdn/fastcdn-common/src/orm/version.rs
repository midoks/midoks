use crate::db::pool;

pub async fn count() -> Result<i64, Box<dyn std::error::Error>> {
    let db = pool::Manager::instance().await?;
    let results = db.count("versions", None).await?;
    Ok(results)
}

// 更新记录版本信息
pub async fn update(v: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let db = pool::Manager::instance().await?;
    println!("update:1");
    let num = count().await?;
    if num > 0 {
        let update = db
            .update_builder("versions")
            .set_str("version", v)
            .where_id(1);
        let _affected = db.update_with_builder(update).await?;
        println!("_affected:{:?}", _affected);
        println!("update:1.1");
    } else {
        println!("update:1.2");
        let insert = db.insert_builder("versions").set_str("version", v);
        let _id = db.insert_with_builder(insert).await?;
        println!("_id:{:?}", _id);
    }
    println!("update:2");
    Ok(true)
}
