#![allow(dead_code)]
use std::collections::HashSet;

use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser.pest"]
struct GameParser;

#[derive(Clone)]
pub struct Card {
    number: i32,
    win_numbers: HashSet<i32>,
    have_numbers: HashSet<i32>,
}

impl Card {

    pub fn number(&self) -> i32 {
        self.number
    }

    pub fn win_numbers(&self) -> impl Iterator<Item = &i32> {
        self.win_numbers.iter()
    }

    pub fn win_set(&self) -> &HashSet<i32> {
        &self.win_numbers
    }

    pub fn have_numbers(&self) -> impl Iterator<Item = &i32> {
        self.have_numbers.iter()
    }

    pub fn have_set(&self) -> &HashSet<i32> {
        &self.have_numbers
    }
}

pub fn parse_line(line: &str) -> Card {
    let mut parsed = GameParser::parse(Rule::LINE, line).unwrap();
    let mut line = parsed.next().unwrap().into_inner();
    let card_id = line.next().unwrap();
    let number: i32 = card_id.into_inner().next().unwrap().as_str().parse().unwrap();
    let win_numbers = generate_num_list(&mut line);
    let have_numbers = generate_num_list(&mut line);
    Card {
        number,
        win_numbers,
        have_numbers,
    }
}

fn generate_num_list(line: &mut Pairs<'_, Rule>) -> HashSet<i32> {
    line.next()
        .unwrap()
        .into_inner()
        .map(|n| n.as_str().parse().unwrap())
        .collect()
}
