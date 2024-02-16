/// This program contains code to generate lists and data required for the library. It may be based
/// on external data sources.

use bzip2::read::BzDecoder;
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write, Seek};
use zilib::{cjk, common, data};

/*
# More details about file format of varcon can be found in:
# - http://wordlist.aspell.net/scowl-readme/
# - https://github.com/en-wl/wordlist/blob/master/varcon/README
*/

fn generate_english_variants(out_filename : &str) -> io::Result<()> {
    let filename = "./lists/varcon.txt.bz2";

    // Read the file as bzip2 instead of plain text. The file is in ISO-8859-1 encoding (also known
    // as Windows-1252)
    let decoder = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(BzDecoder::new(File::open(filename)?));
    let reader = BufReader::new(decoder);

    let re_cluster_header = Regex::new(r"^# ([^ ]+) .*\(level ([0-9]+)\)").unwrap();

    let mut reverse_map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut current_cluster: Option<String> = None;

    let mut line_num = 0;
    for raw_line in reader.lines() {
        line_num += 1;
        // If the line cannot be read, give an empty string
        let mut line = raw_line.unwrap_or_else( |err| {
            eprintln!("Error reading line {}, due to {} ", line_num, err);
            "".to_string() });

        line = line.to_lowercase();
        line = line.trim().to_string();

        if let Some(caps) = re_cluster_header.captures(&line) {
            current_cluster = Some(caps.get(1).unwrap().as_str().to_string());
            let level: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
            if level > 50 {
                current_cluster = None;
            }
            continue;
        }

        if current_cluster.is_none() {
            continue;
        }

        line = line.split('#').next().unwrap().trim().to_string();
        line = line.split('|').next().unwrap().trim().to_string();

        if line.is_empty() {
            continue;
        }

        let cluster = current_cluster.as_ref().unwrap();
        let vset = reverse_map.entry(cluster.to_string()).or_insert_with(HashSet::new);
        if vset.is_empty() {
            vset.insert(cluster.to_string());
        }

        for entry in line.split('/') {
            let parts: Vec<&str> = entry.split(':').collect();
            let var = parts[1].trim().to_string();

            if is_suffix(&var, &vset) {
                continue;
            }

            vset.insert(var);
        }
    }

    // Writing the map as json file to the output file
    let mut out_file = File::create(out_filename)?;

    // newbie note: the double braces are an escape sequence in rust for the curly braces
    writeln!(out_file, "{{")?;

    // count the expected number of items
    let mut total = 0;
    for (_, s) in reverse_map.iter() {
        for _ in s.iter() {
            total += 1;
        }
    }

    let mut count = 0;
    for (k, s) in sorted_by(reverse_map.iter(), |a, b| a.0.cmp(b.0)).iter()  {
        for v in sorted(s.iter()).iter() {
            count += 1;
            if v != k {
                write!(out_file, r#""{}":"{}""#, v, k)?;
                // Check whether the last element is reached
                if count + 1 < total {
                    writeln!(out_file, ",")?;
                } else {
                    writeln!(out_file, "")?;
                }
            }
        }
    }
    writeln!(out_file, "}}")?;

    Ok(())
}

fn wordshk_character_set() -> HashSet<char> {
    let mut resultset = HashSet::new();
    let set_files = vec!["./lists/edb_charlist.txt", "./lists/wordshk_charlist.txt"];
    for f in set_files {
        let file = File::open(f).expect(format!("File {} not found", f).as_str());
        for line in BufReader::new(file).lines() {
            let line = line.expect(format!("Error reading file {}", f).as_str());
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line.starts_with('#') {
                continue;
            }
            if line.starts_with('!') {
                for c in line.chars().skip(1) {
                    if common::is_cjk_cp(c as u32) {
                        resultset.remove(&c);
                    }
                }
            } else {
                for c in line.chars() {
                    if common::is_cjk_cp(c as u32) {
                        resultset.insert(c);
                    }
                }
            }
        }
    }
    resultset
}

fn generate_wordshk_charset(out_filename : &str) -> io::Result<()> {
    let canonical_set = wordshk_character_set();
    let unihan_data = data::unihan_data(); // this can be a slow operation
    let mut out_file = File::create(out_filename)?;
    writeln!(out_file, "[")?;

    let mut last_radical_label : Option<&str> = None;
    for c in sorted_by(canonical_set.iter(), |a, b| cjk::radical_char_cmp(a, b)).iter() {
        let (this_radical_label, _) = unihan_data.get(c).map(|uh| uh.get_radical_strokes()).unwrap_or((None, None));
        if this_radical_label != last_radical_label {
            // println!();
            write!(out_file, "\n")?;
            last_radical_label = this_radical_label;
        }
        write!(out_file, " \"{}\",", c)?;
    }

    // backtrack one character to remove the trailing comma
    out_file.seek(io::SeekFrom::End(-1))?;
    writeln!(out_file, "")?;
    writeln!(out_file, "]")?;
    Ok(())
}

// Function that takes an iterator and returns another vector that is sorted
fn sorted<T: Ord, I: Iterator<Item=T>>(iter: I) -> Vec<T> {
    let mut v: Vec<_> = iter.collect();
    v.sort();
    v
}

// Function that takes an iterator and returns another vector that is sorted by a custom comparator
fn sorted_by<T, F, I>(iter: I, cmp: F) -> Vec<T>
where
    F: FnMut(&T, &T) -> Ordering,
    I: Iterator<Item = T>,
{
    let mut v: Vec<_> = iter.collect();
    v.sort_by(cmp);
    v
}

fn is_suffix(s: &str, ref_set: &HashSet<String>) -> bool {
    (s.ends_with("'s") || s.ends_with("ed")) && ref_set.contains(&s[..s.len()-2].to_string())
    || (s.ends_with('s') || s.ends_with('d')) && ref_set.contains(&s[..s.len()-1].to_string())
}

fn usage() {
    println!("Usage: zigen <command> <args>");
    println!("Commands:");
    println!("  generate_english_variants <output_filename> - Generate a map of English variants from the varcon file");
}


// Recursive helper function
fn recur(w: &HashMap<char, char>, k: char, st: &mut Vec<char>, charset: &HashSet<char>) -> Option<char> {
    if st.contains(&k) {
        return None;
    }
    st.push(k);
    if let Some(next_k) = w.get(&k) {
        if w.contains_key(next_k) {
            recur(w, *next_k, st, charset)
        } else {
            Some(next_k.clone())
        }
    } else {
        Some(k)
    }
}

// Main function to deloop the map
fn deloop(rawmap: HashMap<char, char>, charset: HashSet<char>) -> HashMap<char, char> {
    let mut deloopedmap: HashMap<char, char> = HashMap::new();

    for (fr, _) in rawmap.iter() {
        let mut looped = Vec::new();
        if let Some(res) = recur(&rawmap, *fr, &mut looped, &charset) {
            deloopedmap.insert(fr.clone(), res);
        } else {
            let canonical: Vec<char> = looped.iter().filter(|x| charset.contains(*x)).cloned().collect();
            if canonical.len() == 1 {
                for x in looped.iter() {
                    if x != &canonical[0] {
                        deloopedmap.insert(x.clone(), canonical[0].clone());
                    }
                }
            }
        }
    }

    deloopedmap
}

// 'a is the lifetime of the resulting iterator
fn line_generator(path: &str, splitter: char) -> impl Iterator<Item = io::Result<Vec<String>>> {
    let file = File::open(path).expect(format!("File not found {}", path).as_str());
    let reader = io::BufReader::new(file).lines();

    reader.filter_map(move |line| {
        let line = line.ok()?;
        let line = line.trim().to_string();
        if line.is_empty() || line.starts_with('#') {
            return None;
        }

        // get the first char from every splitted segment
        let results: Vec<String> = line.split(splitter).map(|s| s.trim().to_string()).collect();
        assert!(results.len() > 1, "Error: line {} has less than 2 segments", line);
        Some(Ok(results))
    })
}

fn wordshk_variant_map() -> Result<HashMap<char, char>, Box<dyn std::error::Error>> {
    let mut rawmap: HashMap<char, char> = HashMap::new();
    let charset = wordshk_character_set();

    let tw_path = "lists/TWVariants.txt";
    let hk_path = "lists/HKVariants.txt";
    let hf_path = "lists/hfhchan-kVariants.txt";
    let wordshk_path = "lists/wordshk_variantmap.txt";

    // TWVariants.txt processing
    for splits in line_generator(&tw_path, '\t') {
        let splits = splits?;
        let to = splits[0].chars().next().unwrap();
        let fr = splits[1].chars().next().unwrap();
        rawmap.insert(fr, to);
    }

    // HKVariants.txt processing
    for splits in line_generator(&hk_path, '\t') {
        let splits = splits?;
        let fr = splits[0].chars().next().unwrap();
        let to = splits[1].chars().next().unwrap();
        // Original comment from python code: Sometimes there are multiple values here, and we just
        // need to take the first.
        rawmap.insert(fr, to);
    }

    // hfhchan-kVariants.txt processing
    for splits in line_generator(&hf_path, '\t') {
        let splits = splits?;
        let fr = &splits[0];
        let annotate = &splits[1];
        let to = &splits[2];

        if !annotate.contains("simp") {
            let fr0 = fr.chars().next().unwrap();
            let to0 = to.chars().next().unwrap();
            if charset.contains(&to0) && !charset.contains(&fr0) {
                rawmap.insert(fr0, to0);
            }
            if charset.contains(&fr0) && !charset.contains(&to0) {
                rawmap.insert(to0, fr0);
            }
        }
    }

    // wordshk_variantmap.txt processing
    let mut overridemap: HashMap<char, char> = HashMap::new();
    for splits in line_generator(&wordshk_path, '\t') {
        let splits = splits?;
        let fr = &splits[0];
        let to = &splits[1];
        let fr0 = fr.chars().next().unwrap();
        let to0 = to.chars().next().unwrap();
        if to.contains("#!!") {
            overridemap.insert(fr0, to0);
        } else {
            if let Some(existing) = rawmap.get(&fr0) {
                if *existing == to0 {
                    println!("Warning: {} => {} already defined", fr, to);
                } else {
                    println!("Warning: {} => {} specified but already have {} => {}", fr, to, fr, existing);
                }
            }
        }
        rawmap.insert(fr0, to0);
    }

    // Deloop the map

    // Override with custom map
    let mut deloopedmap = deloop(rawmap, charset.clone());
    for (fr, to) in overridemap {
        deloopedmap.insert(fr, to);
    }

    // Check the results
    let additional_charset: HashSet<char> = "捝敚棁榅涚煴緼腽蒀輼鰛".chars().collect();
    let invalids: Vec<&char> = deloopedmap.values().filter(|&c| !charset.contains(c) && !additional_charset.contains(&c)).collect();
    if !invalids.is_empty() {
        let invalid_chars: String = invalids.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", ");
        return Err(format!("Found characters '{}' which are not in HK standard charset!", invalid_chars).into());
    }

    Ok(deloopedmap)
}

