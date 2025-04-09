use std::fmt::Write;

use axum_extra::extract::CookieJar;
use deadpool_postgres::{Manager, ManagerConfig, Pool, PoolError};
use tokio_postgres::{Config, NoTls, types::ToSql};
use uuid::Uuid;

use crate::error::Error;

pub async fn get_database_pool(
    host: String,
    port: u16,
    database: String,
    user: String,
    pass: String,
) -> Result<Pool, PoolError> {
    let mut pg_config = Config::new();
    pg_config.host(host).port(port);
    pg_config.user(user).password(pass).dbname(database);

    let manager = Manager::from_config(pg_config, NoTls, ManagerConfig::default());

    let pool = Pool::builder(manager).max_size(16).build().unwrap();
    let _ = pool.get().await?;

    Ok(pool)
}

pub fn extract_user_id(cookies: &CookieJar) -> Result<Uuid, Error> {
    let cookie = match cookies.get("todo_app_user_id") {
        Some(c) => c,
        None => return Err(Error::InvalidRequest("User ID was not sent".to_string())),
    };

    match Uuid::try_parse(cookie.value()) {
        Ok(i) => Ok(i),
        Err(_) => Err(Error::InvalidRequest(
            "User ID was not a UUID format".to_string(),
        )),
    }
}

pub const NULL: Option<String> = None;

// TODO: make something to convert into sql query string, ie `column = $1`, etc
pub enum PostgresCmp {
    Equal,
    NotEqual,
    Less,
    LessEq,
    Greater,
    GreaterEq,
    IsNull,
    NotNull,
    Like,
    ILike,
}

impl PostgresCmp {
    pub fn as_sql_cmp(&self) -> &str {
        match self {
            PostgresCmp::Equal => "=",
            PostgresCmp::NotEqual => "<>",
            PostgresCmp::Less => "<",
            PostgresCmp::LessEq => "<=",
            PostgresCmp::Greater => ">",
            PostgresCmp::GreaterEq => ">=",
            PostgresCmp::IsNull => "ISNULL",
            PostgresCmp::NotNull => "NOTNULL",
            PostgresCmp::Like => "LIKE",
            PostgresCmp::ILike => "ILIKE",
        }
    }
}

pub trait ToPostgresCmp {
    fn to_postgres_cmp(&self) -> PostgresCmp;
}

