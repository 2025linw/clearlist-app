pub mod area;
pub mod project;
pub mod tag;
pub mod task;

use axum_extra::extract::CookieJar;
use bytes::BytesMut;
use serde::Deserialize;
use tokio_postgres::types::{to_sql_checked, Format, IsNull, ToSql, Type};

use chrono::{DateTime, Local, NaiveDate};
use uuid::Uuid;

const TIMESTAMP_NULL: &Option<DateTime<Local>> = &None;

pub mod query {
    use bitflags::bitflags;
    use serde::Deserialize;
    use std::fmt::Display;
    use std::ops::RangeInclusive;

    #[rustfmt::skip]
	bitflags! {
		#[derive(Debug, Deserialize, PartialEq)]
		pub struct CmpFlag: u8 {
			const EQUAL 	= 	0b001;
			const BEFORE 	= 	0b010;
			const ON_BEFORE = 	0b011;
			const AFTER 	= 	0b100;
			const ON_AFTER 	= 	0b101;
			const NOT_EQUAL =	0b110;
			const IS_NULL 	= 	0b111;
		}
	}

    impl CmpFlag {
        pub fn to_sql_cmp(&self) -> &str {
            if self == &Self::EQUAL {
                return "=";
            } else if self == &Self::BEFORE {
                return "<";
            } else if self == &Self::AFTER {
                return ">";
            } else if self == &Self::ON_BEFORE {
                return "<=";
            } else if self == &Self::ON_AFTER {
                return ">=";
            } else if self == &Self::NOT_EQUAL {
                return "!=";
            } else if self == &Self::IS_NULL {
                return "NULL";
            }

            panic!()
        }

        pub fn is_null(&self) -> bool {
            if self == &Self::IS_NULL {
                true
            } else {
                false
            }
        }
    }

    pub struct ColumnCmp<T> {
        value: T,
        comparison: CmpFlag,
    }

    fn parameter_string(range: RangeInclusive<u8>) -> String {
        range
            .map(|n| format!("${}", n))
            .collect::<Vec<String>>()
            .join(", ")
    }

    enum Type {
        SELECT,
        INSERT,
        UPDATE,
        DELETE,
    }

    pub enum Condition {
        AND,
        OR,
    }

