use unicode_categories::UnicodeCategories;

/// Returns the number of characters that are in the "Letter" category in unicode"""
pub fn letter_count(s: &str) -> usize {
    s.chars().filter(|c| c.is_letter()).count() // Count the number of letters
}

/// Removes "other" categories from unicode, which are basically control characters, reserved and
/// private use, etc.
pub fn remove_unicode_other(s: &str) -> String {
    s.chars().filter(|c| *c == '\n' || *c == '\r' || !c.is_other()).collect::<String>()
}

/// Returns whether the code point is a CJK character
pub fn is_cjk_cp(cp: u32) -> bool {
    (0x3400..=0x4DBF).contains(&cp) || (0x4E00..=0x9FFF).contains(&cp) || (0xF900..=0xFAFF).contains(&cp) || (0x20000..=0x2FFFF).contains(&cp)
}

/// Returns whether all the characters of the string are CJK characters
pub fn is_cjk(s: &str) -> bool {
    s.chars().any(|c| !is_cjk_cp(c as u32)) == false
}

/// Returns whether the string has any CJK characters
pub fn has_cjk(s: &str) -> bool {
    s.chars().any(|c| is_cjk_cp(c as u32))
}

/// Returns the number of characters that are latin letters
pub fn is_latin(s: &str) -> bool {
    // XXX: In the python implementation this actually counts the latin characters, but here it
    // counts all letters. There might be some differences in the results but it shouldn't matter.
    s.chars().all(|c| !is_cjk_cp(c as u32) && (c.is_letter_lowercase() || c.is_letter_uppercase()))
}

pub fn is_latin_c(c: char) -> bool {
    !is_cjk_cp(c as u32) && (c.is_letter_lowercase() || c.is_letter_uppercase())
}
/// Returns true if the input looks like a Chinese/Cantonese sentence, assuming it uses
/// "Full-Width" punctuations. Do not assume this is very reliable
pub fn looks_like_a_sentence(s: &str) -> bool {
    s.chars().count() > 8 || "。，；？「」！".chars().any(|punct| s.find(punct).is_some())
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

use std::fs::File;
use std::io::{self, Seek, SeekFrom, Read};
use std::cmp::Ordering;

pub fn binary_search_file(
    path: &str,
    target: &[u8],
    record_delim: u8,
    field_delim: u8,
    start_pos: usize,
    end_pos: Option<usize>,
    mut line_size: usize,
    cmp: fn(&[u8], &[u8]) -> std::cmp::Ordering,
) -> io::Result<Option<usize>> {
    let mut f = File::open(path)?;
    let mut bug_detector = 0;
    let mut first_found_line_idx = None;

    let file_size : usize = match end_pos {
        Some(pos) => pos as usize,
        None => f.seek(SeekFrom::End(0))? as usize,
    };

    const DEBUG_LIMIT : usize = 9999;

    let mut start_pos = start_pos;
    let mut end_pos = match end_pos {
        Some(pos) => pos,
        None => file_size as usize,
    };

    while start_pos < end_pos {
        bug_detector += 1;
        assert!(bug_detector < DEBUG_LIMIT);
        let mid = (start_pos + end_pos) / 2;
        // print!("{} - {} - {}\n", start_pos, mid, end_pos);


        let mut last_try = false;

        // TODO: rename all "line" into "record" for consistency.

        // double the line size until we find a line with the record value
        while !last_try {
            // print!("start:{} mid:{} end:{} line_size:{}\n", start_pos, mid, end_pos, line_size);
            bug_detector += 1;
            assert!(bug_detector < DEBUG_LIMIT);

            last_try = if mid + line_size > file_size {
                line_size = file_size - mid;
                true
            } else {
                false
            };

            let mut line_buf = vec![0u8; line_size];
            f.seek(SeekFrom::Start(mid as u64))?;
            // EOF should not be reached because we checked the size of the file above
            f.read_exact(&mut line_buf)?;


            // Find the start of the line
            let record_delim_idx = line_buf.iter().position(|&x| x == record_delim);
            if record_delim_idx.is_none() {
                line_size *= 2;
                continue;
            }
            let record_delim_idx = record_delim_idx.unwrap();
            let field_start_idx = record_delim_idx + 1;

            // if the record delimiter is found, we can only assume the field value is the whole
            // record
            let field_delim_idx = line_buf[field_start_idx..].iter().position(|&x| x == field_delim || x == record_delim);

            if let Some(field_delim_idx) = field_delim_idx {

                let what = &line_buf[field_start_idx..(field_start_idx + field_delim_idx)];

                let ordering = cmp(what, target);

                // println!("what={:?}, {:?}, target={:?} start_pos={}", String::from_utf8_lossy(what), ordering, String::from_utf8_lossy(target), start_pos);

                last_try = false; // We move the start/end positions, so we're not in the last try anymore
                // We found the record. Yay!
                match ordering {
                    Ordering::Equal => {
                        // We can't break here because there may be multiple records.
                        // Instead we have to find the position of the record that is
                        // *just* smaller than target
                        let proposed_end = mid + record_delim_idx + 1;

                        // only update first_found_line_idx if it's None or if the
                        // field_delim_idx is smaller
                        first_found_line_idx = first_found_line_idx.map(|x : usize| x.min(mid + field_start_idx)).or(Some(mid + field_start_idx));
                        // print!("ORDERING FOUND: ** first_found_line_idx={} mid={} field_start_idx={}\n", first_found_line_idx.unwrap(), mid, field_start_idx);

                        if proposed_end >= end_pos {
                            end_pos = mid;
                        } else {
                            end_pos = proposed_end;
                        }
                    }
                    Ordering::Less => {
                        // Note, in some cases this will cause the whole loop to end. But
                        // we may not have checked whether the line starting with start_pos
                        // also has the target value. So at the end we may need to double
                        // check
                        start_pos = mid + record_delim_idx + 1;
                        // println!("ORDERING LESS: mid:{} record_delim_idx:{:?}", mid, record_delim_idx);
                        // print!("ORDERING LESS: mid:{} next_record_delim_idx:{} new start_pos:{}\n", mid, record_delim_idx, start_pos);
                    }
                    Ordering::Greater => {
                        end_pos = mid;
                    }
                }

                break;
            } else {
                line_size *= 2;
                continue;
            }
        }
        if last_try {
            end_pos = mid;
            // println!("LAST TRY: setting end_pos=mid={}", end_pos);
        }
    }

    // println!("first_found_line_idx={:?} start_pos={} end_pos={}", first_found_line_idx, start_pos, end_pos);

    // Start reading from start_pos to see whether the target is at start_pos
    if first_found_line_idx.is_none() || start_pos < first_found_line_idx.unwrap() {
        let intended_read_size = target.len() + 1;
        if start_pos + intended_read_size <= file_size {
            // Maybe we should seek to start_pos-1 to ensure it starts from a newline
            f.seek(SeekFrom::Start(start_pos as u64))?;
            let mut line_buf = vec![0u8; intended_read_size];
            f.read(&mut line_buf)?;
            if line_buf.starts_with(target) && (line_buf[target.len()] == field_delim || line_buf[target.len()] == record_delim) {
                return Ok(Some(start_pos));
            }
        }
    }

    Ok(first_found_line_idx)
}