/// SQL Query Builder
///
/// This may/should become its own crate
/// (most likely for personal use only)
#[derive(Default)]
pub struct SQLQueryBuilder<'a> {
    table: String,
    columns: Vec<(String, &'a (dyn ToSql + Sync))>,
    conditions: Vec<(String, PostgresCmp, &'a (dyn ToSql + Sync))>,
    _placeholder: Vec<String>,
    limit: Option<usize>,
    offset: Option<usize>,
    return_columns: Vec<String>,
}

impl<'a> SQLQueryBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get element at a given index in columns
    pub fn get_column(&self, index: usize) -> Option<&&'a (dyn ToSql + Sync)> {
        match self.columns.get(index) {
            Some((_, val)) => Some(val),
            None => None,
        }
    }

    /// Set table to query on
    pub fn set_table(&mut self, table_name: &str) {
        self.table = table_name.to_string();
    }

    /// Add value for a given column
    pub fn add_column(&mut self, column: &str, value: &'a (dyn ToSql + Sync)) {
        self.columns.push((column.to_string(), value));
    }

    /// Adds a condition to query
    ///
    /// If cmp is an operator that does not require a value, then the value is ignored.
    /// However, a value must be included.
    ///
    /// A `NULL` value is provided in the crate for this purpose.
    pub fn add_condition(&mut self, column: &str, cmp: PostgresCmp, value: &'a (dyn ToSql + Sync)) {
        self.conditions.push((column.to_string(), cmp, value));
    }

    /// Set limit for query
    pub fn set_limit(&mut self, limit: usize) {
        self.limit = Some(limit);
    }

    /// Set offset for query
    pub fn set_offset(&mut self, offset: usize) {
        self.offset = Some(offset);
    }

    /// Set query to return columns given
    pub fn set_return(&mut self, columns: Vec<&str>) {
        self.return_columns = columns.iter().map(|s| s.to_string()).collect();
    }

    /// Sets query to return all columns
    pub fn set_return_all(&mut self) {
        self.return_columns.clear();
        self.return_columns.push("*".to_string());
    }

    /// Builds an SQL SELECT query
    ///
    /// Consumes query builder after use
    pub fn build_select(self) -> (String, Vec<&'a (dyn ToSql + Sync)>) {
        let mut query: String = String::from("SELECT");
        let mut param_n = 1;
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();

        // Add columns to retrieve
        if self.return_columns.is_empty() || self.return_columns[0].eq("*") {
            query.push_str(" *");
        } else {
            query.push(' ');
            query.push_str(&self.return_columns.join(", "));
        }

        // Add table to retrieve from
        if self.table.is_empty() {
            panic!();
        }
        query.push_str(" FROM ");
        query.push_str(&self.table);

        // Add any conditions to select
        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");

            for (n, (col, cmp, val)) in self.conditions.iter().enumerate() {
                if n > 0 {
                    query.push_str(" AND ");
                }

                match cmp {
                    PostgresCmp::IsNull => write!(query, "{} ISNULL", col).unwrap(),
                    PostgresCmp::NotNull => write!(query, "{} NOTNULL", col).unwrap(),
                    PostgresCmp::Like => {
                        write!(query, "{} LIKE '%' || ${} || '%'", col, param_n).unwrap();
                        param_n += 1;
                        params.push(val.to_owned());
                    }
                    PostgresCmp::ILike => {
                        write!(query, "{} ILIKE '%' || ${} || '%'", col, param_n).unwrap();
                        param_n += 1;
                        params.push(val.to_owned());
                    }
                    _ => {
                        write!(query, "{} {} ${}", col, cmp.as_sql_cmp(), param_n).unwrap();
                        param_n += 1;
                        params.push(val.to_owned());
                    }
                }
            }
        }

        // Add limit
        if let Some(limit) = self.limit {
            write!(query, " LIMIT {}", limit).unwrap();
        }

        // Add offset
        if let Some(offset) = self.offset {
            write!(query, " OFFSET {}", offset).unwrap();
        }

        (query, params)
    }

    /// Builds an SQL INSERT query
    ///
    /// Consumes query builder after use
    pub fn build_insert(self) -> (String, Vec<&'a (dyn ToSql + Sync)>) {
        let mut query: String = String::from("INSERT");
        let mut param_n = 1;
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();

        // Add table to insert into
        if self.table.is_empty() {
            panic!();
        }
        query.push_str(" INTO ");
        query.push_str(&self.table);

        // Add columns to get from
        if self.columns.is_empty() {
            panic!();
        }
        let (col, val): (Vec<String>, Vec<&'a (dyn ToSql + Sync)>) =
            self.columns.iter().cloned().unzip();
        query.push_str(" (");
        query.push_str(&col.join(", "));
        query.push_str(") VALUES (");
        for (n, val) in val.iter().enumerate() {
            if n > 0 {
                query.push_str(", ");
            }

            write!(query, "${}", param_n).unwrap();
            param_n += 1;
            params.push(val.to_owned());
        }
        query.push(')');

        // Add returning columns
        if !self.return_columns.is_empty() {
            if self.return_columns[0].eq("*") {
                query.push_str(" RETURNING *");
            } else {
                query.push_str(" RETURNING ");
                query.push_str(&self.return_columns.join(", "));
            }
        }

        (query, params)
    }

    /// Builds an SQL UPDATE query
    ///
    /// Consumes query builder after use
    pub fn build_update(self) -> (String, Vec<&'a (dyn ToSql + Sync)>) {
        let mut query: String = String::from("UPDATE");
        let mut param_n = 1;
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();

        // Add table to update
        if self.table.is_empty() {
            panic!();
        }
        query.push(' ');
        query.push_str(&self.table);

        // Add columns to update to
        if self.columns.is_empty() {
            panic!();
        }
        query.push_str(" SET ");
        for (n, (col, val)) in self.columns.iter().enumerate() {
            if n > 0 {
                query.push_str(", ");
            }

            write!(query, "{}=${}", col, param_n).unwrap();
            param_n += 1;
            params.push(val.to_owned());
        }

        // Add conditions
        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");

            for (n, (col, cmp, val)) in self.conditions.iter().enumerate() {
                if n > 0 {
                    query.push_str(" AND ");
                }

                match cmp {
                    PostgresCmp::IsNull => write!(query, "{} ISNULL", col).unwrap(),
                    PostgresCmp::NotNull => write!(query, "{} NOTNULL", col).unwrap(),
                    PostgresCmp::Like => {
                        write!(query, "{} LIKE '%' || ${} || '%'", col, param_n).unwrap();
                        param_n += 1;
                        params.push(val.to_owned());
                    }
                    PostgresCmp::ILike => {
                        write!(query, "{} ILIKE '%' || ${} || '%'", col, param_n).unwrap();
                        param_n += 1;
                        params.push(val.to_owned());
                    }
                    _ => {
                        write!(query, "{} {} ${}", col, cmp.as_sql_cmp(), param_n).unwrap();
                        param_n += 1;
                        params.push(val.to_owned());
                    }
                }
            }
        }

        // Add returning columns
        if !self.return_columns.is_empty() {
            if self.return_columns[0].eq("*") {
                query.push_str(" RETURNING *");
            } else {
                query.push_str(" RETURNING ");
                query.push_str(&self.return_columns.join(", "));
            }
        }

        (query, params)
    }

    /// Builds an SQL DELETE query
    ///
    /// Consumes query builder after use
    pub fn build_delete(self) -> (String, Vec<&'a (dyn ToSql + Sync)>) {
        let mut query: String = String::from("DELETE");
        let mut param_n = 1;
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();

        // Add table to delete from
        if self.table.is_empty() {
            panic!();
        }
        query.push_str(" FROM ");
        query.push_str(&self.table);

        // Add conditions
        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            for (n, (col, cmp, val)) in self.conditions.iter().enumerate() {
                if n > 0 {
                    query.push_str(" AND ");
                }

                match cmp {
                    PostgresCmp::IsNull => write!(query, "{} ISNULL", col).unwrap(),
                    PostgresCmp::NotNull => write!(query, "{} NOTNULL", col).unwrap(),
                    PostgresCmp::Like => {
                        write!(query, "{} LIKE '%' || ${} || '%'", col, param_n).unwrap();
                        param_n += 1;
                        params.push(val.to_owned());
                    }
                    PostgresCmp::ILike => {
                        write!(query, "{} ILIKE '%' || ${} || '%'", col, param_n).unwrap();
                        param_n += 1;
                        params.push(val.to_owned());
                    }
                    _ => {
                        write!(query, "{} {} ${}", col, cmp.as_sql_cmp(), n + 1).unwrap();
                        param_n += 1;
                        params.push(val.to_owned());
                    }
                }
            }
        }

        // Add returning columns
        if !self.return_columns.is_empty() {
            if self.return_columns[0].eq("*") {
                query.push_str(" RETURNING *");
            } else {
                query.push_str(" RETURNING ");
                query.push_str(&self.return_columns.join(", "));
            }
        }

        (query, params)
    }
}

