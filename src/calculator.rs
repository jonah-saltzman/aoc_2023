use crate::parser::{Game, Pull};


pub fn get_power(game: &Game) -> usize {
    let mut min_green: usize = 0;
    let mut min_red: usize = 0;
    let mut min_blue: usize = 0;
    for set in game.sets.iter() {
        for ele in set.0.iter() {
            match ele {
                Pull::Red(n) => min_red = min_red.max(*n),
                Pull::Green(n) => min_green = min_green.max(*n),
                Pull::Blue(n) => min_blue = min_blue.max(*n)
            }
        }
    }
    min_red * min_green * min_blue
}