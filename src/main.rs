use parser::parse_line;
use std::io::{stdin, BufRead};

mod parser;

fn main() {
    let mut buf = String::new();
    let mut reader = stdin().lock();
    let mut sum: usize = 0;
    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            break;
        }
        let game = parse_line(&buf);
        if game.is_possible() {
            sum += game.number
        }
        buf.clear();
    }
    println!("{}", sum);
}