pub trait AddToQuery<'a, 'b> {
    fn add_to_query(&'a self, builder: &'b mut SQLQueryBuilder<'a>);
}

#[cfg(test)]
mod select_builder_tests {
    use tokio_postgres::types::ToSql;

    use super::{NULL, PostgresCmp, SQLQueryBuilder};

    // TODO: as ISNULL and NOTNULL to tests

    #[test]
    #[should_panic]
    fn no_title() {
        let builder = SQLQueryBuilder::new();

        builder.build_select();
    }

    #[test]
    fn empty() {
        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        let (statement, params) = builder.build_select();

        assert_eq!(statement.as_str(), "SELECT * FROM table");
        assert_eq!(params.len(), 0);
    }

    #[test]
    fn one_column() {
        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.set_return(vec!["col_1"]);

        let (statement, params) = builder.build_select();

        assert_eq!(statement.as_str(), "SELECT col_1 FROM table");
        assert_eq!(params.len(), 0);
    }

    #[test]
    fn many_columns() {
        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.set_return(vec!["col_1", "col_2", "col_3"]);

        let (statement, params) = builder.build_select();

        assert_eq!(statement.as_str(), "SELECT col_1, col_2, col_3 FROM table");
        assert_eq!(params.len(), 0);
    }

    #[test]
    fn all_columns() {
        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.set_return_all();

        let (statement, params) = builder.build_select();

        assert_eq!(statement.as_str(), "SELECT * FROM table");
        assert_eq!(params.len(), 0);
    }

