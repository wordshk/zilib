// Sort a vector of strings by the CJK radical

use zilib::{data, cjk};

// call radical_cmp and print debug info
fn radical_cmp_debug(a: &Vec<char>, b: &Vec<char>) -> std::cmp::Ordering {
    let unihan_data = data::unihan_data(); // this can be a slow operation
    let cmp = cjk::radical_cmp(a, b);
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

// TODO: Not very sure what's the best way to transform and pass the vectors here.
fn radical_sort_vc(mut v: Vec<Vec<char>>, debug: bool) -> Vec<Vec<char>> {
    if debug {
        v.sort_by(radical_cmp_debug);
    } else {
        v.sort_by(cjk::radical_cmp);
    }
    v
}

fn radical_sort(v: &Vec<String>, debug: bool) -> Vec<String> {
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

    for s in radical_sort(&v, debug) {
        println!("{}", s);
    }
}
