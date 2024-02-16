/*!
Match (CJK) characters to Cantonese pronunciations in a way that makes it easy
to generate ruby text. Uses a variant of Longest Common Subsequence to generate
the best match.
*/

use crate::common;
use crate::data;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

// A dictionary of (characters) => (lists of pronunciations stripped of tones)
// This is useful when there are tonal changes (which often occurs in practice) that are not recognized by the char list.
fn cantonese_charlist_half() -> &'static HashMap<char, Vec<String>> {
    static DATA: OnceLock<HashMap<char, Vec<String>>> = OnceLock::new();
    DATA.get_or_init(|| {
        let charlist = data::cantonese_charlist_with_jyutping();
        charlist.iter()
            .map(|(ch, pd)| (*ch, pd.keys().map(|p| p.trim_end_matches(&['1', '2', '3', '4', '5', '6']).to_string()).collect()))
            .collect()
    })
}

/// Maximum items for either text or pronunciation input. This is required because the algorithm is
/// O(n^2) and can be slow/memory intensive for long inputs.
pub const RUBY_MATCH_MAX : u32 = 300;

const FULL_MATCH_SCORE : i32 = 1000;
const HALF_MATCH_SCORE : i32 = 500;

// Some token value to force the lcs to prefer one unmatched situation over
// another. Not totally sure what value this should contain to be correct.
const EPSILON_SCORE : i32 = 1;

// Score to discourage unmatched pronunciations from "corrupting" well matched txt-pr pairs.
// XXX: No idea what this is about, might actually be useless.
const CORRUPTION_SCORE: i32 = 10;

fn ruby_text_ignore() -> &'static HashSet<char> {
    static DATA: OnceLock<HashSet<char>> = OnceLock::new();
    DATA.get_or_init(|| {
        let ascii_ignore : HashSet<char> = (32u8..128u8)
            .filter_map(|x| {
                let c = x as char;
                if !common::is_latin_c(c) && !(0x30..0x40).contains(&x) {
                    Some(c)
                } else {
                    None
                }
            })
            .collect();

        // add punctuation
        let punctuation = "、。，；．：？！…‥﹐﹔﹕﹖﹗—「」〈〉︿﹀《》【】『』（）";
        punctuation.chars().chain(ascii_ignore).collect()
    })
}

fn _is_alphanumeric(c: char) -> bool {
    let cc = c as u32;
    (0x30..0x40).contains(&cc) || (0xFF10..0xFF20).contains(&cc) || common::is_latin_c(c)
}

fn _flush_helper(buf: &Vec<char>, res: &mut Vec<String>) {
    if buf.is_empty() {
        return;
    }

    // If the token is a "link", then we should just consume the whole part.
    if buf[0] == '#' {
        res.push(buf.iter().collect());
        return;
    }

    // Find the last latin char, so that the buffer will contain
    // contiguous latin+punctuation, except at the beginning and
    // end, the latter which should be considered part of the plain
    // txt
    let idx = buf.iter().rposition(|&c| _is_alphanumeric(c));

    if let Some(idx) = idx {
        res.push(buf[..=idx].iter().collect());
        res.extend(buf[idx + 1..].iter().map(|&c| c.to_string()));
    } else {
        res.push(buf.iter().collect());
    }
}

/// Try to tokenize the input text -- cjk characters are individual tokens, latin characters should
/// be joined, and other characters are individual tokens unless they join up with latin
/// characters. The idea is that each token should ideally match one pronunciation.
fn _text_tokenizer(txt: &str) -> Vec<String> {
    let mut res = Vec::new();
    let mut buf = Vec::new();
    let mut state: i32 = 0; // 0: normal, 1: in a link
    for c in txt.chars() {
        match c {
            '#' => {
                _flush_helper(&buf, &mut res);
                buf.clear();
                buf.push(c);
                state = 1;
                // continue;
            },
            _ if state == 1 && (common::is_cjk_cp(c as u32) || _is_alphanumeric(c)) => {
                buf.push(c);
                // continue;
            },
            _ => {
                if state == 1 {
                    _flush_helper(&buf, &mut res);
                    buf.clear();
                }
                state = 0;

                if common::is_cjk_cp(c as u32) {
                    _flush_helper(&buf, &mut res);
                    buf.clear();
                    res.push(c.to_string());
                } else if _is_alphanumeric(c) {
                    buf.push(c);
                } else {
                    if !buf.is_empty() {
                        buf.push(c);
                    } else {
                        res.push(c.to_string());
                    }
                }
            }
        }
    }
    _flush_helper(&buf, &mut res); // flush everything before we leave
    res
}


