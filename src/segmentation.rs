use std::collections::HashMap;
use std::collections::HashSet;
use crate::data;

/// Constants
const BT_MATCH: i32 = 0;
const BT_ODD: i32 = -1;
const BIG_VALUE : f32 = f32::INFINITY;

/// Memoized recursive function (i.e. "DP" as we call it)
fn _dp(
    a: usize,
    b: usize,
    s: &Vec<char>,
    d: &HashSet<String>,
    dp: &mut HashMap<(usize, usize), f32>,
    bt: &mut HashMap<(usize, usize), i32>,
) -> f32 {
    // TODO: we can tweak the recurrence function a bit later. But this is the
    // generic one that's useful enough.

    /*" Recurrence function:
    f(a,b) = {
        // yay, good stuff here  -- 1/(b-a) is explained in note #1, position factor is to
        // encourage earlier matches
        1/(b-a) + 2n-a-b/POSITION_FACTOR if s[a:b] in input_set,

        // big cost -- note #2
        10 if b - a == 1

        min( f(a, i) + f(i, b) for i in [a..b] )
    }

    Note #1: Supposedly the cost should be near zero, but we prefer longer
             words than short ones. The best heuristical value probably still
             needs to be tweaked.
    Note #2: Just needs to be significantly larger than the value in #1.
    "*/

    assert!(a < b);

    if let Some(&res) = dp.get(&(a, b)) {
        return res;
    }

    let mut bt_res = 0;
    let mut res = BIG_VALUE;

    // if s[a:b] is in the dictionary
    if d.contains(&s[a..b].iter().collect::<String>()) {
        res = 1.0 / (b - a) as f32;
        bt_res = BT_MATCH;
    } else if b - a == 1 {
        res = 10.0;
        bt_res = BT_ODD;
    } else {
        for i in a + 1..b {
            let v = _dp(a, i, s, d, dp, bt) + _dp(i, b, s, d, dp, bt);
            if v <= res { // <= is used here to encourage partitioning at later rather than earlier, to encourage "sticking" of words earlier
                res = v;
                bt_res = i.try_into().expect("a, b should be small enough to fit into i32.");
            }
        }
    }

    dp.insert((a, b), res);
    bt.insert((a, b), bt_res);
    res
}

/// Backtracking function.
/// Returns a tuple containing: tuples of odd character indices that are not
/// matched, and indices of the segments (segment's start index).
fn _bt(
    a: usize,
    b: usize,
    bt: &HashMap<(usize, usize), i32>,
) -> (Vec<usize>, Vec<usize>) {
    if bt[&(a, b)] == BT_MATCH {
        return (vec![], vec![a]);
    }

    if bt[&(a, b)] == BT_ODD {
        return (vec![a], vec![a]);
    }

    let mid = bt[&(a, b)] as usize;
    let (left_odd, left_seg) = _bt(a, mid, bt);
    let (right_odd, right_seg) = _bt(mid, b, bt);

    let mut odd = left_odd;
    odd.extend(right_odd);

    let mut seg = left_seg;
    seg.extend(right_seg);

    (odd, seg)
}

/// Returns two lists as a pair: The first list contains indices of unmatched odd characters. The
/// second list contains the indices of the segments (the segment's start index).
pub fn segment_with_dictionary(phrase: &str, dictionary: Option<&HashSet<String>>) -> (Vec<usize>, Vec<usize>) {
    let chars : Vec<char> = phrase.chars().collect();
    let n = chars.len();

    if n == 0 {
        return (vec![], vec![]);
    }

    let mut dp: HashMap<(usize, usize), f32> = HashMap::new();
    let mut bt: HashMap<(usize, usize), i32> = HashMap::new();

    if let Some(dictionary) = dictionary {
        _dp(0, n, &chars, dictionary, &mut dp, &mut bt);
    } else {
        let dictionary : HashSet<String> = data::cantonese_wordlist_with_jyutping().keys().cloned().collect();
        _dp(0, n, &chars, &dictionary, &mut dp, &mut bt);
    }
    _bt(0, n, &bt)
}

// TODO: review this
fn sequence_filter(x: &[usize]) -> Vec<usize> {
    let mut ret = Vec::new();
    for i in 1..x.len() {
        if x[i - 1] + 1 == x[i] {
            if ret.is_empty() || *ret.last().unwrap() != x[i - 1] {
                ret.push(x[i - 1]);
            }
            if ret.is_empty() || *ret.last().unwrap() != x[i] {
                ret.push(x[i]);
            }
        }
    }
    ret
}


/// Returns a user-friendly segmentation result for text-based programs. If dictionary is None, we
/// will load an out-of-date Cantonese dictionary from words.hk
pub fn end_user_friendly_segment(s: &str, dictionary: Option<&HashSet<String>>) -> (Vec<char>, Vec<char>, Vec<String>) {
    let (odd_idx, segment_idx) = segment_with_dictionary(s, dictionary);
    let s_chars: Vec<char> = s.chars().collect();

    let bad_words: Vec<char> = sequence_filter(&odd_idx).iter().map(|&idx| s_chars[idx]).collect();
    let odd_words: Vec<char> = odd_idx.iter().map(|&idx| s_chars[idx]).collect();
    let mut segmentation: Vec<String> = Vec::new();

    for window in segment_idx.windows(2) {
        segmentation.push(s_chars[window[0]..window[1]].iter().collect());
    }
    if let Some(&last) = segment_idx.last() {
        segmentation.push(s_chars[last..].iter().collect());
    }

    (bad_words, odd_words, segmentation)
}

