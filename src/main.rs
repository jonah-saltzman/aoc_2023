use parser::parse_line;
use std::io::{stdin, BufRead};

use crate::calculator::get_power;

mod parser;
mod calculator;

fn main() {
    let mut buf = String::new();
    let mut reader = stdin().lock();
    let mut sum: usize = 0;
    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            break;
        }
        let game = parse_line(&buf);
        let game_power = get_power(&game);
        sum += game_power;
        buf.clear();
    }
    println!("{}", sum);
}