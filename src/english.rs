use crate::data;

pub fn usa_english(word : &str) -> String {
    data::english_variants_data().get(word).unwrap_or(&word.to_string()).to_string()
}

/* Ported from the python implementation in the links below. Code may look suspiciously like
 * another rust implementation (https://github.com/minhnhdo/rust-stem/blob/master/src/lib.rs) due
 * to convergence. I did reference the code there cross check whether my implementation was sane.
 * Original comments are kept for reference:
    """Porter Stemming Algorithm
    This is the Porter stemming algorithm, ported to Python from the
    version coded up in ANSI C by the author. It may be be regarded
    as canonical, in that it follows the algorithm presented in

    Porter, 1980, An algorithm for suffix stripping, Program, Vol. 14,
    no. 3, pp 130-137,

    only differing from it at the points maked --DEPARTURE-- below.

    See also http://www.tartarus.org/~martin/PorterStemmer

    The algorithm as described in the paper could be exactly replicated
    by adjusting the points of DEPARTURE, but this is barely necessary,
    because (a) the points of DEPARTURE are definitely improvements, and
    (b) no encoding of the Porter stemmer I have seen is anything like
    as exact as this version, even with the points of DEPARTURE!

    Vivake Gupta (v@nano.com)

    Release 1: January 2001

    Further adjustments by Santiago Bruno (bananabruno@gmail.com)
    to allow word input not restricted to one word per line, leading
    to:

    release 2: July 2008
    """
*/

/// Implementation of the Porter Stemming Algorithm in rust
pub struct PorterStemmer {
    b: Vec<u8>, // Use vector of u8 instead of String since English words are ASCII
    k: usize,
    j: usize, // j is a general offset into the string
}

impl PorterStemmer {
    /*"The main part of the stemming algorithm starts here.
    b is a buffer holding a word to be stemmed. The letters are in b[k0],
    b[k0+1] ... ending at b[k]. In fact k0 = 0 in this demo program. k is
    readjusted downwards as the stemming progresses. Zero termination is
    not in fact used in the algorithm.

    Note that only lower case sequences are stemmed. Forcing to lower case
    should be done before stem(...) is called.
    "*/
    fn new(word: &str) -> Self {
        Self {
            // lower case ASCII a-z only, map from chars to u8
            b: word.chars().filter(|c| c.is_ascii_alphabetic()).map(|c| c.to_ascii_lowercase() as u8).collect::<Vec<u8>>(),
            k: 0,
            j: 0,
        }
    }

    fn cons(&self, i: usize) -> bool {
        // cons(i) is TRUE <=> b[i] is a consonant.
        match self.b[i] {
            b'a' | b'e' | b'i' | b'o' | b'u' => false,
            b'y' => {
                if i == 0 {
                    true
                } else {
                    !self.cons(i - 1)
                }
            }
            _ => true,
        }
    }

    fn m(&self) -> usize {
        /*"m() measures the number of consonant sequences between k0 and j.
        if c is a consonant sequence and v a vowel sequence, and <..>
        indicates arbitrary presence,

           <c><v>       gives 0
           <c>vc<v>     gives 1
           <c>vcvc<v>   gives 2
           <c>vcvcvc<v> gives 3
           ....
        "*/
        let mut n: usize = 0;
        let mut i = 0;

        loop {
            if i >= self.j {
                return n
            }
            if !self.cons(i) {
                break
            }
            i = i + 1
        }
        i = i + 1;
        loop {
            loop {
                if i >= self.j {
                    return n
                }
                if self.cons(i) {
                    break
                }
                i = i + 1
            }
            i = i + 1;
            n = n + 1;
            loop {
                if i >= self.j {
                    return n
                }
                if ! self.cons(i) {
                    break
                }
                i = i + 1
            }
            i = i + 1
        }
    }

    fn vowelinstem(&self) -> bool {
        // vowelinstem() is TRUE <=> k0,...j contains a vowel
        for i in 0..self.j {
            if !self.cons(i) {
                return true;
            }
        }
        false
    }

    fn doublec(&self, j: usize) -> bool {
        // doublec(j) is TRUE <=> j,(j-1) contain a double consonant.
        if j < 1 {
            false
        } else if self.b[j] != self.b[j - 1] {
            false
        } else {
            self.cons(j)
        }
    }

