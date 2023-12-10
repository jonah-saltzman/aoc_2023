mod calculator;
mod parser;

pub use parser::{Machine, ParserOutput};
use calculator::closest_seed;
use std::io::{stdin, BufRead};

fn main() {
    let mut buf = String::new();
    let mut reader = stdin().lock();
    let mut parser = Machine::new();
    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            let output = parser.into_result();
            let result = closest_seed(output.seeds, output.maps);
            println!("{}", result);
            break
        }
        parser.step(&buf);
        buf.clear();
    }
}
