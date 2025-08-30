# 数据库查询构建器 API

## 概述

为了提高数据库操作的可读性和易用性，我们引入了新的查询构建器模式。相比原来的方法，新的API更加清晰、类型安全，并且支持链式调用。

## 主要改进

### 1. 原来的问题

```rust
// 原来的方式 - 参数不够清晰
let tokens = manager.select(
    "fastcdn_api_tokens",
    Some(&["id,node_id,secret,role"]),  // 容易出错：应该是字符串数组而不是单个字符串
    Some("role='admin'"),
    Some(&[]),
).await?;
```

### 2. 新的解决方案

```rust
// 新的方式 - 更清晰易读
let query = manager.query_builder("fastcdn_api_tokens")
    .select(&["id", "node_id", "secret", "role"])  // 明确的字符串数组
    .where_eq("role", "admin");

let tokens = manager.query(query).await?;
```

## API 使用指南

### 查询操作 (SELECT)

```rust
use fastcdn_common::db::Manager;

// 基本查询
let query = manager.query_builder("users")
    .select(&["id", "name", "email"])
    .where_eq("status", "active");

let results = manager.query(query).await?;

// 复杂查询
let query = manager.query_builder("fastcdn_api_tokens")
    .select(&["id", "node_id", "role", "created_at"])
    .where_eq("role", "admin")
    .where_eq("state", "1")
    .where_condition("created_at > '2024-01-01'")
    .order_by("created_at", "DESC")
    .limit(10)
    .offset(0);

let results = manager.query(query).await?;
```

### 插入操作 (INSERT)

```rust
// 使用插入构建器
let insert = manager.insert_builder("fastcdn_api_tokens")
    .set_str("node_id", "node123")
    .set_str("secret", "secret123")
    .set_str("role", "user")
    .set_bool("state", true);

let id = manager.insert_with_builder(insert).await?;
```

### 更新操作 (UPDATE)

```rust
// 使用更新构建器
let update = manager.update_builder("fastcdn_api_tokens")
    .set_str("role", "admin")
    .set_bool("state", true)
    .where_id(1);

let affected = manager.update_with_builder(update).await?;

// 复杂更新条件
let update = manager.update_builder("users")
    .set_str("status", "inactive")
    .where_eq("role", "user")
    .where_condition("last_login < '2023-01-01'");

let affected = manager.update_with_builder(update).await?;
```

### 删除操作 (DELETE)

```rust
// 使用删除构建器
let delete = manager.delete_builder("fastcdn_api_tokens")
    .where_eq("state", "0")
    .where_eq("role", "user");

let affected = manager.delete_with_builder(delete).await?;

// 按ID删除
let delete = manager.delete_builder("users")
    .where_id(123);

let affected = manager.delete_with_builder(delete).await?;
```

## 构建器方法说明

### QueryBuilder 方法

- `select(&[&str])` - 指定要查询的列
- `select_all()` - 查询所有列 (SELECT *)
- `where_eq(column, value)` - 添加等值条件
- `where_condition(condition)` - 添加自定义条件
- `where_with_param(condition, param)` - 添加带参数的条件
- `order_by(column, direction)` - 排序 (ASC/DESC)
- `limit(count)` - 限制结果数量
- `offset(count)` - 跳过指定数量的记录

### InsertBuilder 方法

- `set(column, value)` - 设置字段值 (JSON Value)
- `set_str(column, value)` - 设置字符串值
- `set_int(column, value)` - 设置整数值
- `set_bool(column, value)` - 设置布尔值

### UpdateBuilder 方法

- `set(column, value)` - 设置字段值 (JSON Value)
- `set_str(column, value)` - 设置字符串值
- `set_int(column, value)` - 设置整数值
- `set_bool(column, value)` - 设置布尔值
- `where_eq(column, value)` - 添加等值条件
- `where_id(id)` - 按ID条件更新

### DeleteBuilder 方法

- `where_eq(column, value)` - 添加等值条件
- `where_id(id)` - 按ID条件删除

## 表前缀功能

数据库管理器支持表前缀功能，用于实现多租户、环境隔离等场景。

### 基本用法

```rust
use fastcdn_common::db::pool::Manager;

// 获取数据库管理器实例
let db = Manager::instance().await?;

// 设置表前缀
db.set_table_prefix("app_".to_string());

// 获取带前缀的表名
let table_name = db.get_table_name("users"); // 返回 "app_users"

// 创建带前缀的新实例
let tenant_db = db.with_prefix("tenant_a_");
let table_name = tenant_db.get_table_name("orders"); // 返回 "tenant_a_orders"
```

### 表前缀方法

- `set_table_prefix(prefix: String)` - 设置表前缀
- `get_table_prefix() -> String` - 获取当前表前缀
- `get_table_name(table: &str) -> String` - 获取带前缀的表名
- `with_prefix(prefix: &str) -> Manager` - 创建带指定前缀的新实例

### 使用场景

1. **多租户应用**
```rust
let tenant_a_db = db.with_prefix("tenant_a_");
let tenant_b_db = db.with_prefix("tenant_b_");
```

2. **环境隔离**
```rust
let dev_db = db.with_prefix("dev_");
let test_db = db.with_prefix("test_");
let prod_db = db.with_prefix(""); // 生产环境无前缀
```

3. **模块化应用**
```rust
let user_db = db.with_prefix("user_");
let order_db = db.with_prefix("order_");
```

### 自动应用

表前缀会自动应用到所有数据库操作中：
- 传统方法：`insert`, `select`, `update`, `delete`, `count`
- 查询构建器：`query_builder`, `insert_builder`, `update_builder`, `delete_builder`

## 兼容性

新的查询构建器API与原有的方法完全兼容。你可以：

1. 继续使用原有的 `select()`, `insert()`, `update()`, `delete()` 方法
2. 逐步迁移到新的构建器API
3. 在同一个项目中混合使用两种方式

表前缀功能对所有方法都有效。

## 最佳实践

1. **新代码推荐使用构建器API** - 更清晰、更安全
2. **复杂查询使用构建器** - 提高可读性
3. **简单操作可以继续使用原有方法** - 保持简洁
4. **团队开发统一使用构建器API** - 保持代码风格一致

## 示例代码

完整的使用示例请参考 `examples.rs` 文件，其中包含了各种常见场景的使用方法。

## 类型安全

新的API提供了更好的类型安全性：

- 列名使用字符串数组而不是逗号分隔的字符串
- 参数绑定更加明确
- 编译时可以发现更多错误
- IDE 自动补全支持更好

## 性能

查询构建器在运行时的性能开销极小：

- 构建器对象很轻量
- SQL 生成逻辑简单高效
- 最终执行的SQL与手写SQL相同
- 参数绑定方式保持不变