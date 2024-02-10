// Sort a vector of strings by the CJK radical

use zilib::unihan;
use std::cmp;

// Compare two strings by their CJK radicals, then by their strokes
// Note that there is a similar comparison function in
// https://www.unicode.org/reports/tr38/#SortingAlgorithm
// But it may not give as much fine grained control (it also assumes the radical is a 8 bit
// integer (which seems to be a bit risky since we already have 240+ radicals in CJKRadicals.txt,
// also the documentation is unclear how to deal with radicals with apostrophes), and seems to make
// assumptions about simplified vs traditional characters)
fn radical_cmp(a: &Vec<char>, b: &Vec<char>) -> std::cmp::Ordering {
    let unihan_data = unihan::unihan_data(); // this can be a slow operation

    // Compare each character in the string
    for idx in 0..cmp::min(a.len(), b.len()) {
        let a_c = a.get(idx).unwrap(); // guaranteed from for loop condition
        let b_c = b.get(idx).unwrap(); // guaranteed from for loop condition

        if a_c == b_c {
            continue;
        }

        let a_rs = unihan_data.get(a_c).map(|uh| uh.get_radical_strokes()).unwrap_or((None, None));
        let b_rs = unihan_data.get(b_c).map(|uh| uh.get_radical_strokes()).unwrap_or((None, None));

        let a_radical = a_rs.0;
        let b_radical = b_rs.0;

        if a_radical.is_none() && b_radical.is_none() {
            if a < b {
                return std::cmp::Ordering::Less;
            } else if a > b {
                return std::cmp::Ordering::Greater;
            } else {
                continue;
            }
        } else if a_radical.is_none() {
            return std::cmp::Ordering::Less;
        } else if b_radical.is_none() {
            return std::cmp::Ordering::Greater;
        }
        assert!(a_radical.is_some() && b_radical.is_some());
        let a_radical = a_radical.unwrap();
        let b_radical = b_radical.unwrap();

        if a_radical == b_radical {
            // compare the strokes of a, b if the radicals are the same
            let a_strokes = a_rs.1.unwrap(); // get_radical_strokes always return Some(strokes) if the radical is Some
            let b_strokes = b_rs.1.unwrap(); // get_radical_strokes always return Some(strokes) if the radical is Some
            if a_strokes < b_strokes {
                return std::cmp::Ordering::Less;
            } else if a_strokes > b_strokes {
                return std::cmp::Ordering::Greater;
            } else {
                // compare the unicode codepoint of the characters if the radicals are the same
                if a_c < b_c {
                    return std::cmp::Ordering::Less;
                } else if a_c > b_c {
                    return std::cmp::Ordering::Greater;
                } else {
                    // this should never happen since at the beginning of the loop we checked if a_c == b_c
                    assert!(false);
                    continue;
                }
            }
        }

        assert!(a_radical != b_radical);
        // compare the strokes of the radicals if the radicals are different
        let a_radical_char = unihan::radical_label_to_chars().get(a_radical).map(|v| v.0).unwrap_or(Some('\0')).unwrap_or('\0');
        let b_radical_char = unihan::radical_label_to_chars().get(b_radical).map(|v| v.0).unwrap_or(Some('\0')).unwrap_or('\0');
        let a_radical_char_stroke = unihan_data.get(&a_radical_char).map(|uh| uh.get_radical_strokes()).unwrap_or((None, None)).1;
        let b_radical_char_stroke = unihan_data.get(&b_radical_char).map(|uh| uh.get_radical_strokes()).unwrap_or((None, None)).1;
        if a_radical_char_stroke < b_radical_char_stroke {
            return std::cmp::Ordering::Less;
        } else if a_radical_char_stroke > b_radical_char_stroke {
            return std::cmp::Ordering::Greater;
        }

        assert!(a_radical_char_stroke == b_radical_char_stroke);
        // compare the unicode codepoint of the radicals if the strokes are the same
        if a_radical_char < b_radical_char {
            return std::cmp::Ordering::Less;
        } else if a_radical_char > b_radical_char {
            return std::cmp::Ordering::Greater;
        } else {
            // this should never happen since we checked that a_radical != b_radical
            assert!(false);
            continue;
        }

    }

    if a.len() < b.len() {
        return std::cmp::Ordering::Less;
    } else if a.len() > b.len() {
        return std::cmp::Ordering::Greater;
    }

    assert!(a == b, "expected a==b, but actually, a={:?}, b={:?}", a, b);

    std::cmp::Ordering::Equal
}

// call radical_cmp and print debug info
fn radical_cmp_debug(a: &Vec<char>, b: &Vec<char>) -> std::cmp::Ordering {
    let unihan_data = unihan::unihan_data(); // this can be a slow operation
    let cmp = radical_cmp(a, b);
    println!("radical_cmp({:?}, {:?}) = {:?}", a.get(0), b.get(0), cmp);
    if let Some(a0) = a.get(0) {
        let a_rs = unihan_data.get(a0).map(|uh| uh.get_radical_strokes()).unwrap_or((None, None));
        println!("  a: {:?} {:?} {:?}", a0, a_rs.0, a_rs.1);
    }
    if let Some(b0) = b.get(0) {
        let b_rs = unihan_data.get(b0).map(|uh| uh.get_radical_strokes()).unwrap_or((None, None));
        println!("  b: {:?} {:?} {:?}", b0, b_rs.0, b_rs.1);
    }
    cmp
}

// TODO: Decide whether we want in-place or out-of-place sorting
fn radical_sort_vc(mut v: Vec<Vec<char>>, debug: bool) -> Vec<Vec<char>> {
    if debug {
        v.sort_by(radical_cmp_debug);
    } else {
        v.sort_by(radical_cmp);
    }
    v
}

fn radical_sort(v: Vec<String>, debug: bool) -> Vec<String> {
    radical_sort_vc(v.iter().map(|s| s.chars().collect()).collect(), debug).iter().map(|v| v.iter().collect()).collect()
}

// Read stdin by lines into a vector and sort them
fn main() {
    let mut v = Vec::new();
    let debug = std::env::args().nth(1).map(|s| s == "--debug").unwrap_or(false);
    loop {
        let mut s = String::new();
        match std::io::stdin().read_line(&mut s) {
            Ok(0) => break,
            Ok(_) => v.push(s.trim().to_string()),
            Err(e) => {
                eprintln!("error: {}", e);
                std::process::exit(1);
            }
        }
    }

    for s in radical_sort(v, debug) {
        println!("{}", s);
    }
}
