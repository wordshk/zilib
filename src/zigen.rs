use bzip2::read::BzDecoder;
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/*
# More details about file format of varcon can be found in:
# - http://wordlist.aspell.net/scowl-readme/
# - https://github.com/en-wl/wordlist/blob/master/varcon/README
*/

fn generate_english_variants(filename : &str) -> io::Result<()> {

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

    // Printing the header
    println!("{}", r#"
Data taken from varcon.txt under the SCOWL project http://wordlist.aspell.net/
Attaching the Licenses and Copyright notices from the project below:

----

Copyright
=========

Copyright 2000-2020 by Kevin Atkinson (kevina@gnu.org) and Benjamin
Titze (btitze@protonmail.ch).

Copyright 2000-2019 by Kevin Atkinson

Permission to use, copy, modify, distribute and sell this array, the
associated software, and its documentation for any purpose is hereby
granted without fee, provided that the above copyright notice appears
in all copies and that both that copyright notice and this permission
notice appear in supporting documentation. Kevin Atkinson makes no
representations about the suitability of this array for any
purpose. It is provided "as is" without express or implied warranty.

Copyright 2016 by Benjamin Titze

Permission to use, copy, modify, distribute and sell this array, the
associated software, and its documentation for any purpose is hereby
granted without fee, provided that the above copyright notice appears
in all copies and that both that copyright notice and this permission
notice appear in supporting documentation. Benjamin Titze makes no
representations about the suitability of this array for any
purpose. It is provided "as is" without express or implied warranty.

Since the original words lists come from the Ispell distribution:

Copyright 1993, Geoff Kuenning, Granada Hills, CA
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:

1. Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.
3. All modifications to the source code must be clearly marked as
   such.  Binary redistributions based on modified source code
   must be clearly marked as modified versions in the documentation
   and/or other materials provided with the distribution.
(clause 4 removed with permission from Geoff Kuenning)
5. The name of Geoff Kuenning may not be used to endorse or promote
   products derived from this software without specific prior
   written permission.

THIS SOFTWARE IS PROVIDED BY GEOFF KUENNING AND CONTRIBUTORS ``AS IS'' AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED.  IN NO EVENT SHALL GEOFF KUENNING OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
SUCH DAMAGE.
"#.trim());

    println!("VARIANT_TO_US_ENGLISH = {{");
    for (k, s) in reverse_map {
        for v in &s {
            if v != &k {
                println!("    {:?}: {:?},", v, k);
            }
        }
    }
    println!("}}");

    Ok(())
}

fn is_suffix(s: &str, ref_set: &HashSet<String>) -> bool {
    (s.ends_with("'s") || s.ends_with("ed")) && ref_set.contains(&s[..s.len()-2].to_string())
    || (s.ends_with('s') || s.ends_with('d')) && ref_set.contains(&s[..s.len()-1].to_string())
}

fn usage() {
    println!("Usage: zigen <command> <args>");
    println!("Commands:");
    println!("  generate_english_variants <filename> - Generate a map of English variants from the varcon file");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match (args.get(1).map(String::as_str), args.get(2)) {
        (Some("generate_english_variants"), Some(filename)) => generate_english_variants(filename),
        _ => {
            usage();
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid usage"))
        }
    }
}
