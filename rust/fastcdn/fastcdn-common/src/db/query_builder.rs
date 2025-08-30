use serde_json::Value;
use std::collections::HashMap;

/// 查询构建器，提供更清晰的数据库操作API
#[derive(Debug, Clone)]
pub struct QueryBuilder {
    table: String,
    columns: Vec<String>,
    conditions: Vec<String>,
    params: Vec<String>,
    order_by: Option<String>,
    limit: Option<u64>,
    offset: Option<u64>,
}

impl QueryBuilder {
    /// 创建新的查询构建器
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            columns: Vec::new(),
            conditions: Vec::new(),
            params: Vec::new(),
            order_by: None,
            limit: None,
            offset: None,
        }
    }

    /// 选择列
    pub fn select(mut self, columns: &[&str]) -> Self {
        self.columns = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    /// 选择所有列
    pub fn select_all(mut self) -> Self {
        self.columns.clear();
        self
    }

    /// 添加WHERE条件
    pub fn where_eq(mut self, column: &str, value: &str) -> Self {
        self.conditions.push(format!("{} = ?", column));
        self.params.push(value.to_string());
        self
    }

    /// 添加WHERE条件（自定义条件）
    pub fn where_condition(mut self, condition: &str) -> Self {
        self.conditions.push(condition.to_string());
        self
    }

    /// 添加WHERE条件并绑定参数
    pub fn where_with_param(mut self, condition: &str, param: &str) -> Self {
        self.conditions.push(condition.to_string());
        self.params.push(param.to_string());
        self
    }

    /// 添加ORDER BY
    pub fn order_by(mut self, column: &str, direction: &str) -> Self {
        self.order_by = Some(format!("{} {}", column, direction));
        self
    }

    /// 添加LIMIT
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// 添加OFFSET
    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    /// 构建SQL查询语句
    pub fn build_sql(&self) -> String {
        let columns = if self.columns.is_empty() {
            "*".to_string()
        } else {
            self.columns.join(", ")
        };

        let mut sql = format!("SELECT {} FROM {}", columns, self.table);

        if !self.conditions.is_empty() {
            sql.push_str(&format!(" WHERE {}", self.conditions.join(" AND ")));
        }

        if let Some(ref order) = self.order_by {
            sql.push_str(&format!(" ORDER BY {}", order));
        }

        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = self.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        sql
    }

    /// 获取参数
    pub fn get_params(&self) -> &[String] {
        &self.params
    }

    /// 获取表名
    pub fn get_table(&self) -> &str {
        &self.table
    }
}

/// 插入构建器
#[derive(Debug, Clone)]
pub struct InsertBuilder {
    table: String,
    data: HashMap<String, Value>,
}

impl InsertBuilder {
    /// 创建新的插入构建器
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            data: HashMap::new(),
        }
    }

    /// 设置字段值
    pub fn set(mut self, column: &str, value: Value) -> Self {
        self.data.insert(column.to_string(), value);
        self
    }

    /// 设置字符串值
    pub fn set_str(mut self, column: &str, value: &str) -> Self {
        self.data.insert(column.to_string(), Value::String(value.to_string()));
        self
    }

    /// 设置整数值
    pub fn set_int(mut self, column: &str, value: i64) -> Self {
        self.data.insert(column.to_string(), Value::Number(serde_json::Number::from(value)));
        self
    }

    /// 设置布尔值
    pub fn set_bool(mut self, column: &str, value: bool) -> Self {
        self.data.insert(column.to_string(), Value::Bool(value));
        self
    }

    /// 获取数据
    pub fn get_data(&self) -> &HashMap<String, Value> {
        &self.data
    }

    /// 获取表名
    pub fn get_table(&self) -> &str {
        &self.table
    }
}

/// 更新构建器
#[derive(Debug, Clone)]
pub struct UpdateBuilder {
    table: String,
    data: HashMap<String, Value>,
    conditions: Vec<String>,
    params: Vec<String>,
}

