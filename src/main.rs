mod calculator;
mod parser;

pub use parser::{parse_line, Card};
pub use calculator::Calculator;
use std::io::{stdin, BufRead};

fn main() {
    let mut buf = String::new();
    let mut reader = stdin().lock();
    let mut calc = Calculator::new();
    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            break;
        }
        let card = parse_line(&buf);
        calc.handle_card(card);
        buf.clear();
    }
    let answer = calc.into_result();
    println!("{}", answer);
}
