use unicode_categories::UnicodeCategories;

use pyo3::pyfunction;
use crate::common::is_cjk_cp;

/// Returns the number of characters that are latin letters
#[pyfunction]
pub fn is_latin(s: &str) -> bool {
    // XXX: In the python implementation this actually counts the latin characters, but here it
    // counts all letters. There might be some differences in the results but it shouldn't matter.
    s.chars().all(|c| !is_cjk_cp(c as u32) && (c.is_letter_lowercase() || c.is_letter_uppercase()))
}
