use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::sync::OnceLock;

pub enum DataKind {
    CantoneseCharListWithJyutping,
    CantoneseWordListWithJyutping,
    RadicalLabelToChars,
    UnihanData,
    EnglishVariants,
}

/// Set or get the path to the data file. Then, read the file and load the data (if applicable).
pub fn initialize_data(which : DataKind, path: &str) {
    match which {
        DataKind::CantoneseCharListWithJyutping => { _cantonese_charlist_with_jyutping(Some(path), None, false); },
        DataKind::CantoneseWordListWithJyutping => { _cantonese_wordlist_with_jyutping(Some(path), None, false); },
        DataKind::RadicalLabelToChars => { _radical_label_to_chars(Some(path), None, false); },
        DataKind::UnihanData => { _unihan_data(Some(path)); },
        DataKind::EnglishVariants => { _english_variants_data(Some(path), None, false); },
    }
}

pub fn cantonese_charlist_with_jyutping() -> &'static HashMap<char, HashMap<String, u64>> {
#[cfg(feature = "downloaded_data")]
    return _cantonese_charlist_with_jyutping(None, Some(include_str!("../lists/charlist.json")), true);
#[cfg(not(feature = "downloaded_data"))]
    return _cantonese_charlist_with_jyutping(None, None, true);
}

fn _cantonese_charlist_with_jyutping(path : Option<&str>, data_str: Option<&str>, load: bool) -> &'static HashMap<char, HashMap<String, u64>> {
    let path = unsafe {
        static mut PATH: OnceLock<String> = OnceLock::new();
        if let Some(path) = path {
            PATH.take();
            let _ = PATH.set(path.to_string());
        }
        &PATH
    };
    if load {
        static DATA: OnceLock<HashMap<char, HashMap<String, u64>>> = OnceLock::new();
        DATA.get_or_init(|| {
            if let Some(data_str) = data_str {
                serde_json::from_str(data_str).expect("Failed to parse data_str")
            } else {
                let path = path.get().expect("_cantonese_charlist_with_jyutping uninitialized. Please initialize the data path first").as_str();
                // Read from file
                let file = File::open(path).expect(format!("Failed to open file: {:?}", path).as_str());
                let reader = std::io::BufReader::new(file);
                serde_json::from_reader(reader).expect("Failed to read/parse data from file")
            }
        })
    } else {
        static DATA: OnceLock<HashMap<char, HashMap<String, u64>>> = OnceLock::new();
        DATA.get_or_init(|| { HashMap::new() })
    }
}

// A dictionary of (words) => (lists of pronunciations)
pub fn cantonese_wordlist_with_jyutping() -> &'static HashMap<String, Vec<String>> {
#[cfg(feature = "downloaded_data")]
    return _cantonese_wordlist_with_jyutping(None, Some(include_str!("../lists/wordslist.csv")), true);
#[cfg(not(feature = "downloaded_data"))]
    return _cantonese_wordlist_with_jyutping(None, None, true);
}

fn _cantonese_wordlist_with_jyutping(path : Option<&str>, data_str: Option<&str>, load: bool) -> &'static HashMap<String, Vec<String>> {
    let path = unsafe {
        static mut PATH: OnceLock<String> = OnceLock::new();
        if let Some(path) = path {
            PATH.take();
            let _ = PATH.set(path.to_string());
        }
        &PATH
    };

    if load {
        static DATA: OnceLock<HashMap<String, Vec<String>>> = OnceLock::new();
        DATA.get_or_init(|| {
            let mut reader_builder = Box::new(csv::ReaderBuilder::new()); // Apparently this Box fixes the problem of lifetime of the reader when the reader is boxed below.
            reader_builder.has_headers(true).comment(Some(b'#')).flexible(true);

            let records : Box<dyn Iterator<Item = csv::Result<csv::StringRecord>>> = if let Some(data_str) = data_str {
                Box::new(reader_builder.from_reader(data_str.as_bytes()).into_records())
            } else {
                let path = path.get().expect("Please initialize the data path first");
                let file = File::open(path).expect(format!("Failed to open file: {:?}", path).as_str());
                Box::new(reader_builder.from_reader(file).into_records())
            };

            let mut data = HashMap::new();
            for result in records {
                let record = result.unwrap(); // XXX: unwrap error detectable immediately in tests due to inclusion of string during build time
                let pronunciations = record.iter().skip(1).map(|s| s.to_string()).collect();
                data.insert(record[0].to_string(), pronunciations);
            }
            data
        })
    } else {
        static DATA: OnceLock<HashMap<String, Vec<String>>> = OnceLock::new();
        DATA.get_or_init(|| { HashMap::new() })
    }
}

