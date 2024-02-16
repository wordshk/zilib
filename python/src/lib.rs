use std::collections::HashSet;

use zilib::cantonese;
use zilib::common;
use zilib::english;
use zilib::ruby_match;
use zilib::segmentation;
use zilib::data;

// Python (PyO3) bindings for functions in zilib

use pyo3::prelude::*;

/* START_OF_GENERATED_FUNCTION_WRAPPERS */
/// Gets the pronunciation of a Cantonese string from charlist.
#[pyfunction]
pub fn get_ping3jam1_from_charlist(chars:Vec<char>) -> Vec<Vec<String>> {
    cantonese::get_ping3jam1_from_charlist(chars)
}
/// Gets the pronunciation of a Cantonese string on a best effort basis. Each returned result
/// corresponds to a character in the input string. IF there are multiple pronunciations for a
/// character, multiple results are returned. If a character is not found in the dictionary, an
/// empty list is returned.
#[pyfunction]
pub fn get_ping3jam1(s: &str) -> String {
    cantonese::get_ping3jam1(s)
}
/// Regex string for validating formatting of Jyutping. Does not try to determine whether the
/// pronunciation is valid.
#[pyfunction]
pub fn jyutping_validator_string() -> String {
    cantonese::jyutping_validator_string()
}
/// Regex for validating formatting of Jyutping. Does not try to determine whether the
/// pronunciation is valid.
/// Validates the formatting of a Jyutping string. Does not try to determine whether the
/// pronunciation is valid.
#[pyfunction]
pub fn is_jyutping_valid(jyutping: &str) -> bool {
    cantonese::is_jyutping_valid(jyutping)
}
/// Ruby match. Returns a zipped (token, pronunciation) list of the structure of the match.
#[pyfunction]
pub fn ruby_match_zipped(txt: &str, pronunciation: &str) -> Vec<(String, String)> {
    ruby_match::ruby_match_zipped(txt, pronunciation)
}
/// Ruby match. Returns a plain text representation. Useful for unit testing (since the results are easier to understand)
#[pyfunction]
pub fn ruby_match_plain(txt: &str, pronunciation: &str) -> String {
    ruby_match::ruby_match_plain(txt, pronunciation)
}
/// Returns two lists as a pair: The first list contains indices of unmatched odd characters. The
/// second list contains the indices of the segments (the segment's start index).
#[pyfunction]
pub fn segment_with_dictionary(phrase: &str, dictionary: Option<HashSet<String>>) -> (Vec<usize>, Vec<usize>) {
    segmentation::segment_with_dictionary(phrase, dictionary.as_ref())
}
/// Returns a user-friendly segmentation result for text-based programs. If dictionary is None, we
/// will load an out-of-date Cantonese dictionary from words.hk
#[pyfunction]
pub fn end_user_friendly_segment(s: &str, dictionary: Option<HashSet<String>>) -> (Vec<char>, Vec<char>, Vec<String>) {
    segmentation::end_user_friendly_segment(s, dictionary.as_ref())
}
#[pyfunction]
pub fn usa_english(word : &str) -> String {
    english::usa_english(word)
}
#[pyfunction]
pub fn american_english_stem(w: &str) -> String {
    english::american_english_stem(w)
}
/// Returns the number of characters that are in the "Letter" category in unicode"""
#[pyfunction]
pub fn letter_count(s: &str) -> usize {
    common::letter_count(s)
}
/// Removes "other" categories from unicode, which are basically control characters, reserved and
/// private use, etc.
#[pyfunction]
pub fn remove_unicode_other(s: &str) -> String {
    common::remove_unicode_other(s)
}
/// Returns whether the code point is a CJK character
#[pyfunction]
pub fn is_cjk_cp(cp: u32) -> bool {
    common::is_cjk_cp(cp)
}
/// Returns whether all the characters of the string are CJK characters
#[pyfunction]
pub fn is_cjk(s: &str) -> bool {
    common::is_cjk(s)
}
/// Returns whether the string has any CJK characters
#[pyfunction]
pub fn has_cjk(s: &str) -> bool {
    common::has_cjk(s)
}
/// Returns the number of characters that are latin letters
#[pyfunction]
pub fn is_latin(s: &str) -> bool {
    common::is_latin(s)
}
#[pyfunction]
pub fn is_latin_c(c: char) -> bool {
    common::is_latin_c(c)
}
/// Returns true if the input looks like a Chinese/Cantonese sentence, assuming it uses
/// "Full-Width" punctuations. Do not assume this is very reliable
#[pyfunction]
pub fn looks_like_a_sentence(s: &str) -> bool {
    common::looks_like_a_sentence(s)
}
/// Returns a constant representing the language group the text is in. Might be inaccurate.
#[pyfunction]
pub fn guess_language(s: &str) -> String {
    common::guess_language(s)
}
/* END_OF_GENERATED_FUNCTION_WRAPPERS */

