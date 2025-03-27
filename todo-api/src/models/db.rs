pub mod area;
pub mod project;
pub mod tag;
pub mod task;

pub use area::*;
pub use project::*;
pub use tag::*;
pub use task::*;

use std::ops::RangeInclusive;

pub fn parameter_values(range: RangeInclusive<usize>) -> Vec<String> {
    range.map(|n| format!("${}", n)).collect()
}
