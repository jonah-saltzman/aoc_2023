use itertools::Itertools;

use crate::parser::{Coordinate, HasCoordinates, Number, Symbol};

pub struct Calculator {
    x: i32,
    y: i32,
    numbers: Vec<Vec<Number>>,
    symbols: Vec<Vec<Symbol>>,
}

impl Calculator {
    pub fn new(x: i32, y: i32, numbers: Vec<Vec<Number>>, symbols: Vec<Vec<Symbol>>) -> Self {
        Self {
            x,
            y,
            numbers,
            symbols,
        }
    }

    pub fn into_answer(self) -> usize {
        let mut answer: usize = 0;
        for symbol in self.symbols.iter().flatten() {
            let y1 = (symbol.coord.y - 1).min(0) as usize;
            let y2 = (symbol.coord.y + 1).max(self.y) as usize;
            let adj_nums = self.adjacent_numbers(*symbol, &self.numbers[y1..=y2]);
            if adj_nums.len() == 2 {
                answer += adj_nums[0].val * adj_nums[1].val
            }
        }
        answer
    }

    #[allow(dead_code)]
    fn touches_symbol(&self, number: Number, symbols: &[Vec<Symbol>]) -> bool {
        let mut adjacent_coords = self.get_adjacents(number);
        let symbols = symbols.iter().flatten().map(|s| s.coord);
        adjacent_coords.any(|c| symbols.clone().contains(&c))
    }

    fn adjacent_numbers(&self, symbol: Symbol, numbers: &[Vec<Number>]) -> Vec<Number> {
        let adjacent_coords = self.get_adjacents(symbol).collect_vec();
        numbers
            .iter()
            .flatten()
            .map(|n| *n)
            .filter(|&n| {
                adjacent_coords
                    .iter()
                    .any(|&c| self.number_includes_coord(n, c))
            })
            .collect_vec()
    }

    #[inline]
    fn number_includes_coord(&self, number: Number, coord: Coordinate) -> bool {
        number.left.y == coord.y && coord.x >= number.left.x && coord.x <= number.right.x
    }

    #[inline]
    fn is_on_board(&self, coord: &Coordinate) -> bool {
        coord.x >= 0 && coord.x <= self.x && coord.y >= 0 && coord.y <= self.y
    }

    fn get_adjacents<'a, T>(&'a self, number: T) -> impl Iterator<Item = Coordinate> + '_
    where
        T: HasCoordinates + Copy + 'a,
    {
        let above = number.left().y - 1;
        let below = number.left().y + 1;
        let left = number.left().x - 1;
        let right = number.right().x + 1;

        let coords_above = (left..=right).map(move |x| Coordinate::new(x, above));
        let coords_below = (left..=right).map(move |x| Coordinate::new(x, below));
        let coords_left = (left..=left).map(move |x| Coordinate::new(x, number.left().y));
        let coords_right = (right..=right).map(move |x| Coordinate::new(x, number.right().y));
        coords_above
            .chain(coords_below)
            .chain(coords_left)
            .chain(coords_right)
            .unique()
            .filter(|c| self.is_on_board(c))
    }
}
