use itertools::Itertools;

use crate::parser::Card;

impl Card {
    pub fn score(self) -> i32 {
        let intersection = self.have_set().intersection(self.win_set()).collect_vec();
        match intersection.len() {
            0 => 0,
            n => 2_i32.pow(n as u32 - 1),
        }
    }
}