// Custom comparator for sorting the map: first by "to" character, then by "from" character
fn radical_char_cmp_for_map_item(a: &(&char, &char), b: &(&char, &char)) -> Ordering {
    let (a_from, a_to) = a;
    let (b_from, b_to) = b;

    if a_to == b_to {
        cjk::radical_char_cmp(&a_from, &b_from)
    } else {
        cjk::radical_char_cmp(&a_to, &b_to)
    }
}

fn generate_wordshk_variantmap(out_filename : &str) -> io::Result<()> {
    let map = wordshk_variant_map().expect("Error generating wordshk variant map");
    let unihan_data = data::unihan_data(); // this can be a slow operation
    let mut out_file = File::create(out_filename)?;
    write!(out_file, "{{")?;

    let mut count = 0;
    let total = map.len();
    let mut last_radical_label : Option<&str> = None;
    for (k, v) in sorted_by(map.iter(), radical_char_cmp_for_map_item).iter() {
        // if radical changes, print a newline
        let (this_radical_label, _) = unihan_data.get(v).map(|uh| uh.get_radical_strokes()).unwrap_or((None, None));
        if this_radical_label != last_radical_label {
            write!(out_file, "\n")?;
            write!(out_file, " ")?;
            last_radical_label = this_radical_label;
        }
        count += 1;
        write!(out_file, r#""{}":"{}""#, k, v)?;
        if count < total {
            write!(out_file, ",")?;
        }
    }
    write!(out_file, "\n")?;
    write!(out_file, "}}\n")?;

    Ok(())
}

fn generate_wordshk_autoconvert(out_filename : &str) -> io::Result<()> {
    let mut map = HashMap::new();
    let safemap_path = "lists/wordshk_autoconvert.txt";
    for splits in line_generator(safemap_path, '\t') {
        let splits = splits.unwrap();
        let fr = splits[0].chars().next().unwrap();
        let to = splits[1].chars().next().unwrap();
        map.insert(fr, to);
    }
    let unihan_data = data::unihan_data(); // this can be a slow operation
    let mut out_file = File::create(out_filename)?;
    write!(out_file, "{{")?;

    let mut count = 0;
    let total = map.len();
    let mut last_radical_label : Option<&str> = None;
    for (k, v) in sorted_by(map.iter(), |a, b| cjk::radical_char_cmp(a.1, b.1)).iter() {
        // if radical changes, print a newline
        let (this_radical_label, _) = unihan_data.get(v).map(|uh| uh.get_radical_strokes()).unwrap_or((None, None));
        if this_radical_label != last_radical_label {
            write!(out_file, "\n")?;
            write!(out_file, " ")?;
            last_radical_label = this_radical_label;
        }

        count += 1;
        write!(out_file, r#""{}":"{}""#, k, v)?;
        if count < total {
            write!(out_file, ",")?;
        }

    }
    write!(out_file, "\n")?;
    write!(out_file, "}}\n")?;

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    data::initialize_data(data::DataKind::UnihanData, "lists/Unihan_IRGSources.txt"); // initialization. this can be a slow operation
    match (args.get(1).map(String::as_str), args.get(2)) {
        (Some("generate_english_variants"), Some(out_filename) ) => generate_english_variants(out_filename),
        (Some("generate_wordshk_charset"), Some(out_filename)) => generate_wordshk_charset(out_filename),
        (Some("generate_wordshk_variantmap"), Some(out_filename)) => generate_wordshk_variantmap(out_filename),
        (Some("generate_wordshk_autoconvert"), Some(out_filename)) => generate_wordshk_autoconvert(out_filename),
        _ => {
            usage();
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid usage"))
        }
    }
}
