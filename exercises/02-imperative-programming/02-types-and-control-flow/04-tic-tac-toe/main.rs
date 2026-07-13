// TODO: define the `Cell` enum (Empty, X, O) with required derives.

// TODO: define a type alias `Board` as `[[Cell; 3]; 3]`.

// TODO: implement `winner` — return the winning player or None.
// fn winner(board: &Board) -> Option<Cell> { ... }

// TODO: implement `is_full` — true if no cell is Empty.
// fn is_full(board: &Board) -> bool { ... }

fn main() {
    let mut board = [[Cell::Empty; 3]; 3];
    board[0][0] = Cell::X;
    board[0][1] = Cell::X;
    board[0][2] = Cell::X;
    println!("winner: {:?}", winner(&board));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_wins_row() {
        let mut board = [[Cell::Empty; 3]; 3];
        board[0][0] = Cell::X;
        board[0][1] = Cell::X;
        board[0][2] = Cell::X;
        assert_eq!(winner(&board), Some(Cell::X));
    }

    #[test]
    fn test_o_wins_column() {
        let mut board = [[Cell::Empty; 3]; 3];
        board[0][1] = Cell::O;
        board[1][1] = Cell::O;
        board[2][1] = Cell::O;
        assert_eq!(winner(&board), Some(Cell::O));
    }

    #[test]
    fn test_x_wins_diagonal() {
        let mut board = [[Cell::Empty; 3]; 3];
        board[0][0] = Cell::X;
        board[1][1] = Cell::X;
        board[2][2] = Cell::X;
        assert_eq!(winner(&board), Some(Cell::X));
    }

    #[test]
    fn test_o_wins_anti_diagonal() {
        let mut board = [[Cell::Empty; 3]; 3];
        board[0][2] = Cell::O;
        board[1][1] = Cell::O;
        board[2][0] = Cell::O;
        assert_eq!(winner(&board), Some(Cell::O));
    }

    #[test]
    fn test_no_winner() {
        let board = [[Cell::Empty; 3]; 3];
        assert_eq!(winner(&board), None);
    }

    #[test]
    fn test_is_full() {
        let mut board = [[Cell::X; 3]; 3];
        assert!(is_full(&board));
        board[2][2] = Cell::Empty;
        assert!(!is_full(&board));
    }
}
