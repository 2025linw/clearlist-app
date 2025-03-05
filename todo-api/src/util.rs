use std::ops::RangeInclusive;

pub fn parameter_values(range: RangeInclusive<usize>) -> Vec<String> {
    range.map(|n| format!("${}", n)).collect()
}
