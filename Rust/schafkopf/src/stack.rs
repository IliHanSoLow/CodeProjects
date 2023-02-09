use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card::*;

pub struct Stack {
    cstack: Vec<Card>
}

impl Stack {
    pub fn new() -> Stack{
        Stack{ cstack : Stack::import_all_cards() }
    }

    fn import_all_cards() -> Vec<Card> {
        let mut tmp: Vec<Card> = Vec::new();
        for farbe in vec![Farbe::Eichel, Farbe::Gras, Farbe::Herz, Farbe::Schelle] {
            for typ in vec![
                Typ::Ass,
                Typ::Zehn,
                Typ::König,
                Typ::Ober,
                Typ::Unter,
                Typ::Neun,
                Typ::Acht,
                Typ::Sieben,
            ] {
                tmp.push(Card::new(farbe, typ));
            }
        }
        tmp
    }

    pub fn shuffle(& mut self) {
        self.cstack.shuffle(&mut thread_rng());
        println!("shuffled");
        println!();
    }

    pub fn print(&self) {
        for i in &self.cstack{
            i.print();
        }
        println!();
    }

    pub fn check_trumpf(&self){
        
    }
}