pub fn radical_label_to_chars() -> &'static HashMap<String, (Option<char>, char)> {
#[cfg(feature = "downloaded_data")]
    return _radical_label_to_chars(None, Some(include_str!("../lists/CJKRadicals.txt")), true);
#[cfg(not(feature = "downloaded_data"))]
    return _radical_label_to_chars(None, None, true);
}

/// Map a unihan radical label (r"[0-9]+'{0,2}") to a pair of characters. The first character is
/// the radical character, and the second character is the ideograph. (eg. "9" -> (Some('亻'), '人'))
/// The radical character can be None (hence the Optional result) if it is not included in the
/// Kangxi Radicals block or the CJK Radicals Supplement block.
pub fn _radical_label_to_chars(path : Option<&str>, data_str: Option<&str>, load: bool) -> &'static HashMap<String, (Option<char>, char)> {
    let path = unsafe {
        static mut PATH: OnceLock<String> = OnceLock::new();
        if let Some(path) = path {
            PATH.take();
            let _ = PATH.set(path.to_string());
        }
        &PATH
    };
    if load {
        static RADICAL_LABEL_TO_CHARS : OnceLock<HashMap<String, (Option<char>, char)>> = OnceLock::new();
        RADICAL_LABEL_TO_CHARS.get_or_init(|| {
            let lines : Box<dyn Iterator<Item = std::io::Result<String>>> = if let Some(data_str) = data_str {
                Box::new(data_str.lines().map(|l| Ok(l.to_string())))
            } else {
                let path = path.get().expect("Please initialize the data path first");
                let file = File::open(path).expect(format!("Failed to open file: {:?}", path).as_str());
                Box::new(std::io::BufReader::new(file).lines())
            };

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
            let mut map = HashMap::new();
            for line in lines {
                let line = line.expect(format!("Failed to read line in file: {:?}", path).as_str());
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
    } else {
        static DATA: OnceLock<HashMap<String, (Option<char>, char)>> = OnceLock::new();
        DATA.get_or_init(|| { HashMap::new() })
    }
}

pub fn unihan_data() -> &'static HashMap<char, UnihanData> {
    return _unihan_data(None);
}

pub fn _unihan_data(initial_data_path : Option<&str>) -> &'static HashMap<char, UnihanData> {
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

pub(crate) fn english_variants_data() -> &'static HashMap<String, String> {
    // I tried using https://github.com/SOF3/include-flate and it didn't seem to work in terms
    // of file size reduction. Perhaps the overhead of decompression is too high.
#[cfg(feature = "downloaded_data")]
    return _english_variants_data(None, Some(include_str!("../lists/english_variants.json")), true);
#[cfg(not(feature = "downloaded_data"))]
    return _english_variants_data(None, None, true);
}

fn _english_variants_data(path:Option<&str>, data_str: Option<&str>, load: bool) -> &'static HashMap<String, String> {
    let path = unsafe {
        static mut PATH: OnceLock<String> = OnceLock::new();
        if let Some(path) = path {
            PATH.take();
            let _ = PATH.set(path.to_string());
        }
        &PATH
    };
    if load {
        static DATA: OnceLock<HashMap<String, String>> = OnceLock::new();
        DATA.get_or_init(|| {
            if let Some(data_str) = data_str {
                serde_json::from_str(data_str).expect("Failed to parse data_str")
            } else {
                let path = path.get().expect("Please initialize the data path first");
                let file = File::open(path).expect(format!("Failed to open file: {:?}", path).as_str());
                let reader = std::io::BufReader::new(file);
                serde_json::from_reader(reader).expect("Failed to read/parse data from file")
            }
        })
    } else {
        static DATA: OnceLock<HashMap<String, String>> = OnceLock::new();
        DATA.get_or_init(|| { HashMap::new() })
    }
}