#[pyfunction]
pub fn ruby_match_max() -> u32 {
    ruby_match::RUBY_MATCH_MAX
}

#[pyfunction]
pub fn binary_search_file(
    path: &str,
    target: &[u8],
    record_delim: &[u8],
    field_delim: &[u8],
    start_pos: usize,
    end_pos: Option<usize>
) -> std::io::Result<Option<usize>> {
    let lexicalgraphical_cmp : fn(&[u8], &[u8]) -> std::cmp::Ordering = |a, b| {
        a.cmp(b)
    };
    if record_delim.len() != 1 || field_delim.len() != 1 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "record_delim and field_delim must be single byte"));
    }
    common::binary_search_file(path, target, record_delim[0], field_delim[0], start_pos, end_pos, 1, lexicalgraphical_cmp)

}

#[pyfunction]
pub fn initialize_data(kind: &str, path: &str) -> std::io::Result<()> {
    let kind = match kind {
        "CantoneseCharListWithJyutping" => data::DataKind::CantoneseCharListWithJyutping,
        "CantoneseWordListWithJyutping" => data::DataKind::CantoneseWordListWithJyutping,
        "RadicalLabelToChars" => data::DataKind::RadicalLabelToChars,
        "UnihanData" => data::DataKind::UnihanData,
        "EnglishVariants" => data::DataKind::EnglishVariants,
        _ => { return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid data kind string")); }
    };

    data::initialize_data(kind, path);
    Ok(())
}

#[pymodule]
#[pyo3(name="zilib")]
fn zilib_python(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    /* START_OF_GENERATED_ADD_FUNCTIONS */
    m.add_function(wrap_pyfunction!(get_ping3jam1_from_charlist, m)?)?;
    m.add_function(wrap_pyfunction!(get_ping3jam1, m)?)?;
    m.add_function(wrap_pyfunction!(jyutping_validator_string, m)?)?;
    m.add_function(wrap_pyfunction!(is_jyutping_valid, m)?)?;
    m.add_function(wrap_pyfunction!(ruby_match_zipped, m)?)?;
    m.add_function(wrap_pyfunction!(ruby_match_plain, m)?)?;
    m.add_function(wrap_pyfunction!(segment_with_dictionary, m)?)?;
    m.add_function(wrap_pyfunction!(end_user_friendly_segment, m)?)?;
    m.add_function(wrap_pyfunction!(usa_english, m)?)?;
    m.add_function(wrap_pyfunction!(american_english_stem, m)?)?;
    m.add_function(wrap_pyfunction!(letter_count, m)?)?;
    m.add_function(wrap_pyfunction!(remove_unicode_other, m)?)?;
    m.add_function(wrap_pyfunction!(is_cjk_cp, m)?)?;
    m.add_function(wrap_pyfunction!(is_cjk, m)?)?;
    m.add_function(wrap_pyfunction!(has_cjk, m)?)?;
    m.add_function(wrap_pyfunction!(is_latin, m)?)?;
    m.add_function(wrap_pyfunction!(is_latin_c, m)?)?;
    m.add_function(wrap_pyfunction!(looks_like_a_sentence, m)?)?;
    m.add_function(wrap_pyfunction!(guess_language, m)?)?;
    /* END_OF_GENERATED_ADD_FUNCTIONS */

    m.add_function(wrap_pyfunction!(ruby_match_max, m)?)?;
    m.add_function(wrap_pyfunction!(binary_search_file, m)?)?;
    m.add_function(wrap_pyfunction!(initialize_data, m)?)?;

    Ok(())
}
