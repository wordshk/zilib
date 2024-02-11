// Read stdin by lines into a vector and count the number of "zi" according to specification.

use zilib::common;
fn cjk_count(s: &str) -> usize {
    s.chars().filter(|c| common::is_cjk_cp(*c as u32)).count()
}

fn unicode_count(s: &str) -> usize {
    s.chars().count()
}

fn usage() {
    eprintln!("Usage: zicount --cjk|--unicode");
    eprintln!("Count the number of recognized characters (as specified by the option) in the input.");
    std::process::exit(1);
}

fn main() {
    let which_counter = match std::env::args().nth(1).as_deref() {
        Some("cjk") => cjk_count,
        Some("--cjk") => cjk_count,
        Some("unicode") => unicode_count,
        Some("--unicode") => unicode_count,
        _ => {
            usage();
            std::process::exit(1);
        }
    };

    let mut count = 0;
    loop {
        let mut s = String::new();
        match std::io::stdin().read_line(&mut s) {
            Ok(0) => break,
            Ok(_) => {
                count += which_counter(&s);
            }
            Err(e) => {
                eprintln!("error: {}", e);
                std::process::exit(1);
            }
        }
    }
    println!("{}", count);

}

