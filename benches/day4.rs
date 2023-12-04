use aoc_2023::parser::parse_line;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn get_input() -> &'static str {
    static INPUT: &str = include_str!("../input");
    return INPUT;
}

pub fn parse_benchmark(c: &mut Criterion) {
    let input = get_input();
    c.bench_function("parse_line", |b| b.iter(|| parse_line(black_box(input))));
}

pub fn calc_benchmark(c: &mut Criterion) {
    let input = get_input();
    let card = parse_line(input);
    c.bench_function("calc", |b| b.iter(|| black_box(card.clone()).matches()));
}

criterion_group!(parsing, parse_benchmark);
criterion_group!(calculating, calc_benchmark);
criterion_main!(parsing, calculating);