/// Ruby match. Returns a zipped (token, pronunciation) list of the structure of the match.
pub fn ruby_match_zipped(txt: &str, pronunciation: &str) -> Vec<(String, String)> {
    let mut rm = RubyMatch::new(txt, pronunciation);
    rm.run();
    rm.structure()
}

/// Ruby match. Returns a plain text representation. Useful for unit testing (since the results are easier to understand)
pub fn ruby_match_plain(txt: &str, pronunciation: &str) -> String {
    let mut rm = RubyMatch::new(txt, pronunciation);
    rm.run();
    rm.plain_text()
}


pub struct RubyMatch {
    btd: HashMap<(i32, i32, bool), (i32, i32, bool)>,
    dp: HashMap<(i32, i32, bool), i32>,
    txt: Vec<String>,
    pronunciation: Vec<String>,
    ltxt: i32,
    lpr: i32,
    ruby: HashMap<i32, Vec<i32>>,
    lcs_result: i32,
    error: Option<String>,
}

impl RubyMatch {
    fn new(txt: &str, pronunciation: &str) -> Self {
        let txt_tokens = _text_tokenizer(txt);
        let pronunciation_tokens: Vec<String> = pronunciation.split_whitespace() // divergence: original code just splits the string by ' '
            .map(|pr| pr.trim_matches(|c| ruby_text_ignore().contains(&c)).to_string()).collect();

        let ltxt = txt_tokens.len() as i32;
        let lpr = pronunciation_tokens.len() as i32;

        if ltxt > RUBY_MATCH_MAX as i32 || lpr > RUBY_MATCH_MAX as i32 {
            return RubyMatch {
                btd: HashMap::new(),
                dp: HashMap::new(),
                txt: Vec::new(),
                pronunciation: Vec::new(),
                ltxt,
                lpr,
                ruby: HashMap::new(),
                lcs_result: -1,
                error: Some("Input text too long".to_string()),
            };
        }

        RubyMatch {
            btd: HashMap::new(),
            dp: HashMap::new(),
            txt: txt_tokens,
            pronunciation: pronunciation_tokens,
            ltxt,
            lpr,
            ruby: HashMap::new(),
            lcs_result: -1,
            error: None,
        }
    }

    fn _lcs(&mut self, arg: (i32, i32, bool)) -> i32 {
        let (t_i, p_j, already_mismatch) = arg;
        if t_i == -1 && p_j == -1 {
            // Seems like we should prefer this case over the below (only either
            // t_i or p_j < 0) but from the tests it doesn't seem to matter?
            return 0;
        }

        if t_i == -1 || p_j == -1 {
            return 0;
        }

        if let Some(&ret) = self.dp.get(&arg) {
            return ret;
        }

        assert!(t_i >= 0 && p_j >= 0);

        // XXX: these clone are unnecessary, but we haven't figured out how to avoid it yet since
        // rust doesn't allow us to make parts of the struct immutable. Luckily, the performance
        // impact should still be small because the length of the strings are small.
        let te = self.txt[t_i as usize].clone();
        let pe = self.pronunciation[p_j as usize].clone();

        let te0 = if te.starts_with('#') && te.len() > 1 {
            // Special case to try match some single char '#' links
            te[1..].chars()
        } else {
            te.chars()
        }.next().expect("te0 is guaranteed to exist due to tokenizer implementation");

        let (max_arg, max_v) =
            if ruby_text_ignore().contains(&te0) {
                // Case: this token is ignored, just continue to next token
                let the_arg = (t_i - 1, p_j, false);
                (the_arg, self._lcs(the_arg)) // returned to max_arg, max_v
            } else if data::cantonese_charlist_with_jyutping().get(&te0).map_or(false, |ps| ps.contains_key(&pe)) {
                // Case: match! (somewhat greedily since we could also have matched in the half
                // part...) We consume both the token and the pronunciation, and add a
                // FULL_MATCH_SCORE with CORRUPTION_SCORE adjustment
                let the_arg = (t_i - 1, p_j - 1, false);
                (the_arg, self._lcs(the_arg) + FULL_MATCH_SCORE + if !already_mismatch { CORRUPTION_SCORE } else { 0 }) // returned to max_arg, max_v
            } else {
                if cantonese_charlist_half().get(&te0).map_or(false, |ps| ps.contains(&pe.trim_end_matches(&['1', '2', '3', '4', '5', '6']).to_string())) {
                    let the_arg = (t_i - 1, p_j - 1, false);
                    let v = self._lcs(the_arg) + HALF_MATCH_SCORE + if !already_mismatch { CORRUPTION_SCORE } else { 0 };
                    (the_arg, v) // returned to max_arg, max_v
                } else {
                    // At this point already_mismatch does not matter, so let's see
                    // if we already have a prior result from our negation.
                    let negarg = (t_i, p_j, !already_mismatch);
                    if self.dp.contains_key(&negarg) {
                        (self.btd[&negarg], self.dp[&negarg])
                    } else {
                        let mut the_arg = (t_i, p_j-1, true);
                        let mut v = self._lcs(the_arg);

                        let targ = (t_i-1, p_j, true);
                        let tv = self._lcs(targ);

                        if tv > v {
                            the_arg = targ;
                            v = tv;
                        }

                        // if te.len() > 1 || common::is_cjk_cp(te0 as u32) { // this if condition is probably not needed?
                        let targ = (t_i-1, p_j-1, false);
                        let tv = self._lcs(targ) + EPSILON_SCORE;
                        if tv > v {
                            the_arg = targ;
                            v = tv;
                        }
                        // }

                        (the_arg, v)
                    }
                }
            };

        self.btd.insert(arg, max_arg);
        self.dp.insert(arg, max_v);
        max_v
    }


