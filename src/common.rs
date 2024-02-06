use unicode_categories::UnicodeCategories;

use pyo3::pyfunction;

/// Returns the number of characters that are in the "Letter" category in unicode"""
#[pyfunction]
pub fn letter_count(s: &str) -> usize {
    s.chars().filter(|c| c.is_letter()).count() // Count the number of letters
}

/// Removes "other" categories from unicode, which are basically control characters, reserved and
/// private use, etc.
#[pyfunction]
pub fn remove_unicode_other(s: &str) -> String {
    s.chars().filter(|c| *c == '\n' || *c == '\r' || !c.is_other()).collect::<String>()
}

/// Returns whether the code point is a CJK character
#[pyfunction]
pub fn is_cjk_cp(cp: u32) -> bool {
    (0x3400..=0x4DBF).contains(&cp) || (0x4E00..=0x9FFF).contains(&cp) || (0xF900..=0xFAFF).contains(&cp) || (0x20000..=0x2FFFF).contains(&cp)
}

/// Returns whether all the characters of the string are CJK characters
#[pyfunction]
pub fn is_cjk(s: &str) -> bool {
    s.chars().any(|c| !is_cjk_cp(c as u32)) == false
}

/// Returns whether the string has any CJK characters
#[pyfunction]
pub fn has_cjk(s: &str) -> bool {
    s.chars().any(|c| is_cjk_cp(c as u32))
}

/// Returns the number of characters that are latin letters
#[pyfunction]
pub fn is_latin(s: &str) -> bool {
    // XXX: In the python implementation this actually counts the latin characters, but here it
    // counts all letters. There might be some differences in the results but it shouldn't matter.
    s.chars().all(|c| !is_cjk_cp(c as u32) && (c.is_letter_lowercase() || c.is_letter_uppercase()))
}

/// Returns true if the input looks like a Chinese/Cantonese sentence, assuming it uses
/// "Full-Width" punctuations. Do not assume this is very reliable
#[pyfunction]
pub fn looks_like_a_sentence(s: &str) -> bool {
    s.len() > 8 || "。，；？「」！".chars().any(|punct| s.find(punct).is_some())
}

enum LanguageGroup {
    Unknown,
    Chinese,
    English,
}

impl ToString for LanguageGroup {
    fn to_string(&self) -> String {
        match self {
            LanguageGroup::Unknown => "xx".to_string(),
            LanguageGroup::Chinese => "zh".to_string(),
            LanguageGroup::English => "en".to_string(),
        }
    }
}

/// Implementation of guess_language
fn _guess_language(s: &str) -> LanguageGroup {
    let cjk_count = s.chars().filter(|c| is_cjk_cp(*c as u32)).count();

    // XXX: In the python implementation this actually counts the latin characters, but here it
    // counts all letters. There might be some differences in the results but it shouldn't matter.
    let latin_count = s.chars().filter(|c| c.is_letter_lowercase() || c.is_letter_uppercase() ).count();

    let k = letter_count(s);

    // There's really no point in guessing if there are no letters. (CJK are "letters" in unicode)
    if k == 0 {
        return LanguageGroup::Unknown;
    }

    // Don't treat as English if the input string is just two characters... it might as well be gibberish
    if latin_count > 2 && (latin_count as f64 / k as f64 > 0.5) {
        // Adjusting CJK-weight since one CJK char represents more meaning.
        // Otherwise, we will recognize this as 'English': 「Come on,
        // James，可不可以成熟一點呢」
        if latin_count > cjk_count * 3 {
            return LanguageGroup::English;
        } else {
            return LanguageGroup::Chinese;
        }
    }

    // Don't treat as Chinese if the input string is just one character... it might as well be gibberish (but two characters can be very meaningful)
    if cjk_count > 1 && cjk_count as f32 / k as f32 > 0.5 {
        return LanguageGroup::Chinese;
    }

    LanguageGroup::Unknown
}

/// Returns a constant representing the language group the text is in. Might be inaccurate.
pub fn guess_language(s: &str) -> String {
    _guess_language(s).to_string()
}

