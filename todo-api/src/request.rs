pub mod area;
pub mod project;
pub mod tag;
pub mod task;

use bytes::BytesMut;
use serde::Deserialize;
use tokio_postgres::types::{to_sql_checked, Format, IsNull, ToSql, Type};

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
// Do something similar to QueryMethod
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

pub mod api {
    use axum_extra::extract::CookieJar;
    use deadpool_postgres::Object;
    use tokio_postgres::Row;
    use uuid::Uuid;

    use crate::response::Error;

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

    pub trait Create {
        fn query(
            &self,
            conn: &mut Object,
        ) -> impl std::future::Future<Output = Result<Row, Error>> + Send;
    }

    pub trait Retrieve {
        fn query(
            &self,
            conn: &mut Object,
        ) -> impl std::future::Future<Output = Result<Option<Row>, Error>> + Send;
    }

    pub trait Update {
        fn query(
            &self,
            conn: &mut Object,
        ) -> impl std::future::Future<Output = Result<Option<Row>, Error>> + Send;
    }

    pub trait Delete {
        fn query(
            &self,
            conn: &mut Object,
        ) -> impl std::future::Future<Output = Result<bool, Error>> + Send;
    }

    pub trait Query {
        fn query(
            &self,
            conn: &mut Object,
        ) -> impl std::future::Future<Output = Result<Vec<Row>, Error>> + Send;
    }
}

pub mod query {
    use bitflags::bitflags;
    use chrono::{DateTime, Local};
    use serde::Deserialize;
    use std::ops::RangeInclusive;
    use tokio_postgres::types::ToSql;

    pub const TIMESTAMP_NULL: &Option<DateTime<Local>> = &None;

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
            }

            panic!()
        }
    }

    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    pub enum QueryMethod<T> {
        Compare(T, CmpFlag),
        Match(T),
        NotNull(bool),
    }

    impl<T> QueryMethod<T> {
        pub fn condition_string(&self, column_name: &str, n: usize) -> (String, usize) {
            match self {
                QueryMethod::Compare(_, cmp) => {
                    (format!("{}{}${}", column_name, cmp.to_sql_cmp(), n), n + 1)
                }
                QueryMethod::Match(_) => (format!("{}=${}", column_name, n), n + 1),
                QueryMethod::NotNull(b) => {
                    if *b {
                        (format!("{} IS NOT NULL", column_name), n)
                    } else {
                        (format!("{} IS NULL", column_name), n)
                    }
                }
            }
        }

        pub fn get_param(&self) -> Option<&T> {
            match self {
                QueryMethod::Compare(o, _) | QueryMethod::Match(o) => return Some(o),
                QueryMethod::NotNull(..) => return None,
            }
        }
    }

    pub fn parameter_string(range: RangeInclusive<usize>) -> String {
        range
            .map(|n| format!("${}", n))
            .collect::<Vec<String>>()
            .join(",")
    }

    pub trait ToQuery {
        fn statement(&self) -> String;
        fn params(&self) -> Vec<&(dyn ToSql + Sync)>;
    }
}
