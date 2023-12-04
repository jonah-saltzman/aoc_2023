use itertools::Itertools;

use crate::parser::{Coordinate, Number, Symbol};

pub struct Calculator {
    x: i32,
    y: i32,
    numbers: Vec<Number>,
    symbols: Vec<Vec<Symbol>>,
}

impl Calculator {
    pub fn new(x: i32, y: i32, numbers: Vec<Number>, symbols: Vec<Vec<Symbol>>) -> Self {
        Self {
            x,
            y,
            numbers,
            symbols,
        }
    }

    pub fn into_answer(self) -> usize {
        let mut answer: usize = 0;
        for &number in self.numbers.iter() {
            let y1 = (number.left.y - 1).max(0) as usize;
            let y2 = (number.left.y + 1).min(self.y) as usize;
            if self.touches_symbol(number, &self.symbols[y1..=y2]) {
                answer += number.val;
            }
        }
        answer
    }

    fn touches_symbol(&self, number: Number, symbols: &[Vec<Symbol>]) -> bool {
        let mut adjacent_coords = self.get_adjacents(number);
        let symbols = symbols.iter().flatten().map(|s| s.coord);
        adjacent_coords.any(|c| symbols.clone().contains(&c))
    }

    #[inline]
    fn is_on_board(&self, coord: &Coordinate) -> bool {
        coord.x >= 0 && coord.x <= self.x && coord.y >= 0 && coord.y <= self.y
    }

    fn get_adjacents(&self, number: Number) -> impl Iterator<Item = Coordinate> + '_ {
        let above = number.left.y - 1;
        let below = number.left.y + 1;
        let left = number.left.x - 1;
        let right = number.right.x + 1;

        let coords_above = (left..=right).map(move |x| Coordinate::new(x, above));
        let coords_below = (left..=right).map(move |x| Coordinate::new(x, below));
        let coords_left = (left..=left).map(move |x| Coordinate::new(x, number.left.y));
        let coords_right = (right..=right).map(move |x| Coordinate::new(x, number.right.y));
        coords_above
            .chain(coords_below)
            .chain(coords_left)
            .chain(coords_right)
            .filter(|c| self.is_on_board(c))
    }
}
