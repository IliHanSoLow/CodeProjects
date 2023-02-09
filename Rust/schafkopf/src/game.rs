use crate::player::Player;
use std::io;

pub struct Game {
    player1: Player,
    player2: Player,
    player3: Player,
    player4: Player,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player1: Player::new(),
            player2: Player::new(),
            player3: Player::new(),
            player4: Player::new()
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
    }
}
