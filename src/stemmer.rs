use pyo3::pyfunction;

pub struct PorterStemmer {
    b: String, // buffer for word to be stemmed
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
    fn new() -> Self {
        Self {
            b: String::new(),
            k: 0,
            j: 0,
        }
    }

    fn cons(&self, i: usize) -> bool {
        // cons(i) is TRUE <=> b[i] is a consonant.
        match self.b.chars().nth(i).unwrap() { // FIXME: unwrap
            'a' | 'e' | 'i' | 'o' | 'u' => false,
            'y' => {
                if i == 0 {
                    true
                } else {
                    !self.cons(i - 1)
                }
            }
            _ => true,
        }
    }

    fn m(&self) -> i32 {
        /*"m() measures the number of consonant sequences between k0 and j.
        if c is a consonant sequence and v a vowel sequence, and <..>
        indicates arbitrary presence,

           <c><v>       gives 0
           <c>vc<v>     gives 1
           <c>vcvc<v>   gives 2
           <c>vcvcvc<v> gives 3
           ....
        "*/
        let mut n: i32 = 0; // FIXME: need i32?
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
        } else if self.b.chars().nth(j) != self.b.chars().nth(j - 1) {
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
            match self.b.chars().nth(i).unwrap() { // FIXME: unwrap
                'w' | 'x' | 'y' => false,
                _ => true,
            }
        }
    }

    fn ends(&mut self, s: &str) -> bool {
        // ends(s) is TRUE <=> k0,...k ends with the string s.
        let length = s.len();
        if length > self.k {
            return false;
        }
        if self.b[self.k - length..self.k] == *s {
            self.j = self.k - length;
            true
        } else {
            false
        }
    }

    fn setto(&mut self, s: &str) {
        // setto(s) sets (j+1),...k to the characters in the string s, readjusting k.
        let length = s.len();
        self.b = self.b[..self.j].to_string() + s + &self.b[self.j + length..];
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

        if self.b.chars().nth(self.k - 1).unwrap() == 's' {
            if self.ends("sses") {
                self.k -= 2;
            } else if self.ends("ies") {
                self.setto("i");
            } else if self.b.chars().nth(self.k - 2).unwrap() != 's' {
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
                let ch = self.b.chars().nth(self.k - 1).unwrap();
                if ch == 'l' || ch == 's' || ch == 'z' {
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
            self.b = self.b[..self.k - 1].to_string() + "i" + &self.b[self.k..];
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
        match self.b.chars().nth(self.k - 2).unwrap() {
            'a' => {
                if self.ends("ational") { self.r("ate"); }
                else if self.ends("tional") { self.r("tion"); }
            },
            'c' => {
                if self.ends("enci") { self.r("ence"); }
                else if self.ends("anci") { self.r("ance"); }
            },
            'e' => {
                if self.ends("izer") { self.r("ize"); }
            },
            'l' => {
                if self.ends("bli") { self.r("ble"); } // DEPARTURE
                // Original algorithm: if self.ends("abli") { self.r("able"); }
                else if self.ends("alli") { self.r("al"); }
                else if self.ends("entli") { self.r("ent"); }
                else if self.ends("eli") { self.r("e"); }
                else if self.ends("ousli") { self.r("ous"); }
            },
            'o' => {
                if self.ends("ization") { self.r("ize"); }
                else if self.ends("ation") { self.r("ate"); }
                else if self.ends("ator") { self.r("ate"); }
            },
            's' => {
                if self.ends("alism") { self.r("al"); }
                else if self.ends("iveness") { self.r("ive"); }
                else if self.ends("fulness") { self.r("ful"); }
                else if self.ends("ousness") { self.r("ous"); }
            },
            't' => {
                if self.ends("aliti") { self.r("al"); }
                else if self.ends("iviti") { self.r("ive"); }
                else if self.ends("biliti") { self.r("ble"); }
            },
            'g' => {
                if self.ends("logi") { self.r("log"); } // DEPARTURE
                // Original algorithm does not include this line
            },
            _ => (),
        }
    }

    fn step3(&mut self) {
        // step3() dels with -ic-, -full, -ness etc. similar strategy to step2.
        assert!(self.k > 0);
        match self.b.chars().nth(self.k - 1).unwrap() {
            'e' => {
                if self.ends("icate") { self.r("ic"); }
                else if self.ends("ative") { self.r(""); }
                else if self.ends("alize") { self.r("al"); }
            },
            'i' => {
                if self.ends("iciti") { self.r("ic"); }
            },
            'l' => {
                if self.ends("ical") { self.r("ic"); }
                else if self.ends("ful") { self.r(""); }
            },
            's' => {
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
        match self.b.chars().nth(self.k - 2).unwrap() {
            'a' => {
                if self.ends("al") { }
                else { return; }
            },
            'c' => {
                if self.ends("ance") { }
                else if self.ends("ence") { }
                else { return; }
            },
            'e' => {
                if self.ends("er") { }
                else { return; }
            },
            'i' => {
                if self.ends("ic") { }
                else { return; }
            },
            'l' => {
                if self.ends("able") { }
                else if self.ends("ible") { }
                else { return; }
            },
            'n' => {
                if self.ends("ant") { }
                else if self.ends("ement") { }
                else if self.ends("ment") { }
                else if self.ends("ent") { }
                else { return; }
            },
            'o' => {
                if self.ends("ion") && (self.j > 1 && (self.b.chars().nth(self.j - 1).unwrap() == 's' || self.b.chars().nth(self.j - 1).unwrap() == 't')) { }
                else if self.ends("ou") { }
                // takes care of -ous
                else { return; }
            },
            's' => {
                if self.ends("ism") { }
                else { return; }
            },
            't' => {
                if self.ends("ate") { }
                else if self.ends("iti") { }
                else { return; }
            },
            'u' => {
                if self.ends("ous") { }
                else { return; }
            },
            'v' => {
                if self.ends("ive") { }
                else { return; }
            },
            'z' => {
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
        if self.b.chars().nth(self.k - 1).unwrap() == 'e' {
            let a = self.m();
            if a > 1 || (a == 1 && !self.cvc(self.k - 2)) {
                self.k -= 1;
            }
        }
        if self.b.chars().nth(self.k - 1).unwrap() == 'l' && self.doublec(self.k - 1) && self.m() > 1 {
            self.k -= 1;
        }
    }

    fn stem(&mut self, p: &str) -> String {
        /* In stem(p,i,j), p is a char pointer, and the string to be stemmed
        is from p[i] to p[j] inclusive. Typically i is zero and j is the
        offset to the last character of a string, (p[j+1] == '\0'). The
        stemmer adjusts the characters p[i] ... p[j] and returns the new
        end-point of the string, k. Stemming never increases word length, so
        i <= k <= j. To turn the stemmer into a module, declare 'stem' as
        extern, and delete the remainder of this file.
        "*/
        self.b = p.to_string();
        self.k = self.b.len();

        // With this line, strings of length 1 or 2 don't go through the
        // stemming process, although no mention is made of this in the
        // published algorithm. Remove the line to match the published
        // algorithm.
        if self.k <= 2 {
            return self.b.clone(); // --DEPARTURE--
        }

        self.step1ab();
        self.step1c();
        self.step2();
        self.step3();
        self.step4();
        self.step5();
        self.b[..self.k].to_string()
    }
}

#[pyfunction]
pub fn american_english_stem(w: &str) -> String {
    // lower case ASCII a-z only
    let word = w.chars().filter(|c| c.is_ascii_alphabetic()).collect::<String>().to_lowercase();
    let mut stemmer = PorterStemmer::new();
    stemmer.stem(&word)
}
