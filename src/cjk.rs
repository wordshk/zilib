use std::cmp;
use std::collections::HashMap;
use std::sync::OnceLock;

// Compare two strings by their CJK radicals, then by their strokes
// Note that there is a similar comparison function in
// https://www.unicode.org/reports/tr38/#SortingAlgorithm
// But it may not give as much fine grained control (it also assumes the radical is a 8 bit
// integer (which seems to be a bit risky since we already have 240+ radicals in CJKRadicals.txt,
// also the documentation is unclear how to deal with radicals with apostrophes), and seems to make
// assumptions about simplified vs traditional characters)

pub fn radical_char_cmp(a_c: &char, b_c: &char) -> cmp::Ordering {
    if a_c == b_c {
        return cmp::Ordering::Equal;
    }

    let unihan_data = unihan_data(); // this can be a slow operation
    let a_rs = unihan_data.get(a_c).map(|uh| uh.get_radical_strokes()).unwrap_or((None, None));
    let b_rs = unihan_data.get(b_c).map(|uh| uh.get_radical_strokes()).unwrap_or((None, None));

    let a_radical = a_rs.0;
    let b_radical = b_rs.0;

    if a_radical.is_none() && b_radical.is_none() {
        if a_c < b_c {
            return cmp::Ordering::Less;
        } else if a_c > b_c {
            return cmp::Ordering::Greater;
        } else {
            // this should never happen since at the beginning of the loop we checked if a_c == b_c
            assert!(false);
            return cmp::Ordering::Equal;
        }
    } else if a_radical.is_none() {
        return cmp::Ordering::Less;
    } else if b_radical.is_none() {
        return cmp::Ordering::Greater;
    }
    assert!(a_radical.is_some() && b_radical.is_some());
    let a_radical = a_radical.unwrap();
    let b_radical = b_radical.unwrap();

    if a_radical == b_radical {
        // compare the strokes of a, b if the radicals are the same
        let a_strokes = a_rs.1.unwrap(); // get_radical_strokes always return Some(strokes) if the radical is Some
        let b_strokes = b_rs.1.unwrap(); // get_radical_strokes always return Some(strokes) if the radical is Some
        if a_strokes < b_strokes {
            return cmp::Ordering::Less;
        } else if a_strokes > b_strokes {
            return cmp::Ordering::Greater;
        } else {
            // compare the unicode codepoint of the characters if the radicals are the same
            if a_c < b_c {
                return cmp::Ordering::Less;
            } else if a_c > b_c {
                return cmp::Ordering::Greater;
            } else {
                // this should never happen since at the beginning of the loop we checked if a_c == b_c
                assert!(false);
                return cmp::Ordering::Equal;
            }
        }
    }

    assert!(a_radical != b_radical);
    // compare the strokes of the radicals if the radicals are different
    let a_radical_char = radical_label_to_chars().get(a_radical).map(|v| v.0).unwrap_or(Some('\0')).unwrap_or('\0');
    let b_radical_char = radical_label_to_chars().get(b_radical).map(|v| v.0).unwrap_or(Some('\0')).unwrap_or('\0');
    let a_radical_char_stroke = unihan_data.get(&a_radical_char).map(|uh| uh.get_radical_strokes()).unwrap_or((None, None)).1;
    let b_radical_char_stroke = unihan_data.get(&b_radical_char).map(|uh| uh.get_radical_strokes()).unwrap_or((None, None)).1;
    if a_radical_char_stroke < b_radical_char_stroke {
        return cmp::Ordering::Less;
    } else if a_radical_char_stroke > b_radical_char_stroke {
        return cmp::Ordering::Greater;
    }

    assert!(a_radical_char_stroke == b_radical_char_stroke);
    // compare the unicode codepoint of the radicals if the strokes are the same
    if a_radical_char < b_radical_char {
        return cmp::Ordering::Less;
    } else if a_radical_char > b_radical_char {
        return cmp::Ordering::Greater;
    }

    // this should never happen since we checked that a_radical != b_radical
    assert!(false);
    return cmp::Ordering::Equal;
}

