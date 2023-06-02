use crate::player::{self, Player};
use crate::stack::Stack;
use std::arch::x86_64::_mm_insert_epi32;
use std::io;

pub struct Game {
    player1: Player,
    player2: Player,
    player3: Player,
    player4: Player,
    thisplayer: i32,
    cstack: Stack,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player1: Player::new(),
            player2: Player::new(),
            player3: Player::new(),
            player4: Player::new(),
            thisplayer: 0,
            cstack: Stack::new(),
        }
    }

    pub fn pregame(&mut self) {
        println!("Schafkopf");
        println!();
        println!("Es wird mit 4 Spielern gespielt!");
        println!("Spieler 1:");
        self.player1.set_name();
        println!("Spieler 2:");
        self.player2.set_name();
        println!("Spieler 3:");
        self.player2.set_name();
        println!("Spieler 4:");
        self.player2.set_name();
        self.cstack.print();
        self.cstack.shuffle();
        self.cstack.check_trumpf();
        self.cstack.print();
        println!("Suche dir die Spielart aus:");
        println!("1. Sauspiel");
        println!("2. todo!()");
        io::stdin().read_line(inp).expect("Failed to get input");
        match inp {
            "1" => sauspiel(),
            "2" => todo!(),
            _ => jump!(),
        }
    }

    fn get_cards(&mut self) {
        self.player1.set_cards(self.cstack.pop());
    }

    fn sauspiel() {
        todo!()
    }
}
