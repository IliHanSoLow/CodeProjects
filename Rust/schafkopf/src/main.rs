pub mod player;
pub mod card;
pub mod stack;
pub mod game;

use stack::*;
use game::*;

fn main() {
    let mut game = Game::new();
    game.pregame();
    let mut stack: Stack = Stack::new();
    stack.print();
    stack.shuffle();
    stack.print();
}