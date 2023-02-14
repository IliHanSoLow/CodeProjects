pub mod card;
pub mod game;
pub mod player;
pub mod stack;

use game::*;
use stack::*;

fn main() {
    let mut game = Game::new();
    game.pregame();
}
