// uses
use std::collections::HashMap;
use std::sync::OnceLock;
use pyo3::pyfunction;

// Thread-safe rust function that reads a file on first call and returns the content, otherwise
// return the cached value
fn english_variants_data() -> &'static HashMap<String, String> {
    static DATA: OnceLock<HashMap<String, String>> = OnceLock::new();
    DATA.get_or_init(|| {
        // I tried using https://github.com/SOF3/include-flate and it didn't seem to work in terms
        // of file size reduction. Perhaps the overhead of decompression is too high.
        let data = include_str!("../lists/english_variants.json");
        serde_json::from_str(data).unwrap_or(HashMap::new())
    })
}

#[pyfunction]
pub fn usa_english(word : &str) -> String {
    english_variants_data().get(word).unwrap_or(&word.to_string()).to_string()
}
