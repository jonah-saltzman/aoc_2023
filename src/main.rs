mod calculator;
mod parser;

pub use parser::{parse_line, Card};
use std::io::{stdin, BufRead};

fn main() {
    let mut buf = String::new();
    let mut reader = stdin().lock();
    let mut answer: i32 = 0;
    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            break;
        }
        let card = parse_line(&buf);
        let score = card.score();
        answer += score;
        buf.clear();
    }
    println!("{}", answer);
}
