use handlebars::Handlebars;
use std::{collections::HashMap, io};

fn main() {
    let player_one = Player {
        symbol: PlayerSymbol::X,
    };
    let player_two = Player {
        symbol: PlayerSymbol::O,
    };
    GameSession::new(&player_one, &player_two).start();
}

struct GameSession<'a> {
    game: Game<'a>,
    player_one: &'a Player,
    player_two: &'a Player,
}

impl<'a> GameSession<'a> {
    fn new(player_one: &'a Player, player_two: &'a Player) -> GameSession<'a> {
        GameSession {
            game: Game::new(&player_one),
            player_one,
            player_two,
        }
    }

    fn start(&mut self) {
        println!("Welcome to Tic-Tac-Toe!");
        loop {
            let selected_cell = self.game.get_player_selection();
            self.game.set_move(selected_cell);
            match self.game.get_state() {
                GameState::InProgress => {
                    if self.game.current_player.symbol == self.player_one.symbol {
                        self.game.set_current_player(self.player_two);
                    } else {
                        self.game.set_current_player(self.player_one);
                    }
                }
                GameState::Win => {
                    println!("Player {:?} wins!", self.game.current_player.symbol);
                    break;
                }
                GameState::Draw => {
                    println!("Draw!");
                    break;
                }
            }
        }
    }
}

struct Game<'a> {
    board: Board,
    current_player: &'a Player,
}

impl<'a> Game<'a> {
    fn new(current_player: &'a Player) -> Game<'a> {
        Game {
            board: Board::new(),
            current_player,
        }
    }

    /// Draws the current board with numbers on the cells
    /// that the current player can select from
    fn draw_board(&self) {
        let mut hb = Handlebars::new();
        let source = "
            -------------
            | {{0}} | {{1}} | {{2}} |
            |---|---|---|
            | {{3}} | {{4}} | {{5}} |
            |---|---|---|
            | {{6}} | {{7}} | {{8}} |
            -------------
        ";
        hb.register_template_string("board", source).unwrap();
        let mut hm = HashMap::new();
        for (i, cell) in self.board.cells.iter().enumerate() {
            hm.insert(
                i,
                match cell.value {
                    Some(PlayerSymbol::X) => "X".to_string(),
                    Some(PlayerSymbol::O) => "O".to_string(),
                    None => i.to_string(),
                },
            );
        }
        println!("{}", hb.render("board", &hm).unwrap());
    }

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

    fn get_player_selection(&self) -> i8 {
        println!("Type the number from one of the available cells:");
        self.draw_board();
        let selected_index = loop {
            let mut user_input = String::new();
            if let Err(_) = io::stdin().read_line(&mut user_input) {
                println!("No input, don't be typin nothin!");
            }
            match user_input.trim().parse::<i8>() {
                Ok(result) if result >= 0 && result < 9 => break result,
                Ok(_) => println!("Pick an available number from the board:"),
                Err(_) => println!("Invalid input, pick a number from the board:"),
            };
        };
        println!("Current player selected {:?}", selected_index);
        selected_index
    }

    fn set_current_player(&mut self, player: &'a Player) {
        self.current_player = player;
    }

    fn set_move(&mut self, cell_index: i8) {
        self.board.cells[cell_index as usize].value = Some(self.current_player.symbol);
    }
}

struct Board {
    cells: Vec<Box<Cell>>,
}

impl Board {
    fn new() -> Board {
        let mut cells: Vec<Box<Cell>> = Vec::with_capacity(9);
        for _ in 0..9 {
            cells.push(Box::new(Cell { value: None }))
        }
        Board { cells }
    }
}

struct Cell {
    value: Option<PlayerSymbol>,
}

struct Player {
    symbol: PlayerSymbol,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PlayerSymbol {
    X,
    O,
}

enum GameState {
    InProgress,
    Win,
    Draw,
}
