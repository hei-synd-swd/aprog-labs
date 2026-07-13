---
id          = "impp_tic_tac_toe"
name        = "Tic-Tac-Toe Logic"
language    = "rust"
difficulty  = 4
description = "Use enums, arrays, and loops to check win conditions on a board."
topics      = ["enum", "array", "loop", "win-condition"]
---

# Tic-Tac-Toe Logic

Implement the core win-checking logic for a tic-tac-toe game.

1. Define an enum `Cell` with variants `Empty`, `X`, `O`. Derive `Copy`, `Clone`,
   and `PartialEq`.
2. Define a type alias `Board = [[Cell; 3]; 3]`.
3. Implement `winner(board: &Board) -> Option<Cell>` that returns:
   - `Some(Cell::X)` if X has three in a row, column, or diagonal.
   - `Some(Cell::O)` if O has three in a row, column, or diagonal.
   - `None` if there is no winner.
4. Implement `is_full(board: &Board) -> bool` that returns `true` when
   no cell is `Empty`.

## Expected Result

```
let mut board = [[Cell::Empty; 3]; 3];
board[0][0] = Cell::X; board[0][1] = Cell::X; board[0][2] = Cell::X;
assert_eq!(winner(&board), Some(Cell::X));

board = [[Cell::Empty; 3]; 3];
board[0][0] = Cell::O; board[1][1] = Cell::O; board[2][2] = Cell::O;
assert_eq!(winner(&board), Some(Cell::O));

board = [[Cell::Empty; 3]; 3];
board[0][0] = Cell::X; board[1][2] = Cell::O;
assert_eq!(winner(&board), None);
assert!(!is_full(&board));
```