    impl Display for Condition {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Condition::AND => write!(f, "AND"),
                Condition::OR => write!(f, "OR"),
            }
        }
    }

    pub struct BuildQueryInit {
        query: String,
        query_type: Type,
    }

    impl BuildQueryInit {
        pub fn select(table_name: String) -> Self {
            Self {
                query: format!("SELECT * FROM {}", table_name),
                query_type: Type::SELECT,
            }
        }

        pub fn insert(table_name: String) -> Self {
            Self {
                query: format!("INSERT INTO {}", table_name),
                query_type: Type::INSERT,
            }
        }

        pub fn update(table_name: String) -> Self {
            Self {
                query: format!("UPDATE {}", table_name),
                query_type: Type::UPDATE,
            }
        }

        pub fn delete(table_name: String) -> Self {
            Self {
                query: format!("DELETE FROM {}", table_name),
                query_type: Type::DELETE,
            }
        }

        // Next stage
        pub fn next(self) -> BuildQueryColumn {
            match self.query_type {
                Type::SELECT | Type::DELETE => BuildQueryColumn {
                    query: format!("{} ", self.query),
                    query_type: self.query_type,
                    params: 0,
                },
                Type::INSERT => BuildQueryColumn {
                    query: format!("{} (", self.query),
                    query_type: self.query_type,
                    params: 0,
                },
                Type::UPDATE => BuildQueryColumn {
                    query: format!("{} SET (", self.query),
                    query_type: self.query_type,
                    params: 0,
                },
            }
        }
    }

    pub struct BuildQueryColumn {
        query: String,
        query_type: Type,
        params: u8,
    }

    impl BuildQueryColumn {
        pub fn column(&mut self, column_name: String) -> &mut Self {
            match self.query_type {
                Type::SELECT | Type::DELETE => return self,
                Type::INSERT => {
                    if self.params != 1 {
                        self.query.push(',');
                    }

                    self.query.push_str(column_name.as_str());
                    self.params += 1;
                }
                Type::UPDATE => {
                    if self.params != 1 {
                        self.query.push(',');
                    }

                    self.query.push_str(column_name.as_str());
                    self.params += 1;
                }
            }

            self
        }

        // Next stage
        pub fn next(self) -> BuildQueryCondition {
            match self.query_type {
                Type::SELECT => BuildQueryCondition {
                    query: format!("{} WHERE ", self.query),
                    query_type: self.query_type,
                    params: self.params,
                    conditions: 0,
                },
                Type::INSERT => BuildQueryCondition {
                    query: format!(
                        "{}) VALUES ({})",
                        self.query,
                        parameter_string(1..=self.params),
                    ),
                    query_type: self.query_type,
                    params: self.params,
                    conditions: 0,
                },
                Type::UPDATE => BuildQueryCondition {
                    query: format!(
                        "{})=({}) WHERE ",
                        self.query,
                        parameter_string(1..=self.params),
                    ),
                    query_type: self.query_type,
                    params: self.params,
                    conditions: 0,
                },
                Type::DELETE => BuildQueryCondition {
                    query: format!("{} WHERE ", self.query,),
                    query_type: self.query_type,
                    params: self.params,
                    conditions: 0,
                },
            }
        }
    }

    pub struct BuildQueryCondition {
        query: String,
        query_type: Type,
        params: u8,
        conditions: u8,
    }

    impl BuildQueryCondition {
        pub fn condition(
            &mut self,
            column: String,
            compare: CmpFlag,
            condition: Condition,
        ) -> &mut Self {
            match self.query_type {
                Type::INSERT => return self,
                Type::SELECT => {
                    if self.conditions != 0 {
                        self.query.push_str(format!(" {} ", condition).as_str());
                    }

                    // self.query.push
                    self.conditions += 1;
                }
                Type::UPDATE => {
                    if self.conditions != 0 {
                        self.query.push_str(format!("{}", condition).as_str());
                    }

                    self.conditions += 1;
                }
                Type::DELETE => {
                    if self.conditions != 0 {
                        self.query.push_str(format!("{}", condition).as_str());
                    }

                    self.conditions += 1;
                }
            }

            self
        }

        // Finish stage
        pub fn complete(self) -> String {
            // Close

            self.query
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct DateFilter {
    start: Option<NaiveDate>,
    #[serde(default)]
    start_cmp: Option<query::CmpFlag>,

    deadline: Option<NaiveDate>,
    #[serde(default)]
    deadline_cmp: Option<query::CmpFlag>,

    #[serde(default)]
    or: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
enum UpdateMethod<T>
where
    T: ToSql,
{
    Change(T),
    Remove,
}

impl<T: ToSql> ToSql for UpdateMethod<T> {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        match *self {
            UpdateMethod::Change(ref val) => val.to_sql(ty, out),
            UpdateMethod::Remove => Ok(IsNull::Yes),
        }
    }

    fn accepts(ty: &Type) -> bool {
        <T as ToSql>::accepts(ty)
    }

    fn encode_format(&self, ty: &Type) -> Format {
        match self {
            UpdateMethod::Change(ref val) => val.encode_format(ty),
            UpdateMethod::Remove => Format::Binary,
        }
    }

    to_sql_checked!();
}

pub fn extract_user_id(cookies: &CookieJar) -> Option<Uuid> {
    let cookie = match cookies.get("todo_app_user_id") {
        Some(c) => c,
        None => return None,
    };

    let user_id = match Uuid::try_parse(cookie.value()) {
        Ok(i) => i,
        Err(e) => panic!("Value was not a Uuid"),
    };

    Some(user_id)
}

pub mod api {
    use chrono::{DateTime, Local};
    use deadpool_postgres::Object;
    use tokio_postgres::{types::ToSql, Row};
    use uuid::Uuid;

    use crate::response::Error;

    pub struct InfoBuilder {
        user_id: Option<Uuid>,
        obj_id: Option<Uuid>,
        query: Option<String>,
    }

    impl InfoBuilder {
        pub fn new() -> Self {
            Self {
                user_id: None,
                obj_id: None,
                query: None,
            }
        }

        pub fn build(self) -> Info {
            Info {
                timestamp: Local::now(),
                user_id: self.user_id.unwrap(),
                obj_id: self.obj_id,
                query: self.query,
            }
        }

        pub fn user_id(&mut self, i: Uuid) -> &mut Self {
            self.user_id = Some(i);

            self
        }

        pub fn obj_id(&mut self, i: Uuid) -> &mut Self {
            self.obj_id = Some(i);

            self
        }

        pub fn query(&mut self, s: String) -> &mut Self {
            self.query = Some(s);

            self
        }
    }

    pub struct Info {
        timestamp: DateTime<Local>,
        user_id: Uuid,
        obj_id: Option<Uuid>,
        query: Option<String>,
    }

    impl Info {
        pub fn timestamp(&self) -> &DateTime<Local> {
            &self.timestamp
        }

        pub fn user_id(&self) -> &Uuid {
            &self.user_id
        }

        pub fn obj_id(&self) -> &Option<Uuid> {
            &self.obj_id
        }

        pub fn query(&self) -> &Option<String> {
            &self.query
        }
    }

    pub trait Create {
        fn columns_vec(&self) -> Vec<String>;
        fn params<'a>(&'a self) -> Vec<&'a (dyn ToSql + Sync)>;
        fn insert_query(
            &self,
            conn: &mut Object,
            info: Option<Info>,
        ) -> impl std::future::Future<Output = Result<Row, Error>> + Send;
    }

    pub trait Retrieve {
        fn select_query(
            &self,
            conn: &mut Object,
            info: Option<Info>,
        ) -> impl std::future::Future<Output = Result<Option<Row>, Error>> + Send;
    }

    pub trait Update {
        fn columns_vec(&self) -> Vec<String>;
        fn params<'a>(&'a self) -> Vec<&'a (dyn ToSql + Sync)>;
        fn update_query(
            &self,
            conn: &mut Object,
            info: Option<Info>,
        ) -> impl std::future::Future<Output = Result<Option<Row>, Error>> + Send;
    }

    pub trait Delete {
        fn delete_query(
            &self,
            conn: &mut Object,
            info: Option<Info>,
        ) -> impl std::future::Future<Output = Result<bool, Error>> + Send;
    }

    pub trait Query {
        fn conditions(&self) -> Vec<String>;
        fn params(&self) -> Vec<&(dyn ToSql + Sync)>;

        fn query(
            &self,
            conn: &mut Object,
            info: Option<Info>,
        ) -> impl std::future::Future<Output = Result<Vec<Row>, Error>> + Send;
    }
}
