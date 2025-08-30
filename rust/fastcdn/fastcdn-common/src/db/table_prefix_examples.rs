//! 表前缀功能使用示例
//! 
//! 本文件展示了如何使用数据库管理器的表前缀功能

use crate::db::pool::Manager;

/// 表前缀使用示例
pub async fn table_prefix_examples() -> Result<(), Box<dyn std::error::Error>> {
    // 创建数据库管理器实例
    let db = Manager::instance().await?;
    
    println!("=== 表前缀功能演示 ===");
    
    // 1. 默认情况下没有表前缀
    println!("\n1. 默认表名（无前缀）:");
    let table_name = db.get_table_name("users");
    println!("   表名: {}", table_name);
    
    // 2. 使用with_prefix创建带前缀的实例
    println!("\n2. 创建带前缀 'app_' 的实例:");
    let db_with_app_prefix = db.with_prefix("app_");
    let table_name = db_with_app_prefix.get_table_name("users");
    println!("   表名: {}", table_name);
    
    // 3. 获取表前缀
    println!("\n3. 获取表前缀:");
    let prefix = db_with_app_prefix.get_table_prefix();
    println!("   前缀: '{}'", prefix);
    
    // 4. 创建另一个带不同前缀的实例
    println!("\n4. 创建带前缀 'test_' 的实例:");
    let db_with_test_prefix = db.with_prefix("test_");
    let table_name = db_with_test_prefix.get_table_name("products");
    println!("   新实例表名: {}", table_name);
    
    // 5. 演示在查询构建器中的使用
    println!("\n5. 查询构建器中的表前缀:");
    let query_builder = db_with_test_prefix.query_builder("orders");
    println!("   查询构建器使用的表名会自动添加前缀");
    
    // 6. 演示在插入构建器中的使用
    println!("\n6. 插入构建器中的表前缀:");
    let insert_builder = db_with_test_prefix.insert_builder("customers");
    println!("   插入构建器使用的表名会自动添加前缀");
    
    // 7. 演示在更新构建器中的使用
    println!("\n7. 更新构建器中的表前缀:");
    let update_builder = db_with_test_prefix.update_builder("inventory");
    println!("   更新构建器使用的表名会自动添加前缀");
    
    // 8. 演示在删除构建器中的使用
    println!("\n8. 删除构建器中的表前缀:");
    let delete_builder = db_with_test_prefix.delete_builder("logs");
    println!("   删除构建器使用的表名会自动添加前缀");
    
    // 9. 演示传统方法中的表前缀
    println!("\n9. 传统方法中的表前缀:");
    println!("   所有传统方法(insert, select, update, delete, count)都会自动使用表前缀");
    
    // 10. 无前缀实例
    println!("\n10. 无前缀实例:");
    let db_no_prefix = db.with_prefix("");
    let table_name = db_no_prefix.get_table_name("users");
    println!("    无前缀表名: {}", table_name);
    
    println!("\n=== 表前缀功能演示完成 ===");
    
    Ok(())
}

/// 实际使用场景示例
pub async fn practical_usage_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 实际使用场景示例 ===");
    
    // 场景1: 多租户应用
    println!("\n场景1: 多租户应用");
    let db = Manager::instance().await?;
    
    // 为租户A创建专用的数据库管理器
    let tenant_a_db = db.with_prefix("tenant_a_");
    println!("租户A的用户表: {}", tenant_a_db.get_table_name("users"));
    
    // 为租户B创建专用的数据库管理器
    let tenant_b_db = db.with_prefix("tenant_b_");
    println!("租户B的用户表: {}", tenant_b_db.get_table_name("users"));
    
    // 场景2: 环境隔离
    println!("\n场景2: 环境隔离");
    let dev_db = db.with_prefix("dev_");
    let test_db = db.with_prefix("test_");
    let prod_db = db.with_prefix(""); // 生产环境不使用前缀
    
    println!("开发环境表: {}", dev_db.get_table_name("products"));
    println!("测试环境表: {}", test_db.get_table_name("products"));
    println!("生产环境表: {}", prod_db.get_table_name("products"));
    
    // 场景3: 模块化应用
    println!("\n场景3: 模块化应用");
    let user_module_db = db.with_prefix("user_");
    let order_module_db = db.with_prefix("order_");
    let inventory_module_db = db.with_prefix("inv_");
    
    println!("用户模块表: {}", user_module_db.get_table_name("profiles"));
    println!("订单模块表: {}", order_module_db.get_table_name("items"));
    println!("库存模块表: {}", inventory_module_db.get_table_name("stock"));
    
    println!("\n=== 实际使用场景示例完成 ===");
    
    Ok(())
}

/// 最佳实践建议
pub fn best_practices() {
    println!("\n=== 表前缀最佳实践 ===");
    
    println!("\n1. 前缀命名规范:");
    println!("   - 使用小写字母和下划线");
    println!("   - 以下划线结尾，如: 'app_', 'user_', 'tenant_a_'");
    println!("   - 保持简短且有意义");
    
    println!("\n2. 使用场景:");
    println!("   - 多租户应用的数据隔离");
    println!("   - 不同环境的表区分(dev_, test_, prod_)");
    println!("   - 模块化应用的表分组");
    println!("   - 数据库迁移和版本管理");
    
    println!("\n3. 性能考虑:");
    println!("   - 表前缀不会影响查询性能");
    println!("   - 索引名称也应该考虑前缀");
    println!("   - 外键约束需要考虑表前缀");
    
    println!("\n4. 注意事项:");
    println!("   - 确保所有相关表都使用相同前缀");
    println!("   - 数据库迁移脚本需要考虑前缀");
    println!("   - 备份和恢复时注意表前缀");
    
    println!("\n=== 最佳实践建议完成 ===");
}