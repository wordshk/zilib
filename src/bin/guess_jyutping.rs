use zilib::cantonese;

fn main() {
    // read line from stdin
    loop {
        let mut line = String::new();
        let result = std::io::stdin().read_line(&mut line);
        if result.is_err() || line.len() == 0 {
            break;
        }

        let result = cantonese::get_ping3jam1(&line);
        println!("{}", result);
    }
}
