#![allow(dead_code)]

use pest::{iterators::{Pair, Pairs}, *};
use pest_derive::Parser;

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

#[derive(Parser)]
#[grammar = "parser.pest"]
struct GameParser;

#[derive(Debug)]
pub enum Pull {
    Red(usize),
    Green(usize),
    Blue(usize),
}

impl Pull {
    pub fn is_possible(&self) -> bool {
        match self {
            Pull::Red(n) => *n <= MAX_RED,
            Pull::Green(n) => *n <= MAX_GREEN,
            Pull::Blue(n) => *n <= MAX_BLUE
        }
    }
}

impl From<Pair<'_, Rule>> for Pull {
    fn from(value: Pair<Rule>) -> Self {
        assert_eq!(value.as_rule(), Rule::SET_ELE);
        let mut inner = value.into_inner();
        let num_cubes = inner.next().unwrap();
        let num_cubes: usize = num_cubes.as_str().parse().unwrap();
        let color = inner.next().unwrap();
        match color.as_str() {
            "red" => Pull::Red(num_cubes),
            "green" => Pull::Green(num_cubes),
            "blue" => Pull::Blue(num_cubes),
            _ => unreachable!("parser error"),
        }
    }
}

#[derive(Debug)]
pub struct Set(pub Vec<Pull>);

impl From<Pairs<'_, Rule>> for Set {
    fn from(value: Pairs<'_, Rule>) -> Self {
        Set(value.map(|e| Pull::from(e)).collect())
    }
}

#[derive(Debug)]
pub struct Game {
    pub number: usize,
    pub sets: Vec<Set>,
}

impl Game {
    pub fn is_possible(&self) -> bool {
        for set in self.sets.iter() {
            for pull in set.0.iter() {
                if !pull.is_possible() {
                    return false
                }
            }
        }
        true
    }
}

impl From<Pairs<'_, Rule>> for Game {
    fn from(mut value: Pairs<'_, Rule>) -> Self {
        let line = value.next().unwrap();
        assert_eq!(line.as_rule(), Rule::LINE);
        let mut line = line.into_inner();
        let game_id = line.next().unwrap();
        let game_num = game_id.into_inner().next().unwrap();
        let sets = line.next().unwrap();
        let sets: Vec<Set> = sets.into_inner().map(|e| Set::from(e.into_inner())).collect();
        Game { number: game_num.as_str().parse().unwrap(), sets }
    }
}

pub fn parse_line(line: &str) -> Game {
    let parsed = GameParser::parse(Rule::LINE, line).unwrap();
    parsed.into()
}
