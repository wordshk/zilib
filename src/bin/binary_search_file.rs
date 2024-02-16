use zilib::common;

use std::fs::File;
use std::io::{self, Seek, SeekFrom, Read};


fn records_from_sorted_file(path: &str, target: &str, field_delim: u8, cmp: fn(&[u8], &[u8]) -> std::cmp::Ordering) -> io::Result<Vec<String>> {
    let pos = common::binary_search_file(path, target.as_bytes(), b'\n', field_delim, 0, None, 1, cmp)?;
    // println!("pos: {:?}", pos);

    if pos.is_none() {
        return Ok(vec![]);
    }

    let mut f = File::open(path)?;
    f.seek(SeekFrom::Start(pos.unwrap() as u64))?;

    let mut ret = vec![];

    loop {
        let mut line = vec![];
        let mut byte = [0];
        while byte[0] != b'\n' {
            let did_read = f.read(&mut byte)?;
            if did_read == 0 {
                break;
            }
            line.push(byte[0]);
        }
        if line.is_empty() {
            break;
        }
        if line.split(|&c| c == field_delim || c == b'\n').next().unwrap() != target.as_bytes() {
            break;
        }
        // Convert to string assuming UTF-8
        let s = String::from_utf8(line).or(Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8")))?;
        ret.push(s);
    }
    Ok(ret)
}

fn main() -> io::Result<()> {
    // Usage: binary_search_file <file> <target> [field_delim]
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let target = &args[2];
    let field_delim = if args.len() > 3 {
        args[3].as_bytes()[0]
    } else {
        b','
    };
    let which_cmp = if args.len() > 4 {
        args[4].as_str()
    } else {
        "lex"
    };
    let cmp = match which_cmp {
        "length" => |a: &[u8], b: &[u8]| { if a.len() == b.len() { a.cmp(b) } else { a.len().cmp(&b.len()) } },
        "lex" => |a: &[u8], b: &[u8]| { a.cmp(b) },
        _ => panic!("Invalid comparison function")
    };

    // Compare lengths first, if length is equal, compare lexicographically
    let records = records_from_sorted_file(path, target, field_delim, cmp)?;
    for record in records {
        print!("{}", record);
    }

    Ok(())
}
