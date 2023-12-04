use calculator::Calculator;
use std::io::{stdin, Read};

mod calculator;
mod parser;

fn main() {
    let mut buf = String::new();
    let mut reader = stdin().lock();
    if reader.read_to_string(&mut buf).is_ok() {
        let data = parser::get_data(&buf);
        let calc = Calculator::new(data.x as i32, data.y as i32, data.numbers, data.symbols);
        let ans = calc.into_answer();
        println!("{}", ans);
    }
}
