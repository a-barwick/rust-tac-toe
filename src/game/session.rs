use super::game::Game;
use super::player::{Player, PlayerSymbol};

use crate::io::input::Input;

/// Initializes players and starts the game loop
pub fn start(input: &dyn Input) {
    println!("Welcome to Tic-Tac-Toe!");
    let player_one = Player {
        symbol: PlayerSymbol::X,
    };
    let player_two = Player {
        symbol: PlayerSymbol::O,
    };
    Game::new(&player_one, input).run(&player_one, &player_two);
}
