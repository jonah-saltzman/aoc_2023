use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser.pest"]
struct TextParser;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Number {
    pub val: usize,
    pub left: Coordinate,
    pub right: Coordinate,
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Number({}) @ from {},{} to {},{}",
            self.val, self.left.x, self.left.y, self.right.x, self.right.y
        ))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Symbol {
    pub coord: Coordinate,
}

pub struct ParseData {
    pub numbers: Vec<Number>,
    pub symbols: Vec<Vec<Symbol>>,
    pub x: usize,
    pub y: usize,
}

pub fn get_data(input: &str) -> ParseData {
    let text = TextParser::parse(Rule::TEXT, input)
        .unwrap()
        .next()
        .unwrap();
    assert_eq!(text.as_rule(), Rule::TEXT);
    let lines = text.into_inner().enumerate();
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Vec<Symbol>> = vec![];
    let mut x_max: usize = 0;
    let mut y_max: usize = 0;
    for (y, line) in lines {
        y_max = y;
        let line_len = line.as_str().len();
        x_max = line_len - 1;
        let offset = y + (line_len * y);
        symbols.push(vec![]);
        for ele in line.into_inner() {
            match ele.as_rule() {
                Rule::NUMBER => {
                    let val: usize = ele.as_str().parse().unwrap();
                    let left = Coordinate::new((ele.as_span().start() - offset) as i32, y as i32);
                    let right =
                        Coordinate::new((ele.as_span().end() - offset - 1) as i32, y as i32);
                    let number = Number { val, left, right };
                    numbers.push(number);
                }
                Rule::SYMBOL => {
                    let x = ele.as_span().start() - offset;
                    let coord = Coordinate::new(x as i32, y as i32);
                    let symbol = Symbol { coord };
                    symbols[y].push(symbol);
                }
                Rule::SPACE => {}
                _ => panic!("unexpected rule"),
            }
        }
    }
    ParseData {
        numbers,
        symbols,
        x: x_max,
        y: y_max,
    }
}