pub fn radical_cmp(a: &Vec<char>, b: &Vec<char>) -> cmp::Ordering {
    // Compare each character in the string
    for idx in 0..cmp::min(a.len(), b.len()) {
        let cmp = radical_char_cmp(a.get(idx).unwrap(), b.get(idx).unwrap());
        if cmp != cmp::Ordering::Equal {
            return cmp;
        }

    }

    if a.len() < b.len() {
        return cmp::Ordering::Less;
    } else if a.len() > b.len() {
        return cmp::Ordering::Greater;
    }

    assert!(a == b, "expected a==b, but actually, a={:?}, b={:?}", a, b);

    cmp::Ordering::Equal
}

/// Unihan data for a particular character.
pub struct UnihanData {
    data : HashMap<usize, String>,
}

// Fields from Unihan_IRGSources.txt
static FIELDS : [&str; 15] = [
    "kCompatibilityVariant",
    "kIICore",
    "kIRG_GSource",
    "kIRG_HSource",
    "kIRG_JSource",
    "kIRG_KPSource",
    "kIRG_KSource",
    "kIRG_MSource",
    "kIRG_SSource",
    "kIRG_TSource",
    "kIRG_UKSource",
    "kIRG_USource",
    "kIRG_VSource",
    "kRSUnicode",
    "kTotalStrokes",
];

pub enum UnihanField {
#[allow(non_camel_case_types, dead_code)]
kCompatibilityVariant = 0,
#[allow(non_camel_case_types, dead_code)]
kIICore = 1,
#[allow(non_camel_case_types, dead_code)]
kIRG_GSource = 2,
#[allow(non_camel_case_types, dead_code)]
kIRG_HSource = 3,
#[allow(non_camel_case_types, dead_code)]
kIRG_JSource = 4,
#[allow(non_camel_case_types, dead_code)]
kIRG_KPSource = 5,
#[allow(non_camel_case_types, dead_code)]
kIRG_KSource = 6,
#[allow(non_camel_case_types, dead_code)]
kIRG_MSource = 7,
#[allow(non_camel_case_types, dead_code)]
kIRG_SSource = 8,
#[allow(non_camel_case_types, dead_code)]
kIRG_TSource = 9,
#[allow(non_camel_case_types, dead_code)]
kIRG_UKSource = 10,
#[allow(non_camel_case_types, dead_code)]
kIRG_USource = 11,
#[allow(non_camel_case_types, dead_code)]
kIRG_VSource = 12,
#[allow(non_camel_case_types, dead_code)]
kRSUnicode = 13,
#[allow(non_camel_case_types, dead_code)]
kTotalStrokes = 14,
}

impl UnihanData {
    fn new() -> UnihanData {
        UnihanData {
            data : HashMap::new(),
        }
    }

    fn get(&self, key : UnihanField) -> Option<&str> {
        self.data.get(&(key as usize)).map(|s| s.as_str())
    }

    fn set_s(&mut self, key : &str, value : &str) {
        let ukey = FIELDS.iter().position(|&f| f == key);
        if let Some(ukey) = ukey {
            self.data.insert(ukey, value.to_string());
        }
    }

    /// Strokes can be negative, and there can be more than one radical/stroke pair. For sake of
    /// simplicity, considering that the number of edge cases is small and the affected characters
    /// seem to be rare, we will just return the first radical/stroke pair. See Unihan_IRGSources.txt
    /// for the raw data and decide if you want to handle the edge cases yourself.
    /// The value of radical is a string, and if you want to convert it into a char, you can use
    /// the radical_label_to_chars function.
    pub fn get_radical_strokes(&self) -> (Option<&str>, Option<i32>) {
        let rs = self.get(UnihanField::kRSUnicode);
        if let Some(rs) = rs {
            // Split the string on " " first, and discard the rest of the string after the first
            // space. In vast majority of cases there is no space here
            if let Some(space_split0) = rs.split(" ").next() {
                let mut iter = space_split0.split(".");
                let radical = iter.next();
                let strokes : Option<i32> = iter.next().and_then(|s| s.parse().ok());
                if radical.is_some() && strokes.is_some() {
                    return (radical, strokes);
                }
            }
        }
        (None, None)
    }
}

