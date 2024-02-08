pub mod common;
pub mod english;
pub mod ruby_match;
pub mod segmentation;
pub mod cantonese;

use pyo3::prelude::*;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn zilib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(common::letter_count, m)?)?;
    m.add_function(wrap_pyfunction!(common::remove_unicode_other, m)?)?;
    m.add_function(wrap_pyfunction!(common::is_cjk, m)?)?;
    m.add_function(wrap_pyfunction!(common::is_cjk_cp, m)?)?;
    m.add_function(wrap_pyfunction!(common::has_cjk, m)?)?;
    m.add_function(wrap_pyfunction!(common::looks_like_a_sentence, m)?)?;
    m.add_function(wrap_pyfunction!(common::guess_language, m)?)?;

    m.add_function(wrap_pyfunction!(english::usa_english, m)?)?;
    m.add_function(wrap_pyfunction!(english::american_english_stem, m)?)?;

    m.add_function(wrap_pyfunction!(ruby_match::ruby_match, m)?)?;

    m.add_function(wrap_pyfunction!(segmentation::end_user_friendly_segment_with_dictionary, m)?)?;
    m.add_function(wrap_pyfunction!(segmentation::end_user_friendly_segment, m)?)?;

    m.add_function(wrap_pyfunction!(cantonese::get_ping3jam1, m)?)?;
    Ok(())
}
