use std::io::{stdin, BufRead};
use parser::parse_line;

mod parser;

fn main() {
    let mut buf = String::new();
    let mut reader = stdin().lock();
    let mut sum: usize = 0;
    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            break;
        }
        sum += parse_line(&buf[..n - 1]);
        buf.clear();
    }
    println!("{}", sum);
}
