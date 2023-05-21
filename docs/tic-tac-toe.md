# System Design for Tic-Tac-Toe

## Requirements

- 2 players
- 3x3 grid
- 3 in a row wins
- 9 moves max

## Design

### Components

- Game
- Board
- Player
- Move

### Data Structures

- Board
  - 3x3 grid
- Player
    - Name
    - Symbol
- Move
    - Player
    - Cell

## UX Flow

1. Game intro
2. Display board with selections
3. Player 1 inputs number of selection in the console, hits enter
4. Game checks the validity of the move
5. If invalid, show error to player and loop back to 3.
6. If valid, write symbol to cell
7. Validate game state
8. If Win, show win text with current player
9. If Draw, show draw text
10. If InProgress, restart at 3. with Player 2


# TODO

- [ ] Refactor out game loop to allow testing
- [ ] Add tests for game loop
