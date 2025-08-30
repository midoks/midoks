//! 数据库查询构建器使用示例
//! 
//! 这个文件展示了如何使用新的查询构建器API来进行数据库操作，
//! 相比原来的方法，新API更加清晰和易于使用。

use crate::db::pool::Manager;
use crate::db::{QueryBuilder, InsertBuilder, UpdateBuilder, DeleteBuilder};
use serde_json::Value;

/// 示例：使用查询构建器进行复杂查询
pub async fn example_complex_query() -> Result<(), Box<dyn std::error::Error>> {
    let manager = Manager::instance().await?;
    
    // 旧的方式（不够清晰）：
    // let tokens = manager.select(
    //     "fastcdn_api_tokens",
    //     Some(&["id", "node_id", "secret", "role"]),
    //     Some("role = ? AND state = ?"),
    //     Some(&["admin", "1"])
    // ).await?;
    
    // 新的方式（更清晰）：
    let query = manager.query_builder("fastcdn_api_tokens")
        .select(&["id", "node_id", "secret", "role"])
        .where_eq("role", "admin")
        .where_eq("state", "1")
        .order_by("created_at", "DESC")
        .limit(10);
    
    let tokens = manager.query(query).await?;
    println!("查询结果: {:?}", tokens);
    
    Ok(())
}

/// 示例：使用插入构建器
pub async fn example_insert() -> Result<(), Box<dyn std::error::Error>> {
    let manager = Manager::instance().await?;
    
    // 旧的方式：
    // let mut data = HashMap::new();
    // data.insert("node_id".to_string(), Value::String("node123".to_string()));
    // data.insert("secret".to_string(), Value::String("secret123".to_string()));
    // data.insert("role".to_string(), Value::String("user".to_string()));
    // data.insert("state".to_string(), Value::Bool(true));
    // let id = manager.insert("fastcdn_api_tokens", &data).await?;
    
    // 新的方式（更清晰）：
    let insert = manager.insert_builder("fastcdn_api_tokens")
        .set_str("node_id", "node123")
        .set_str("secret", "secret123")
        .set_str("role", "user")
        .set_bool("state", true);
    
    let id = manager.insert_with_builder(insert).await?;
    println!("插入成功，ID: {}", id);
    
    Ok(())
}

/// 示例：使用更新构建器
pub async fn example_update() -> Result<(), Box<dyn std::error::Error>> {
    let manager = Manager::instance().await?;
    
    // 旧的方式：
    // let data = json!({
    //     "role": "admin",
    //     "state": true
    // });
    // let affected = manager.update(
    //     "fastcdn_api_tokens",
    //     &data,
    //     "id = ?",
    //     &["1"]
    // ).await?;
    
    // 新的方式（更清晰）：
    let update = manager.update_builder("fastcdn_api_tokens")
        .set_str("role", "admin")
        .set_bool("state", true)
        .where_id(1);
    
    let affected = manager.update_with_builder(update).await?;
    println!("更新了 {} 条记录", affected);
    
    Ok(())
}

/// 示例：使用删除构建器
pub async fn example_delete() -> Result<(), Box<dyn std::error::Error>> {
    let manager = Manager::instance().await?;
    
    // 旧的方式：
    // let affected = manager.delete(
    //     "fastcdn_api_tokens",
    //     "state = ? AND role = ?",
    //     &["0", "user"]
    // ).await?;
    
    // 新的方式（更清晰）：
    let delete = manager.delete_builder("fastcdn_api_tokens")
        .where_eq("state", "0")
        .where_eq("role", "user");
    
    let affected = manager.delete_with_builder(delete).await?;
    println!("删除了 {} 条记录", affected);
    
    Ok(())
}

/// 示例：链式查询（复杂条件）
pub async fn example_chain_query() -> Result<(), Box<dyn std::error::Error>> {
    let manager = Manager::instance().await?;
    
    // 查询活跃的管理员用户，按创建时间倒序，限制10条
    let query = manager.query_builder("fastcdn_api_tokens")
        .select(&["id", "node_id", "role", "created_at"])
        .where_eq("role", "admin")
        .where_eq("state", "1")
        .where_condition("created_at > '2024-01-01'")
        .order_by("created_at", "DESC")
        .limit(10)
        .offset(0);
    
    let results = manager.query(query).await?;
    
    for result in results {
        println!("管理员: {:?}", result);
    }
    
    Ok(())
}

/// 示例：批量操作
pub async fn example_batch_operations() -> Result<(), Box<dyn std::error::Error>> {
    let manager = Manager::instance().await?;
    
    // 批量插入多个用户
    let users = vec![
        ("user1", "secret1"),
        ("user2", "secret2"),
        ("user3", "secret3"),
    ];
    
    for (node_id, secret) in users {
        let insert = manager.insert_builder("fastcdn_api_tokens")
            .set_str("node_id", node_id)
            .set_str("secret", secret)
            .set_str("role", "user")
            .set_bool("state", true);
        
        let id = manager.insert_with_builder(insert).await?;
        println!("创建用户 {}, ID: {}", node_id, id);
    }
    
    // 批量更新用户状态
    let update = manager.update_builder("fastcdn_api_tokens")
        .set_bool("state", false)
        .where_eq("role", "user")
        .where_condition("created_at < '2024-01-01'");
    
    let affected = manager.update_with_builder(update).await?;
    println!("禁用了 {} 个旧用户", affected);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_query_builder_usage() {
        // 这里可以添加实际的测试用例
        // 注意：需要有测试数据库环境
    }
}