use super::board::Board;
use super::player::{Player, PlayerSymbol};

use crate::io::input::Input;

/// Represents the game state and logic
pub struct Game<'a> {
    pub board: Board,
    pub current_player: &'a Player,
    pub input: &'a dyn Input,
}

impl<'a> Game<'a> {
    /// Creates a new game with the current player
    pub fn new(current_player: &'a Player, input: &'a dyn Input) -> Game<'a> {
        Game {
            board: Board::new(),
            current_player,
            input,
        }
    }

    /// Runs the game loop
    pub fn run(&mut self, player_one: &'a Player, player_two: &'a Player) {
        loop {
            match self
                .get_player_selection()
                .and_then(|idx| self.set_move(idx))
                .map_err(|err| {
                    println!("{}", err);
                }) {
                Ok(_) => match self.get_state() {
                    GameState::InProgress => {
                        if self.current_player.symbol == player_one.symbol {
                            self.set_current_player(player_two);
                        } else {
                            self.set_current_player(player_one);
                        }
                    }
                    GameState::Win => {
                        println!("Player {:?} wins!", self.current_player.symbol);
                        break;
                    }
                    GameState::Draw => {
                        println!("Draw!");
                        break;
                    }
                },
                Err(_) => continue,
            }
        }
    }

    /// Draws the current board with numbers on the cells
    /// that the current player can select from
    fn draw_board(&self) {
        let board_repr: Vec<String> = self
            .board
            .cells
            .iter()
            .enumerate()
            .map(|(i, cell)| match cell.value {
                Some(PlayerSymbol::X) => "X".to_string(),
                Some(PlayerSymbol::O) => "O".to_string(),
                None => i.to_string(),
            })
            .collect();

        println!("\n-------------");
        println!(
            "| {} | {} | {} |",
            board_repr[0], board_repr[1], board_repr[2]
        );
        println!("|---|---|---|");
        println!(
            "| {} | {} | {} |",
            board_repr[3], board_repr[4], board_repr[5]
        );
        println!("|---|---|---|");
        println!(
            "| {} | {} | {} |",
            board_repr[6], board_repr[7], board_repr[8]
        );
        println!("-------------\n");
    }

    /// Checks if the current player has won or if the game is a draw
    fn get_state(&self) -> GameState {
        let winning_indices = [
            // Horizontal
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            // Vertical
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            // Diagonal
            [0, 4, 8],
            [2, 4, 6],
        ];
        for indices in winning_indices.iter() {
            let mut winning = true;
            for index in indices.iter() {
                if self.board.cells[*index].value != Some(self.current_player.symbol) {
                    winning = false;
                    break;
                }
            }
            if winning {
                return GameState::Win;
            }
        }
        let mut draw = true;
        for cell in self.board.cells.iter() {
            if cell.value == None {
                draw = false;
                break;
            }
        }
        if draw {
            return GameState::Draw;
        }
        GameState::InProgress
    }

    /// Gets the current player's selection from stdin
    fn get_player_selection(&self) -> Result<usize, &'static str> {
        println!("Type the number from one of the available cells:");
        self.draw_board();
        let user_input = match self.input.read() {
            Ok(input) => input,
            Err(msg) => {
                println!("{}", msg);
                return self.get_player_selection();
            }
        };
        match user_input.trim().parse::<usize>() {
            Ok(result) if result < 9 => return Ok(result),
            Ok(_) => return Err("Pick an available number from the board:"),
            Err(_) => return Err("Invalid input, pick a number from the board:"),
        };
    }

    /// Sets the current player
    fn set_current_player(&mut self, player: &'a Player) {
        self.current_player = player;
    }

    /// Sets the current player's move on the board
    fn set_move(&mut self, cell_index: usize) -> Result<(), &'static str> {
        if cell_index > 8 {
            return Err("Invalid cell number selected");
        }
        match self.board.cells[cell_index].value {
            Some(_) => Err("Cell already taken"),
            None => {
                self.board.cells[cell_index].value = Some(self.current_player.symbol);
                Ok(())
            }
        }
    }
}

enum GameState {
    InProgress,
    Win,
    Draw,
}

#[cfg(test)]
mod tests {
    use crate::io::input::MockInput;

    use super::*;

    #[test]
    fn test_new() {
        let player_one = Player {
            symbol: PlayerSymbol::X,
        };
        let mock_input = MockInput {
            input: "1".to_string(),
        };
        let game = Game::new(&player_one, &mock_input);
        assert_eq!(game.current_player.symbol, player_one.symbol);
        assert_eq!(game.board.cells.len(), 9);
        assert_eq!(game.board.cells[0].value, None);
    }

    #[test]
    fn test_run() {
        let player_one = Player {
            symbol: PlayerSymbol::X,
        };
        let player_two = Player {
            symbol: PlayerSymbol::O,
        };
        let mock_input = MockInput {
            input: "1".to_string(),
        };
        let mut game = Game::new(&player_one, &mock_input);
        game.run(&player_one, &player_two);
        assert_eq!(game.board.cells[0].value, Some(PlayerSymbol::X));
    }
}