    #[test]
    fn one_condition() {
        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        let val_1 = 10;
        builder.add_condition("col_1", PostgresCmp::Less, &val_1 as &(dyn ToSql + Sync));

        let (statement, params) = builder.build_select();

        assert_eq!(statement.as_str(), "SELECT * FROM table WHERE col_1 < $1");
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn many_conditions() {
        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        let val_1 = 10;
        let val_2 = 35;
        builder.add_condition("col_1", PostgresCmp::Less, &val_1 as &(dyn ToSql + Sync));
        builder.add_condition("col_2", PostgresCmp::Equal, &val_2 as &(dyn ToSql + Sync));
        builder.add_condition("col_3", PostgresCmp::IsNull, &NULL);

        let (statement, params) = builder.build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT * FROM table WHERE col_1 < $1 AND col_2 = $2 AND col_3 ISNULL"
        );
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn columns_and_conditions() {
        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.set_return(vec!["col_1"]);

        let val_1 = 10;
        let val_2 = 35;
        let val_3 = 150;
        builder.add_condition("col_1", PostgresCmp::Less, &val_1 as &(dyn ToSql + Sync));
        builder.add_condition("col_2", PostgresCmp::Equal, &val_2 as &(dyn ToSql + Sync));
        builder.add_condition("col_3", PostgresCmp::Greater, &val_3 as &(dyn ToSql + Sync));

        let (statement, params) = builder.build_select();

        assert_eq!(
            statement.as_str(),
            "SELECT col_1 FROM table WHERE col_1 < $1 AND col_2 = $2 AND col_3 > $3"
        );
        assert_eq!(params.len(), 3);
    }
}

#[cfg(test)]
mod insert_builder_tests {
    use super::SQLQueryBuilder;

    #[test]
    #[should_panic]
    fn no_title() {
        let builder = SQLQueryBuilder::new();

        builder.build_insert();
    }

    #[test]
    #[should_panic]
    fn empty() {
        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.build_insert();
    }

