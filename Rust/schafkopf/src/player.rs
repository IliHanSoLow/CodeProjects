use crate::card::*;
use crate::stack::*;
use std::io;

pub struct Player {
    name: String,
    cards: Vec<Card>,
}
impl Player {
    pub fn new() -> Player {
        Player {
            name: String::from("Player"),
            cards: vec![],
        }
    }
    pub fn set_name(&mut self) {
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("Failed to read");
        self.name = inp;
    }
    pub fn set_cards(&mut self, c:Card){
        self.cards.push(c);
    }
}
