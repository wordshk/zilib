use std::sync::OnceLock;
use crate::{data, segmentation};
use regex::Regex;

/// Gets the pronunciation of a Cantonese string from charlist.
pub fn get_ping3jam1_from_charlist(chars:Vec<char>) -> Vec<Vec<String>> {
    let charlist = data::cantonese_charlist_with_jyutping();
    chars.into_iter().map(|ch| charlist.get(&ch).map(|ps| ps.keys().map(|p| p.clone()).collect()).unwrap_or(vec![])).collect()
}

/// Gets the pronunciation of a Cantonese string from charlist, picking the most common pronunciation.
fn get_ping3jam1_from_charlist_most_common(chars:Vec<char>) -> Vec<String> {
    let charlist = data::cantonese_charlist_with_jyutping();
    chars.into_iter().map(|ch| charlist.get(&ch).map(|ps| ps.iter().max_by_key(|(_, &count)| count).map(|(p, _)| p.clone()).unwrap_or("".to_string())).unwrap_or("".to_string())).collect()
}

/// Gets the pronunciation of a Cantonese string from wordlist by first segmenting the string.
fn get_ping3jam1_from_wordlist(s: &str) -> Vec<String> {
    let wordlist = data::cantonese_wordlist_with_jyutping();
    let (_, _, segments) = segmentation::end_user_friendly_segment(s, None);

    let mut result = vec![];
    for segment in segments {
        // The segmentation result is 1 character, so we should use the most common
        // character instead.
        let word_pronunciation = wordlist.get(&segment).map(|ps| ps.iter().map(|p| p.clone()));
        if segment.chars().count() > 1 || word_pronunciation.is_some() {
            result.push(word_pronunciation.unwrap().next().unwrap()); // safe because the segmentation ensures that the word exists
        } else {
            for cps in get_ping3jam1_from_charlist_most_common(segment.chars().collect()) {
                result.push(cps);
            }
        }
    }
    result
}

/// Gets the pronunciation of a Cantonese string on a best effort basis. Each returned result
/// corresponds to a character in the input string. IF there are multiple pronunciations for a
/// character, multiple results are returned. If a character is not found in the dictionary, an
/// empty list is returned.
pub fn get_ping3jam1(s: &str) -> String {
    get_ping3jam1_from_wordlist(s).join(" ")
}

// From http://humanum.arts.cuhk.edu.hk/Lexis/Canton2/syllabary/ , revised manually with
// suggestions by Chaaak (updates from LSHK), and some are our own modifications (IIRC).
// Some combinations do not make sense, but these functions are not supposed to validate the
// pronunciations, but rather to just validate the format.
const JYUTPING_CONSONANTS : &str = "(b|p|m|f|d|t|n|l|g|k|ng|h|gw|kw|w|z|c|s|j)";
const JYUTPING_FINALS : &str = "(i|ip|it|ik|im|in|ing|iu|yu|yut|yun|u|up|ut|uk|um|un|ung|ui|e|ep|et|ek|em|en|eng|ei|eu|eot|eon|eoi|oe|oet|oek|oeng|o|ot|ok|on|ong|oi|ou|op|om|a|ap|at|ak|am|an|ang|ai|au|aa|aap|aat|aak|aam|aan|aang|aai|aau|m|ng)";
const JYUTPING_TONES : &str = "[1-6]";

/// Regex string for validating formatting of Jyutping. Does not try to determine whether the
/// pronunciation is valid.
pub fn jyutping_validator_string() -> String {
    format!("^{}?{}{}", JYUTPING_CONSONANTS, JYUTPING_FINALS, JYUTPING_TONES)
}

/// Regex for validating formatting of Jyutping. Does not try to determine whether the
/// pronunciation is valid.
pub fn jyutping_validator() -> &'static Regex {
    static JYUTPING_RE: OnceLock<Regex> = OnceLock::new();
    JYUTPING_RE.get_or_init(|| {
        Regex::new(&jyutping_validator_string()).unwrap() // XXX: unwrap error detectable immediately in tests
    })
}

/// Validates the formatting of a Jyutping string. Does not try to determine whether the
/// pronunciation is valid.
pub fn is_jyutping_valid(jyutping: &str) -> bool {
    jyutping_validator().is_match(jyutping)
}