    #[test]
    fn one_column() {
        let col_1 = String::from("Sample data");

        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.add_column("col_1", &col_1);

        let (statement, params) = builder.build_insert();

        assert_eq!(statement.as_str(), "INSERT INTO table (col_1) VALUES ($1)");
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn many_columns() {
        let col_1 = uuid::Uuid::new_v4();
        let col_2 = String::from("Sample Data");
        let col_3 = chrono::Local::now().date_naive();

        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.add_column("col_1", &col_1);
        builder.add_column("col_2", &col_2);
        builder.add_column("col_3", &col_3);

        let (statement, params) = builder.build_insert();

        assert_eq!(
            statement.as_str(),
            "INSERT INTO table (col_1, col_2, col_3) VALUES ($1, $2, $3)"
        );
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn return_one_column() {
        let col_1 = String::from("Sample data");

        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.add_column("col_1", &col_1);

        builder.set_return(vec!["col_1"]);

        let (statement, params) = builder.build_insert();

        assert_eq!(
            statement.as_str(),
            "INSERT INTO table (col_1) VALUES ($1) RETURNING col_1"
        );
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn return_many_columns() {
        let col_1 = uuid::Uuid::new_v4();
        let col_2 = String::from("Sample Data");
        let col_3 = chrono::Local::now().date_naive();

        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.add_column("col_1", &col_1);
        builder.add_column("col_2", &col_2);
        builder.add_column("col_3", &col_3);

        builder.set_return(vec!["col_1", "col_2"]);

        let (statement, params) = builder.build_insert();

        assert_eq!(
            statement.as_str(),
            "INSERT INTO table (col_1, col_2, col_3) VALUES ($1, $2, $3) RETURNING col_1, col_2"
        );
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn return_all_columns() {
        let col_1 = uuid::Uuid::new_v4();
        let col_2 = String::from("Sample Data");
        let col_3 = chrono::Local::now().date_naive();

        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.add_column("col_1", &col_1);
        builder.add_column("col_2", &col_2);
        builder.add_column("col_3", &col_3);

        builder.set_return_all();

        let (statement, params) = builder.build_insert();

        assert_eq!(
            statement.as_str(),
            "INSERT INTO table (col_1, col_2, col_3) VALUES ($1, $2, $3) RETURNING *"
        );
        assert_eq!(params.len(), 3);
    }
}

#[cfg(test)]
mod update_builder_tests {
    use tokio_postgres::types::ToSql;

    use super::{PostgresCmp, SQLQueryBuilder};

    // TODO: as ISNULL and NOTNULL to tests

    #[test]
    #[should_panic]
    fn no_title() {
        let builder = SQLQueryBuilder::new();

        builder.build_update();
    }

    #[test]
    #[should_panic]
    fn empty() {
        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.build_update();
    }

    #[test]
    fn one_column() {
        let col_1 = String::from("Sample Data");

        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.add_column("col_1", &col_1);

        let (statement, params) = builder.build_update();

        assert_eq!(statement.as_str(), "UPDATE table SET col_1=$1");
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn many_columns() {
        let col_1 = uuid::Uuid::new_v4();
        let col_2 = String::from("Sample Data");
        let col_3 = chrono::Local::now().date_naive();

        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.add_column("col_1", &col_1);
        builder.add_column("col_2", &col_2);
        builder.add_column("col_3", &col_3);

        let (statement, params) = builder.build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE table SET col_1=$1, col_2=$2, col_3=$3"
        );
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn one_condition() {
        let col_1 = String::from("Sample Data");

        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.add_column("col_1", &col_1);

        let val_1 = 150;
        builder.add_condition("col_2", PostgresCmp::Less, &val_1 as &(dyn ToSql + Sync));

        let (statement, params) = builder.build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE table SET col_1=$1 WHERE col_2 < $2"
        );
        assert_eq!(params.len(), 2)
    }

    #[test]
    fn many_conditions() {
        let col_1 = String::from("Sample Data");

        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.add_column("col_1", &col_1);

        let val_1 = 150;
        let val_2 = 14;
        builder.add_condition("col_2", PostgresCmp::Less, &val_1 as &(dyn ToSql + Sync));
        builder.add_condition("col_3", PostgresCmp::Equal, &val_2 as &(dyn ToSql + Sync));

        let (statement, params) = builder.build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE table SET col_1=$1 WHERE col_2 < $2 AND col_3 = $3"
        );
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn columns_and_conditions() {
        let col_1 = uuid::Uuid::new_v4();
        let col_2 = String::from("Sample Data");
        let col_3 = chrono::Local::now().date_naive();

        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.add_column("col_1", &col_1);
        builder.add_column("col_2", &col_2);
        builder.add_column("col_3", &col_3);

        let val_1 = 150;
        let val_2 = 14;
        builder.add_condition("col_2", PostgresCmp::Less, &val_1 as &(dyn ToSql + Sync));
        builder.add_condition("col_3", PostgresCmp::Equal, &val_2 as &(dyn ToSql + Sync));

        let (statement, params) = builder.build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE table SET col_1=$1, col_2=$2, col_3=$3 WHERE col_2 < $4 AND col_3 = $5"
        );
        assert_eq!(params.len(), 5);
    }

    #[test]
    fn return_one_column() {
        let col_1 = String::from("Sample Data");

        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.add_column("col_1", &col_1);

        builder.set_return(vec!["col_1"]);

        let (statement, params) = builder.build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE table SET col_1=$1 RETURNING col_1"
        );
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn return_many_columns() {
        let col_1 = String::from("Sample Data");

        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.add_column("col_1", &col_1);

        builder.set_return(vec!["col_1", "col_2", "col_3"]);

        let (statement, params) = builder.build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE table SET col_1=$1 RETURNING col_1, col_2, col_3"
        );
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn return_all_columns() {
        let col_1 = String::from("Sample Data");

        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.add_column("col_1", &col_1);

        let val_1 = 150;
        builder.add_condition("col_2", PostgresCmp::Less, &val_1 as &(dyn ToSql + Sync));

        builder.set_return_all();

        let (statement, params) = builder.build_update();

        assert_eq!(
            statement.as_str(),
            "UPDATE table SET col_1=$1 WHERE col_2 < $2 RETURNING *"
        );
        assert_eq!(params.len(), 2);
    }
}

#[cfg(test)]
mod delete_builder_tests {
    use tokio_postgres::types::ToSql;

    use super::{PostgresCmp, SQLQueryBuilder};

    // TODO: as ISNULL and NOTNULL to tests

    #[test]
    #[should_panic]
    fn no_title() {
        let builder = SQLQueryBuilder::new();

        builder.build_delete();
    }

    #[test]
    fn empty() {
        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        let (statement, params) = builder.build_delete();

        assert_eq!(statement.as_str(), "DELETE FROM table");
        assert_eq!(params.len(), 0);
    }

    #[test]
    fn one_condition() {
        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        let val_1 = 150;
        builder.add_condition("col_2", PostgresCmp::Less, &val_1 as &(dyn ToSql + Sync));

        let (statement, params) = builder.build_delete();

        assert_eq!(statement.as_str(), "DELETE FROM table WHERE col_2 < $1");
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn many_conditions() {
        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        let val_1 = 150;
        let val_2 = 18;
        builder.add_condition("col_2", PostgresCmp::Less, &val_1 as &(dyn ToSql + Sync));
        builder.add_condition("col_3", PostgresCmp::Equal, &val_2 as &(dyn ToSql + Sync));

        let (statement, params) = builder.build_delete();

        assert_eq!(
            statement.as_str(),
            "DELETE FROM table WHERE col_2 < $1 AND col_3 = $2"
        );
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn return_one_column() {
        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.set_return(vec!["col_1"]);

        let (statement, params) = builder.build_delete();

        assert_eq!(statement.as_str(), "DELETE FROM table RETURNING col_1");
        assert_eq!(params.len(), 0);
    }

    #[test]
    fn return_many_columns() {
        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.set_return(vec!["col_1", "col_2"]);

        let (statement, params) = builder.build_delete();

        assert_eq!(
            statement.as_str(),
            "DELETE FROM table RETURNING col_1, col_2"
        );
        assert_eq!(params.len(), 0);
    }

    #[test]
    fn return_all_columns() {
        let mut builder = SQLQueryBuilder::new();
        builder.set_table("table");

        builder.set_return_all();

        let (statement, params) = builder.build_delete();

        assert_eq!(statement.as_str(), "DELETE FROM table RETURNING *");
        assert_eq!(params.len(), 0);
    }
}
