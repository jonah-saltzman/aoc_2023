use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use anyhow::Result;
use crate::calculator::{CatMap, MapRange, SortedCatMap};

#[derive(Parser)]
#[grammar = "parser.pest"]
struct InputParser;

#[derive(Debug)]
struct ParseErr;

impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("infallible")
    }
}

impl std::error::Error for ParseErr {}



#[derive(PartialEq, Debug)]
enum ParserState {
    ExpectDef,
    ExpectNL,
    Input,
    Map(CatMap),
    Done,
}

impl Default for ParserState {
    fn default() -> Self {
        Self::ExpectDef
    }
}

#[derive(Debug)]
pub struct Seeds(pub Vec<i64>);

impl From<Pair<'_, Rule>> for Seeds {
    fn from(value: Pair<'_, Rule>) -> Self {
        assert_eq!(value.as_rule(), Rule::SEEDS_DEF);
        Self(value.into_inner().map(|p| p.as_str().parse().unwrap()).collect())
    }
}

pub struct MapHead {
    from: String,
    to: String
}

impl From<Pair<'_, Rule>> for MapHead {
    fn from(value: Pair<'_, Rule>) -> Self {
        assert_eq!(value.as_rule(), Rule::MAP_HEAD);
        let mut inner = value.into_inner();
        let from = inner.next().unwrap().as_str().to_owned();
        let to = inner.next().unwrap().as_str().to_owned();
        Self { from, to }
    }
}

impl From<Pair<'_, Rule>> for MapRange {
    fn from(value: Pair<'_, Rule>) -> Self {
        assert_eq!(value.as_rule(), Rule::NUM_LINE);
        let mut inner = value.into_inner();
        let dest_start: i64 = inner.next().unwrap().as_str().parse().unwrap();
        let src_start: i64 = inner.next().unwrap().as_str().parse().unwrap();
        let len: i64 = inner.next().unwrap().as_str().parse().unwrap();
        let offset = dest_start - src_start;
        MapRange::new(src_start, len, offset)
    }
}

pub enum ParserOutput {
    None,
    Seeds(Seeds),
    Map(SortedCatMap)
}

pub struct Machine {
    state: ParserState,
    seeds: Option<Seeds>,
    maps: Vec<SortedCatMap>
}

pub struct Output {
    pub seeds: Seeds,
    pub maps: Vec<SortedCatMap>
}

impl Machine {
    pub fn new() -> Self {
        Self { state: ParserState::ExpectDef, seeds: None, maps: vec![] }
    }

    pub fn step(&mut self, input: &str) {
        match std::mem::take(&mut self.state) {
            ParserState::ExpectDef => {
                let seeds = Machine::seed_def(input);
                self.state = ParserState::ExpectNL;
                self.seeds = Some(seeds);
            },
            ParserState::ExpectNL => {
                let _ = Machine::newline(input);
                self.state = ParserState::Input;
            },
            ParserState::Input => {
                if let Ok(head) = Machine::map_head(input) {
                    self.state = ParserState::Map(CatMap::new(head.from, head.to));
                } else {
                    let _ = Machine::newline(input);
                    self.state = ParserState::Done;
                }
            },
            ParserState::Map(mut map) => {
                if let Ok(line) = Machine::map_line(input) {
                    map.add_range(line);
                    self.state = ParserState::Map(map);
                } else {
                    let _ = Machine::newline(input);
                    self.state = ParserState::Input;
                    self.maps.push(SortedCatMap::from_unsorted(map));
                }
            }
            ParserState::Done => {}
        }
    }

    pub fn into_result(self) -> Output {
        let mut output = Output { seeds: self.seeds.unwrap(), maps: self.maps };
        match self.state {
            ParserState::Map(map) => {
                output.maps.push(SortedCatMap::from_unsorted(map));
                output
            },
            _ => output
        }
        
    }

    fn seed_def(input: &str) -> Seeds {
        let line = InputParser::parse(Rule::SEEDS_DEF, input).unwrap().next().unwrap();
        line.into()
    }

    fn newline(input: &str) -> () {
        let _ = InputParser::parse(Rule::NL, input).unwrap();
    }

    fn map_head(input: &str) -> Result<MapHead> {
        let line = InputParser::parse(Rule::MAP_HEAD, input)?.next().ok_or(ParseErr)?;
        Ok(line.into())
    }

    fn map_line(input: &str) -> Result<MapRange> {
        let line = InputParser::parse(Rule::NUM_LINE, input)?.next().ok_or(ParseErr)?;
        Ok(line.into())
    }
}