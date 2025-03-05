pub mod area;
pub mod project;
pub mod tag;
pub mod task;

pub use area::*;
pub use project::*;
pub use tag::*;
pub use task::*;

use bitflags::bitflags;
use serde::Deserialize;

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
pub enum UpdateMethod<T> {
    Change(T),
    Remove,
}

impl<T> UpdateMethod<T> {
    pub fn get_param(&self) -> Option<&T> {
        match self {
            UpdateMethod::Change(o) => Some(o),
            UpdateMethod::Remove => None,
        }
    }
}

impl<T> From<UpdateMethod<T>> for Option<T> {
    fn from(value: UpdateMethod<T>) -> Self {
        match value {
            UpdateMethod::Change(o) => Some(o),
            UpdateMethod::Remove => None,
        }
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
            QueryMethod::Compare(o, _) | QueryMethod::Match(o) => Some(o),
            QueryMethod::NotNull(..) => None,
        }
    }
}
