use std::cmp;

use crate::data;

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

    let unihan_data = data::unihan_data(); // this can be a slow operation
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
    let a_radical_char = data::radical_label_to_chars().get(a_radical).map(|v| v.0).unwrap_or(Some('\0')).unwrap_or('\0');
    let b_radical_char = data::radical_label_to_chars().get(b_radical).map(|v| v.0).unwrap_or(Some('\0')).unwrap_or('\0');
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