/// Helper function to convert a hex string to a char. If the string is not a valid hex string, it
/// will panic
fn hex_to_char(s : &str) -> char {
    // XXX: errors should be easily detectable immediately in tests
    // if s starts with 'U+', skip it
    if s.starts_with("U+") {
        hex_to_char(&s[2..])
    } else {
        char::from_u32(u32::from_str_radix(s, 16).expect(format!("Invalid hex string: {}", s).as_str())).expect(format!("Invalid hex string: {}", s).as_str())
    }
}

/// Map a unihan radical label (r"[0-9]+'{0,2}") to a pair of characters. The first character is
/// the radical character, and the second character is the ideograph. (eg. "9" -> (Some('亻'), '人'))
/// The radical character can be None (hence the Optional result) if it is not included in the
/// Kangxi Radicals block or the CJK Radicals Supplement block.
pub fn radical_label_to_chars() -> &'static HashMap<String, (Option<char>, char)> {
    static RADICAL_LABEL_TO_CHARS : OnceLock<HashMap<String, (Option<char>, char)>> = OnceLock::new();
    RADICAL_LABEL_TO_CHARS.get_or_init(|| {
        let mut map = HashMap::new();
        let data = include_str!("../lists/CJKRadicals.txt");
        // From CJKRadicals.txt:
        // There is one line per CJK radical number. Each line contains three
        // fields, separated by a semicolon (';'). The first field is the
        // CJK radical number. The second field is the CJK radical character,
        // which may be empty if the CJK radical character is not included in
        // the Kangxi Radicals block or the CJK Radicals Supplement block.
        // The third field is the CJK unified ideograph.
        //
        // Example line:
        // 9; 2F08; 4EBA
        for line in data.lines() {
            // continue if the line is a comment
            if line.starts_with("#") {
                continue;
            }

            // continue if the line is empty
            if line.trim().len() == 0 {
                continue;
            }

            let mut iter = line.split(";");

            // this is the radical label, although it is said to be a number, it can contain non-numeric characters
            let radical = iter.next().unwrap().trim(); // XXX: unwrap error detectable immediately in tests

            // might not exist (only one special case...)
            let radical_hex = iter.next().expect(format!("Invalid CJKRadicals.txt line: {}", line).as_str()).trim(); // XXX: unwrap error detectable immediately in tests

            // guaranteed to exist. Decode hex to char
            let ideograph = hex_to_char(iter.next().expect(format!("Invalid CJKRadicals.txt line: {}", line).as_str()).trim()); // XXX: unwrap error detectable immediately in tests

            if radical_hex.len() > 0 {
                map.insert(radical.to_string(), (Some(hex_to_char(radical_hex)), ideograph));
            } else {
                map.insert(radical.to_string(), (None, ideograph));
            }
        }

        map
    })
}

pub fn unihan_data() -> &'static HashMap<char, UnihanData> {
    static UNIHAN_DATA : OnceLock<HashMap<char, UnihanData>> = OnceLock::new();
    UNIHAN_DATA.get_or_init(|| {
        let mut map = HashMap::new();
        let data = include_str!("../lists/Unihan_IRGSources.txt");
        for line in data.lines() {
            if line.starts_with("#") {
                continue;
            }
            if line.len() == 0 {
                continue;
            }
            let mut iter = line.split("\t");
            let codepoint = hex_to_char(iter.next().unwrap().trim()); // XXX: unwrap error detectable immediately in tests
            let field = iter.next().unwrap().trim(); // XXX: unwrap error detectable immediately in tests
            let value = iter.next().unwrap().trim(); // XXX: unwrap error detectable immediately in tests
            let entry = map.entry(codepoint).or_insert(UnihanData::new());
            entry.set_s(field, value);
        }
        map
    })
}