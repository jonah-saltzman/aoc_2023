use std::collections::HashMap;

use itertools::Itertools;

use crate::parser::Card;

impl Card {
    pub fn matches(&self) -> i32 {
        let intersection = self.have_set().intersection(self.win_set()).collect_vec();
        intersection.len() as i32
    }
}

#[derive(Default)]
pub struct Calculator {
    num_cards: i32,
    extras: HashMap<i32, i32>
}

impl Calculator {

    pub fn new() -> Self {
        Self { num_cards: 0, extras: HashMap::new() }
    }

    pub fn handle_card(&mut self, card: Card) {
        let instances = *self.extras.get(&card.number()).unwrap_or(&1);
        self.num_cards += instances;
        let matches = card.matches();
        for card_num in (card.number() + 1)..=(card.number() + matches) {
            let prev_copies = self.extras.get(&card_num).unwrap_or(&1);
            let new_copies = prev_copies + instances;
            self.extras.insert(card_num, new_copies);
        }
    }

    pub fn into_result(self) -> i32 {
        self.num_cards
    }
}