use pyo3::pyfunction;
use std::collections::HashMap;
use std::sync::OnceLock;
use crate::segmentation;

// A dictionary of (characters) => (lists of pronunciations)
pub fn charlist() -> &'static HashMap<char, HashMap<String, u64>> {
    static DATA: OnceLock<HashMap<char, HashMap<String, u64>>> = OnceLock::new();
    DATA.get_or_init(|| {
        // I tried using https://github.com/SOF3/include-flate and it didn't seem to work in terms
        // of file size reduction. Perhaps the overhead of decompression is too high.
        let json_data = include_str!("../lists/charlist.json");

        //               character  pronunciation count
        let data : HashMap<char, HashMap<String, u64>> = serde_json::from_str(json_data).unwrap(); // XXX: unwrap error detectable immediately in tests due to inclusion of string during build time

        data
    })
}

// A dictionary of (words) => (lists of pronunciations)
pub fn wordlist() -> &'static HashMap<String, Vec<String>> {
    static DATA: OnceLock<HashMap<String, Vec<String>>> = OnceLock::new();
    DATA.get_or_init(|| {
        let json_data = include_str!("../lists/wordslist.json");

        //               character  pronunciation
        let data : HashMap<String, Vec<String>> = serde_json::from_str(json_data).unwrap(); // XXX: unwrap error detectable immediately in tests due to inclusion of string during build time

        data
    })
}

/// Gets the pronunciation of a Cantonese string from charlist.
pub fn get_ping3jam1_from_charlist(chars:Vec<char>) -> Vec<Vec<String>> {
    let charlist = charlist();
    chars.into_iter().map(|ch| charlist.get(&ch).map(|ps| ps.keys().map(|p| p.clone()).collect()).unwrap_or(vec![])).collect()
}

/// Gets the pronunciation of a Cantonese string from charlist, picking the most common pronunciation.
fn get_ping3jam1_from_charlist_most_common(chars:Vec<char>) -> Vec<String> {
    let charlist = charlist();
    chars.into_iter().map(|ch| charlist.get(&ch).map(|ps| ps.iter().max_by_key(|(_, &count)| count).map(|(p, _)| p.clone()).unwrap_or("".to_string())).unwrap_or("".to_string())).collect()
}

/// Gets the pronunciation of a Cantonese string from wordlist by first segmenting the string.
fn get_ping3jam1_from_wordlist(s: &str) -> Vec<String> {
    let wordlist = wordlist();
    let (_, _, segments) = segmentation::end_user_friendly_segment(s);

    let mut result = vec![];
    for segment in segments {
        // The segmentation result is 1 character, so we should use the most common
        // character instead.
        let word_pronunciation = wordlist.get(&segment).map(|ps| ps.iter().map(|p| p.clone()));
        if segment.chars().count() > 1 || word_pronunciation.is_some() {
            result.push(word_pronunciation.unwrap().next().unwrap());
        } else {
            for cps in get_ping3jam1_from_charlist_most_common(segment.chars().collect()) {
                result.push(cps);
            }
        }
    }
    result
}

#[pyfunction]
/// Gets the pronunciation of a Cantonese string on a best effort basis. Each returned result
/// corresponds to a character in the input string. IF there are multiple pronunciations for a
/// character, multiple results are returned. If a character is not found in the dictionary, an
/// empty list is returned.
pub fn get_ping3jam1(s: &str) -> String {
    get_ping3jam1_from_wordlist(s).join(" ")
}