impl UpdateBuilder {
    /// 创建新的更新构建器
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            data: HashMap::new(),
            conditions: Vec::new(),
            params: Vec::new(),
        }
    }

    /// 设置字段值
    pub fn set(mut self, column: &str, value: Value) -> Self {
        self.data.insert(column.to_string(), value);
        self
    }

    /// 设置字符串值
    pub fn set_str(mut self, column: &str, value: &str) -> Self {
        self.data.insert(column.to_string(), Value::String(value.to_string()));
        self
    }

    /// 设置整数值
    pub fn set_int(mut self, column: &str, value: i64) -> Self {
        self.data.insert(column.to_string(), Value::Number(serde_json::Number::from(value)));
        self
    }

    /// 设置布尔值
    pub fn set_bool(mut self, column: &str, value: bool) -> Self {
        self.data.insert(column.to_string(), Value::Bool(value));
        self
    }

    /// 添加WHERE条件
    pub fn where_eq(mut self, column: &str, value: &str) -> Self {
        self.conditions.push(format!("{} = ?", column));
        self.params.push(value.to_string());
        self
    }

    /// 添加WHERE条件（ID）
    pub fn where_id(mut self, id: u64) -> Self {
        self.conditions.push("id = ?".to_string());
        self.params.push(id.to_string());
        self
    }

    /// 添加WHERE条件（自定义条件）
    pub fn where_condition(mut self, condition: &str) -> Self {
        self.conditions.push(condition.to_string());
        self
    }

    /// 添加WHERE条件并绑定参数
    pub fn where_with_param(mut self, condition: &str, param: &str) -> Self {
        self.conditions.push(condition.to_string());
        self.params.push(param.to_string());
        self
    }

    /// 获取数据
    pub fn get_data(&self) -> &HashMap<String, Value> {
        &self.data
    }

    /// 获取条件
    pub fn get_conditions(&self) -> String {
        self.conditions.join(" AND ")
    }

    /// 获取参数
    pub fn get_params(&self) -> &[String] {
        &self.params
    }

    /// 获取表名
    pub fn get_table(&self) -> &str {
        &self.table
    }
}

/// 删除构建器
#[derive(Debug, Clone)]
pub struct DeleteBuilder {
    table: String,
    conditions: Vec<String>,
    params: Vec<String>,
}

impl DeleteBuilder {
    /// 创建新的删除构建器
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            conditions: Vec::new(),
            params: Vec::new(),
        }
    }

    /// 添加WHERE条件
    pub fn where_eq(mut self, column: &str, value: &str) -> Self {
        self.conditions.push(format!("{} = ?", column));
        self.params.push(value.to_string());
        self
    }

    /// 添加WHERE条件（ID）
    pub fn where_id(mut self, id: u64) -> Self {
        self.conditions.push("id = ?".to_string());
        self.params.push(id.to_string());
        self
    }

    /// 获取条件
    pub fn get_conditions(&self) -> String {
        self.conditions.join(" AND ")
    }

    /// 获取参数
    pub fn get_params(&self) -> &[String] {
        &self.params
    }

    /// 获取表名
    pub fn get_table(&self) -> &str {
        &self.table
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_builder() {
        let query = QueryBuilder::new("users")
            .select(&["id", "name", "email"])
            .where_eq("status", "active")
            .where_condition("age > 18")
            .order_by("created_at", "DESC")
            .limit(10)
            .offset(20);

        let sql = query.build_sql();
        assert_eq!(
            sql,
            "SELECT id, name, email FROM users WHERE status = ? AND age > 18 ORDER BY created_at DESC LIMIT 10 OFFSET 20"
        );
        assert_eq!(query.get_params(), &["active"]);
    }

    #[test]
    fn test_insert_builder() {
        let insert = InsertBuilder::new("users")
            .set_str("name", "John")
            .set_str("email", "john@example.com")
            .set_int("age", 25)
            .set_bool("active", true);

        assert_eq!(insert.get_table(), "users");
        assert_eq!(insert.get_data().len(), 4);
    }

    #[test]
    fn test_update_builder() {
        let update = UpdateBuilder::new("users")
            .set_str("name", "Jane")
            .set_int("age", 30)
            .where_id(1);

        assert_eq!(update.get_table(), "users");
        assert_eq!(update.get_conditions(), "id = ?");
        assert_eq!(update.get_params(), &["1"]);
    }

    #[test]
    fn test_delete_builder() {
        let delete = DeleteBuilder::new("users")
            .where_eq("status", "inactive")
            .where_eq("last_login", "2023-01-01");

        assert_eq!(delete.get_table(), "users");
        assert_eq!(delete.get_conditions(), "status = ? AND last_login = ?");
        assert_eq!(delete.get_params(), &["inactive", "2023-01-01"]);
    }
}