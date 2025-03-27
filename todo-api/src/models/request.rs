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
    Remove,
    Change(T),
}

impl<T> UpdateMethod<T> {
    pub fn update_string(&self, column_name: &str, n: usize) -> (String, usize) {
        match self {
            UpdateMethod::Remove => (format!("{}=NULL", column_name), n),
            UpdateMethod::Change(_) => (format!("{}=${}", column_name, n), n + 1),
        }
    }

    pub fn get_param(&self) -> Option<&T> {
        match self {
            UpdateMethod::Remove => None,
            UpdateMethod::Change(o) => Some(o),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum QueryMethod<T> {
    NotNull(bool),
    Match(T),
    Compare(T, CmpFlag),
}

impl<T> QueryMethod<T> {
    pub fn condition_string(&self, column_name: &str, n: usize) -> (String, usize) {
        match self {
            QueryMethod::NotNull(b) => {
                if *b {
                    (format!("{} IS NOT NULL", column_name), n)
                } else {
                    (format!("{} IS NULL", column_name), n)
                }
            }
            QueryMethod::Match(_) => (format!("{}=${}", column_name, n), n + 1),
            QueryMethod::Compare(_, cmp) => {
                (format!("{}{}${}", column_name, cmp.to_sql_cmp(), n), n + 1)
            }
        }
    }

    pub fn get_param(&self) -> Option<&T> {
        match self {
            QueryMethod::NotNull(..) => None,
            QueryMethod::Match(o) | QueryMethod::Compare(o, _) => Some(o),
        }
    }
}