    fn _bt(&mut self, in_arg: (i32, i32, bool)) {
        let mut arg = in_arg;
        while let Some(&next_arg) = self.btd.get(&arg) {
            if arg.1 != next_arg.1 {
                self.ruby.entry(arg.0).or_insert_with(Vec::new).insert(0, arg.1);
            }
            arg = next_arg;
        }
        if arg.1 != -1 {
            for i in (0..=arg.1).rev() {
                self.ruby.entry(0).or_insert_with(Vec::new).insert(0, i);
            }
        }
    }

    pub fn run(&mut self) -> Option<&HashMap<i32, Vec<i32>>> {
        if self.error.is_some() {
            return None;
        }
        if self.ruby.is_empty() {
            self.lcs_result = self._lcs((self.ltxt - 1, self.lpr - 1, false));
            self._bt((self.ltxt - 1, self.lpr - 1, false));
        }
        Some(&self.ruby)
    }

    pub fn plain_text(&mut self) -> String {
        // Implementation of the plain_text method
        let mut output = Vec::new();
        for (idx, item) in self.txt.iter().enumerate() {
            if idx > 0 {
                output.push(" ".to_string());
            }

            output.push(item.to_string());

            if let Some(ruby) = self.ruby.get(&(idx as i32)) {
                for pidx in ruby {
                    output.push(self.pronunciation[*pidx as usize].clone());
                }
            }
        }
        output.join("")
    }

    pub fn structure(&mut self) -> Vec<(String, String)> {
        // Implementation of the structure method
        let mut output = Vec::new();
        for (idx, item) in self.txt.iter().enumerate() {
            let ruby = self.ruby.get(&(idx as i32));
            if item.trim().is_empty() && ruby.is_none() {
                continue;
            }

            if let Some(ruby) = ruby {
                output.push((item.to_string(), ruby.iter().map(|pidx| self.pronunciation[*pidx as usize].clone()).collect::<Vec<String>>().join(" ")));
            } else {
                output.push((item.to_string(), "".to_string()));
            }
        }
        output
    }

    pub fn gen_html(&mut self) -> String {
        let mut output = Vec::new();
        output.push("<ruby>".to_string());
        for (idx, item) in self.txt.iter().enumerate() {
            let ruby = self.ruby.get(&(idx as i32));
            if item.trim().is_empty() && ruby.is_none() {
                continue;
            }

            output.push("<rb>".to_string());

            output.push(item.to_string());
            output.push("</rb>".to_string());

            output.push("<rt>".to_string());
            if let Some(ruby) = ruby {
                output.push(ruby.iter().map(|pidx| self.pronunciation[*pidx as usize].clone()).collect::<Vec<String>>().join(" "));
            }
            output.push("</rt>".to_string());
        }
        output.push("</ruby>".to_string());
        output.join("")
    }
}
