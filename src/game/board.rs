use super::player::PlayerSymbol;

/// Represents the 9x9 game board
pub struct Board {
    pub cells: Vec<Cell>,
}

impl Board {
    /// Creates a new board with 9 empty cells
    pub fn new() -> Board {
        let mut cells: Vec<Cell> = Vec::with_capacity(9);
        for _ in 0..9 {
            cells.push(Cell { value: None })
        }
        Board { cells }
    }
}

/// Represents a single cell on the board
pub struct Cell {
    pub value: Option<PlayerSymbol>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board = Board::new();
        assert_eq!(board.cells.len(), 9);
    }

    #[test]
    fn test_new_cell() {
        let cell = Cell { value: None };
        assert_eq!(cell.value, None);
    }
}
