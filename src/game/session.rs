use super::game::Game;
use super::player::{Player, PlayerSymbol};

/// Initializes players and starts the game loop
pub fn start() {
    println!("Welcome to Tic-Tac-Toe!");
    let player_one = Player {
        symbol: PlayerSymbol::X,
    };
    let player_two = Player {
        symbol: PlayerSymbol::O,
    };
    Game::new(&player_one).run(&player_one, &player_two);
}