    fn cvc(&self, i: usize) -> bool {
        /*"cvc(i) is TRUE <=> i-2,i-1,i has the form consonant - vowel - consonant
        and also if the second c is not w,x or y. this is used when trying to
        restore an e at the end of a short  e.g.

           cav(e), lov(e), hop(e), crim(e), but
           snow, box, tray.
        "*/
        if i < 2 || !self.cons(i) || self.cons(i - 1) || !self.cons(i - 2) {
            false
        } else {
            match self.b[i] {
                b'w' | b'x' | b'y' => false,
                _ => true,
            }
        }
    }

    fn ends(&mut self, s: &str) -> bool {
        // ends(s) is TRUE <=> k0,...k ends with the string s.
        let _s = s.as_bytes();
        let length = _s.len();
        if length > self.k {
            return false;
        }
        if &self.b[self.k - length..self.k] == _s {
            self.j = self.k - length;
            true
        } else {
            false
        }
    }

    fn setto(&mut self, s: &str) {
        // setto(s) sets j,...k to the characters in the string s, readjusting k.
        let _s = s.as_bytes();
        let length = _s.len();
        for i in 0..length {
            self.b[self.j + i] = _s[i];
        }

        self.k = self.j + length;
    }

    fn r(&mut self, s: &str) {
        // r(s) is used further down.
        if self.m() > 0 {
            self.setto(s);
        }
    }

    fn step1ab(&mut self) {
        /* step1ab() gets rid of plurals and -ed or -ing. e.g.

           caresses  ->  caress
           ponies    ->  poni
           ties      ->  ti
           caress    ->  caress
           cats      ->  cat

           feed      ->  feed
           agreed    ->  agree
           disabled  ->  disable

           matting   ->  mat
           mating    ->  mate
           meeting   ->  meet
           milling   ->  mill
           messing   ->  mess

           meetings  ->  meet
        "*/

        if self.b[self.k - 1] == b's' {
            if self.ends("sses") {
                self.k -= 2;
            } else if self.ends("ies") {
                self.setto("i");
            } else if self.b[self.k - 2] != b's' {
                self.k -= 1;
            }
        }
        if self.ends("eed") {
            if self.m() > 0 {
                self.k -= 1;
            }
        } else if (self.ends("ed") || self.ends("ing")) && self.vowelinstem() {
            self.k = self.j;
            if self.ends("at") {
                self.setto("ate");
            } else if self.ends("bl") {
                self.setto("ble");
            } else if self.ends("iz") {
                self.setto("ize");
            } else if self.doublec(self.k - 1) {
                self.k -= 1;
                let ch = self.b[self.k - 1];
                if ch == b'l' || ch == b's' || ch == b'z' {
                    self.k += 1;
                }
            } else if self.m() == 1 && self.cvc(self.k - 1) {
                self.setto("e");
            }
        }
    }

    fn step1c(&mut self) {
        // step1c() turns terminal y to i when there is another vowel in the stem.
        if self.ends("y") && self.vowelinstem() {
            self.b[self.k - 1] = b'i';
        }
    }

    fn step2(&mut self) {
        /* step2() maps double suffices to single ones.
        so -ization ( = -ize plus -ation) maps to -ize etc. note that the
        string before the suffix must give m() > 0.
        */
        if self.k < 2 {
            return;
        }
        match self.b[self.k - 2] {
            b'a' => {
                if self.ends("ational") { self.r("ate"); }
                else if self.ends("tional") { self.r("tion"); }
            },
            b'c' => {
                if self.ends("enci") { self.r("ence"); }
                else if self.ends("anci") { self.r("ance"); }
            },
            b'e' => {
                if self.ends("izer") { self.r("ize"); }
            },
            b'l' => {
                if self.ends("bli") { self.r("ble"); } // DEPARTURE
                // Original algorithm: if self.ends("abli") { self.r("able"); }
                else if self.ends("alli") { self.r("al"); }
                else if self.ends("entli") { self.r("ent"); }
                else if self.ends("eli") { self.r("e"); }
                else if self.ends("ousli") { self.r("ous"); }
            },
            b'o' => {
                if self.ends("ization") { self.r("ize"); }
                else if self.ends("ation") { self.r("ate"); }
                else if self.ends("ator") { self.r("ate"); }
            },
            b's' => {
                if self.ends("alism") { self.r("al"); }
                else if self.ends("iveness") { self.r("ive"); }
                else if self.ends("fulness") { self.r("ful"); }
                else if self.ends("ousness") { self.r("ous"); }
            },
            b't' => {
                if self.ends("aliti") { self.r("al"); }
                else if self.ends("iviti") { self.r("ive"); }
                else if self.ends("biliti") { self.r("ble"); }
            },
            b'g' => {
                if self.ends("logi") { self.r("log"); } // DEPARTURE
                // Original algorithm does not include this line
            },
            _ => (),
        }
    }

