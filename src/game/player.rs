/// Represents a player
pub struct Player {
    pub symbol: PlayerSymbol,
}

/// Represents a player's symbol (X or O)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerSymbol {
    X,
    O,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_player() {
        let player = Player {
            symbol: PlayerSymbol::X,
        };
        assert_eq!(player.symbol, PlayerSymbol::X);
    }
}
