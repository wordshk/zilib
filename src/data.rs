use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::sync::OnceLock;

#[cfg(not(feature = "downloaded_data"))]
pub enum DataKind {
    CharList,
    WordList,
    RadicalLabelToChars,
    UnihanData,
    EnglishVariants,
    Dictionary,
}

/// Set or get the path to the data file. Then, read the file and load the data (if applicable).
/// This function is only available if the downloaded_data feature is not enabled.
#[cfg(not(feature = "downloaded_data"))]
pub fn initialize_data(which : DataKind, path: &str) {
    match which {
        DataKind::CharList => {
            let result = _cantonese_charlist_with_jyutping(Some(path), None);
        },

        _ => {},
    }
}

pub fn cantonese_charlist_with_jyutping() -> &'static HashMap<char, HashMap<String, u64>> {
#[cfg(feature = "downloaded_data")]
    return _cantonese_charlist_with_jyutping(None, Some(include_str!("../lists/charlist.json")));
#[cfg(not(feature = "downloaded_data"))]
    return _cantonese_charlist_with_jyutping(None, None);
}

fn _cantonese_charlist_with_jyutping(path : Option<&str>, data_str: Option<&str>) -> &'static HashMap<char, HashMap<String, u64>> {
    static DATA: OnceLock<HashMap<char, HashMap<String, u64>>> = OnceLock::new();
    DATA.get_or_init(|| {
        if let Some(data_str) = data_str {
            return serde_json::from_str(data_str).expect("Failed to parse data_str");
        } else {
            // Read from file
            let file = File::open(path.expect("Please initialize the data path first")).expect(format!("Failed to open file: {:?}", path).as_str());
            let reader = std::io::BufReader::new(file);
            serde_json::from_reader(reader).expect("Failed to read/parse data from file")
        }
    })
}

// Stub function if downloaded_data feature is not enabled
#[cfg(not(feature = "downloaded_data"))]
pub fn cantonese_wordlist_with_jyutping() -> &'static HashMap<String, Vec<String>> {
    panic!("wordlist called without the downloaded_data feature enabled")
}

// A dictionary of (words) => (lists of pronunciations)
#[cfg(feature = "downloaded_data")]
pub fn cantonese_wordlist_with_jyutping() -> &'static HashMap<String, Vec<String>> {
    static DATA: OnceLock<HashMap<String, Vec<String>>> = OnceLock::new();
    DATA.get_or_init(|| {
        let csv_data = include_str!("../lists/wordslist.csv");
        let mut data = HashMap::new();
        let mut reader = csv::ReaderBuilder::new().has_headers(true).comment(Some(b'#')).flexible(true).from_reader(csv_data.as_bytes());
        for result in reader.records() {
            let record = result.unwrap(); // XXX: unwrap error detectable immediately in tests due to inclusion of string during build time
            let pronunciations = record.iter().skip(1).map(|s| s.to_string()).collect();
            data.insert(record[0].to_string(), pronunciations);
        }
        data
    })
}

// Stub function if downloaded_data feature is not enabled
#[cfg(not(feature = "downloaded_data"))]
pub fn radical_label_to_chars() -> &'static HashMap<String, (Option<char>, char)> {
    panic!("radical_label_to_chars called without the downloaded_data feature enabled")
}

/// Map a unihan radical label (r"[0-9]+'{0,2}") to a pair of characters. The first character is
/// the radical character, and the second character is the ideograph. (eg. "9" -> (Some('亻'), '人'))
/// The radical character can be None (hence the Optional result) if it is not included in the
/// Kangxi Radicals block or the CJK Radicals Supplement block.
#[cfg(feature = "downloaded_data")]
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

// Stub function if downloaded_data feature is not enabled
#[cfg(not(feature = "downloaded_data"))]
pub fn unihan_data(_initial_data_path : Option<&str>) -> &'static HashMap<char, UnihanData> {
    panic!("unihan_data called without the downloaded_data feature enabled")
}

#[cfg(feature = "downloaded_data")]
pub fn unihan_data(initial_data_path : Option<&str>) -> &'static HashMap<char, UnihanData> {
    static UNIHAN_DATA : OnceLock<HashMap<char, UnihanData>> = OnceLock::new();
    UNIHAN_DATA.get_or_init(|| {
        let mut map = HashMap::new();
        let initial_data_path = initial_data_path.expect("initial_data_path not provided for first invokation to unihan_data");
        // FIXME: we need to find out a better way to include the unihan database. For now, we just
        // expect users of API to pass a path for the initialization and hope it works out.
        let data = File::open(initial_data_path).expect(format!("Failed to open Unihan_IRGSources.txt from path {}", initial_data_path).as_str());
        for line in std::io::BufReader::new(data).lines().map(|l| l.expect("Failed to read line")) {
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

#[cfg(feature = "generated_data")]
pub(crate) fn english_variants_data() -> &'static HashMap<String, String> {
    static DATA: OnceLock<HashMap<String, String>> = OnceLock::new();
    DATA.get_or_init(|| {
        // I tried using https://github.com/SOF3/include-flate and it didn't seem to work in terms
        // of file size reduction. Perhaps the overhead of decompression is too high.
        let data = include_str!("../lists/english_variants.json");
        serde_json::from_str(data).unwrap_or(HashMap::new())
    })
}

// Stub function if downloaded_data feature is not enabled
#[cfg(not(feature = "downloaded_data"))]
pub(crate) fn load_dictionary() -> &'static HashSet<String> {
    panic!("load_dictionary called without the downloaded_data feature enabled")
}

// load dictionary using include_str on csv
#[cfg(feature = "downloaded_data")]
pub(crate) fn load_dictionary() -> &'static HashSet<String> {
    static DATA: OnceLock<HashSet<String>> = OnceLock::new();
    DATA.get_or_init(|| {
        let csv_data = include_str!("../lists/wordslist.csv");
        let mut data = HashSet::new();
        let mut reader = csv::ReaderBuilder::new().has_headers(true).comment(Some(b'#')).flexible(true).from_reader(csv_data.as_bytes());
        for result in reader.records() {
            let record = result.unwrap(); // XXX: unwrap error detectable immediately in tests
            data.insert(record[0].to_string());
        }
        data
    })
}