    fn step3(&mut self) {
        // step3() dels with -ic-, -full, -ness etc. similar strategy to step2.
        assert!(self.k > 0);
        match self.b[self.k - 1] {
            b'e' => {
                if self.ends("icate") { self.r("ic"); }
                else if self.ends("ative") { self.r(""); }
                else if self.ends("alize") { self.r("al"); }
            },
            b'i' => {
                if self.ends("iciti") { self.r("ic"); }
            },
            b'l' => {
                if self.ends("ical") { self.r("ic"); }
                else if self.ends("ful") { self.r(""); }
            },
            b's' => {
                if self.ends("ness") { self.r(""); }
            },
            _ => (),
        }
    }

    fn step4(&mut self) {
        // step4() takes off -ant, -ence etc., in context <c>vcvc<v>.
        if self.k < 2 {
            return;
        }
        match self.b[self.k - 2] {
            b'a' => {
                if self.ends("al") { }
                else { return; }
            },
            b'c' => {
                if self.ends("ance") { }
                else if self.ends("ence") { }
                else { return; }
            },
            b'e' => {
                if self.ends("er") { }
                else { return; }
            },
            b'i' => {
                if self.ends("ic") { }
                else { return; }
            },
            b'l' => {
                if self.ends("able") { }
                else if self.ends("ible") { }
                else { return; }
            },
            b'n' => {
                if self.ends("ant") { }
                else if self.ends("ement") { }
                else if self.ends("ment") { }
                else if self.ends("ent") { }
                else { return; }
            },
            b'o' => {
                if self.ends("ion") && (self.j > 1 && (self.b[self.j - 1] == b's' || self.b[self.j - 1] == b't')) { }
                else if self.ends("ou") { }
                // takes care of -ous
                else { return; }
            },
            b's' => {
                if self.ends("ism") { }
                else { return; }
            },
            b't' => {
                if self.ends("ate") { }
                else if self.ends("iti") { }
                else { return; }
            },
            b'u' => {
                if self.ends("ous") { }
                else { return; }
            },
            b'v' => {
                if self.ends("ive") { }
                else { return; }
            },
            b'z' => {
                if self.ends("ize") { }
                else { return; }
            },
            _ => return,
        }
        if self.m() > 1 {
            self.k = self.j;
        }
    }

    fn step5(&mut self) {
        //step5() removes a final -e if m() > 1, and changes -ll to -l if m() > 1.
        self.j = self.k;
        if self.b[self.k - 1] == b'e' {
            let a = self.m();
            if a > 1 || (a == 1 && !self.cvc(self.k - 2)) {
                self.k -= 1;
            }
        }
        if self.b[self.k - 1] == b'l' && self.doublec(self.k - 1) && self.m() > 1 {
            self.k -= 1;
        }
    }

    fn stem(&mut self) -> String {
        // Original comments below. We removed i, j, and moved p to new().
        /* In stem(p,i,j), p is a char pointer, and the string to be stemmed
        is from p[i] to p[j] inclusive. Typically i is zero and j is the
        offset to the last character of a string, (p[j+1] == '\0'). The
        stemmer adjusts the characters p[i] ... p[j] and returns the new
        end-point of the string, k. Stemming never increases word length, so
        i <= k <= j. To turn the stemmer into a module, declare 'stem' as
        extern, and delete the remainder of this file.
        "*/
        self.k = self.b.len();

        // With this line, strings of length 1 or 2 don't go through the
        // stemming process, although no mention is made of this in the
        // published algorithm. Remove the line to match the published
        // algorithm.
        if self.k > 2 {
            self.step1ab();
            self.step1c();
            self.step2();
            self.step3();
            self.step4();
            self.step5();
        }

        // unwrap is safe -- we know this will succeed due to the is_ascii_alphabetic check
        String::from_utf8(self.b[..self.k].to_vec()).unwrap()
    }
}

pub fn american_english_stem(w: &str) -> String {
    let mut stemmer = PorterStemmer::new(w);
    stemmer.stem()
}
