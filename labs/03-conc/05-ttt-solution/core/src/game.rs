//! The rules of tic-tac-toe: `Game`, `Player`, `State`, `PlayError`.
//!
//! Pure logic. Nothing here knows about threads, sockets or the bus —
//! which is why every test at the bottom of this file is a plain function
//! call with no setup.

use std::fmt;

/// The two players. X always starts.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn other(self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

/// Where the game stands after each move.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    Playing,
    /// The winner, and the three cells (1..=9) of the winning line.
    Won(Player, [u8; 3]),
    Draw,
}

/// Why a move was refused. The caller decides how to tell the user.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlayError {
    OutOfRange,
    CellTaken,
    GameOver,
}

impl fmt::Display for PlayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlayError::OutOfRange => write!(f, "pick a cell between 1 and 9"),
            PlayError::CellTaken => write!(f, "that cell is already taken"),
            PlayError::GameOver => write!(f, "the game is over"),
        }
    }
}

/// The 8 ways to win, written as cell numbers (1..=9).
const LINES: [[u8; 3]; 8] = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9], // rows
    [1, 4, 7],
    [2, 5, 8],
    [3, 6, 9], // columns
    [1, 5, 9],
    [3, 5, 7], // diagonals
];

/// One game: the board, whose turn it is, and how it stands.
///
/// A cell holds `Some(player)` if it is taken and `None` if it is free.
/// Cells are numbered 1..=9 like a phone keypad, so `board[0]` is cell 1.
#[derive(Clone)]
pub struct Game {
    board: [Option<Player>; 9],
    turn: Player,
    state: State,
}

// `Game::new()` takes no argument, so Rust expects a `Default` too.
// It does the same thing.
impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: [None; 9],
            turn: Player::X,
            state: State::Playing,
        }
    }

    pub fn turn(&self) -> Player {
        self.turn
    }

    pub fn state(&self) -> State {
        self.state
    }

    /// What is in cell `n` (1..=9). Panics outside that range — every
    /// caller checks the range first, `play` included.
    fn cell(&self, n: u8) -> Option<Player> {
        self.board[(n - 1) as usize]
    }

    /// The numbers of the cells that are still free, in order.
    /// Handy for anyone who has to pick a move.
    pub fn free_cells(&self) -> Vec<u8> {
        let mut free = Vec::new();
        for n in 1..=9u8 {
            if self.cell(n).is_none() {
                free.push(n);
            }
        }
        free
    }

    /// The board as 9 characters, cell 1 to cell 9 in order:
    /// `X` or `O` for a taken cell, and the cell's own number if it is
    /// free — so a fresh board reads `123456789`. This is what travels
    /// in a `STATE` frame.
    pub fn wire_state(&self) -> String {
        let mut cells = String::new();
        for n in 1..=9u8 {
            match self.cell(n) {
                Some(Player::X) => cells.push('X'),
                Some(Player::O) => cells.push('O'),
                None => cells.push_str(&n.to_string()),
            }
        }
        cells
    }

    /// The other way round: rebuild a game from those 9 characters.
    /// `None` if the string is not exactly 9 characters, or holds
    /// something that is neither `X`, nor `O`, nor that cell's own number.
    ///
    /// Whose turn it is does not travel: we work it out from the counts.
    /// X starts, so as many X as O means it is X's turn again.
    pub fn from_wire_state(cells: &str) -> Option<Game> {
        let chars: Vec<char> = cells.chars().collect();
        if chars.len() != 9 {
            return None;
        }

        let mut board = [None; 9];
        for i in 0..9 {
            let n = (i + 1) as u8; // the number of this cell, 1..=9
            board[i] = match chars[i] {
                'X' => Some(Player::X),
                'O' => Some(Player::O),
                // A free cell shows its own number.
                c if c.to_digit(10) == Some(n as u32) => None,
                _ => return None,
            };
        }

        // Count the marks to find out whose turn it is.
        let mut xs = 0;
        let mut os = 0;
        for cell in board {
            if cell == Some(Player::X) {
                xs += 1;
            }
            if cell == Some(Player::O) {
                os += 1;
            }
        }
        let turn = if xs == os { Player::X } else { Player::O };

        let mut game = Game { board, turn, state: State::Playing };
        game.state = game.check_state(); // won, draw or still playing?
        Some(game)
    }

    /// The one line every client prints once the game is over.
    /// `None` while it is still running.
    pub fn verdict(&self) -> Option<String> {
        match self.state {
            State::Playing => None,
            State::Won(player, line) => Some(format!(
                "{player} wins on {}-{}-{}!",
                line[0], line[1], line[2]
            )),
            State::Draw => Some("draw.".to_string()),
        }
    }

    /// The player whose turn it is plays cell `n` (1..=9). On success the
    /// turn passes to the other player and the state is updated.
    pub fn play(&mut self, n: u8) -> Result<(), PlayError> {
        if self.state != State::Playing {
            return Err(PlayError::GameOver);
        }
        if !(1..=9).contains(&n) {
            return Err(PlayError::OutOfRange);
        }
        if self.cell(n).is_some() {
            return Err(PlayError::CellTaken);
        }
        self.board[(n - 1) as usize] = Some(self.turn);
        self.state = self.check_state();
        self.turn = self.turn.other();
        Ok(())
    }

    /// Look for a winning line first, then for a full board.
    fn check_state(&self) -> State {
        for line in LINES {
            // Does either player own all three cells of this line?
            for player in [Player::X, Player::O] {
                let owns_the_line = self.cell(line[0]) == Some(player)
                    && self.cell(line[1]) == Some(player)
                    && self.cell(line[2]) == Some(player);
                if owns_the_line {
                    return State::Won(player, line);
                }
            }
        }
        if self.free_cells().is_empty() {
            return State::Draw;
        }
        State::Playing
    }
}

/// The board, drawn. A free cell shows its own number, so the board is
/// its own legend: what you see is what you type.
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..3 {
            if row > 0 {
                writeln!(f, "---+---+---")?;
            }
            for col in 0..3 {
                let n = (row * 3 + col + 1) as u8;
                let mark = match self.cell(n) {
                    Some(player) => player.to_string(),
                    None => n.to_string(),
                };
                if col > 0 {
                    write!(f, "|")?;
                }
                write!(f, " {mark} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_starts_and_turns_alternate() {
        let mut g = Game::new();
        assert_eq!(g.turn(), Player::X);
        g.play(5).unwrap();
        assert_eq!(g.turn(), Player::O);
    }

    #[test]
    fn row_column_and_diagonal_wins() {
        // X: 1 2 3 (row), O interleaves elsewhere.
        let mut g = Game::new();
        for n in [1, 4, 2, 5, 3] {
            g.play(n).unwrap();
        }
        assert_eq!(g.state(), State::Won(Player::X, [1, 2, 3]));

        // O wins column 2 5 8.
        let mut g = Game::new();
        for n in [1, 2, 3, 5, 7, 8] {
            g.play(n).unwrap();
        }
        // X played 1 3 7, O played 2 5 8.
        assert_eq!(g.state(), State::Won(Player::O, [2, 5, 8]));

        // X wins diagonal 1 5 9.
        let mut g = Game::new();
        for n in [1, 2, 5, 3, 9] {
            g.play(n).unwrap();
        }
        assert_eq!(g.state(), State::Won(Player::X, [1, 5, 9]));
    }

    #[test]
    fn full_board_without_line_is_a_draw() {
        let mut g = Game::new();
        // X O X / X O O / O X X — no line of three.
        for n in [1, 2, 3, 5, 4, 7, 8, 6, 9] {
            g.play(n).unwrap();
        }
        assert_eq!(g.state(), State::Draw);
    }

    #[test]
    fn invalid_moves_are_refused() {
        let mut g = Game::new();
        assert_eq!(g.play(0), Err(PlayError::OutOfRange));
        assert_eq!(g.play(10), Err(PlayError::OutOfRange));
        g.play(5).unwrap();
        assert_eq!(g.play(5), Err(PlayError::CellTaken));
    }

    #[test]
    fn no_move_after_game_over() {
        let mut g = Game::new();
        for n in [1, 4, 2, 5, 3] {
            g.play(n).unwrap();
        }
        assert_eq!(g.play(9), Err(PlayError::GameOver));
    }

    #[test]
    fn free_cells_shrink_as_the_board_fills() {
        let mut g = Game::new();
        assert_eq!(g.free_cells().len(), 9);
        g.play(5).unwrap();
        assert!(!g.free_cells().contains(&5));
        assert_eq!(g.free_cells().len(), 8);
    }

    #[test]
    fn verdict_lines_are_pinned() {
        let mut g = Game::new();
        assert_eq!(g.verdict(), None);
        for n in [1, 4, 2, 5, 3] {
            g.play(n).unwrap();
        }
        assert_eq!(g.verdict(), Some("X wins on 1-2-3!".to_string()));
        let mut g = Game::new();
        for n in [1, 2, 3, 5, 4, 7, 8, 6, 9] {
            g.play(n).unwrap();
        }
        assert_eq!(g.verdict(), Some("draw.".to_string()));
    }

    #[test]
    fn wire_state_of_a_fresh_board_is_the_numbers() {
        let g = Game::new();
        assert_eq!(g.wire_state(), "123456789");
    }

    #[test]
    fn wire_state_marks_x_and_o() {
        // X plays 1, O plays 3 -> "X2O456789".
        let mut g = Game::new();
        g.play(1).unwrap();
        g.play(3).unwrap();
        assert_eq!(g.wire_state(), "X2O456789");
    }

    #[test]
    fn wire_state_golden_strings() {
        // The golden boards: a fresh game, then "X plays 5" — the same
        // strings the e2e script looks for on the wire.
        let mut g = Game::new();
        assert_eq!(g.wire_state(), "123456789");
        g.play(5).unwrap();
        assert_eq!(g.wire_state(), "1234X6789");
    }

    #[test]
    fn from_wire_state_round_trips_wire_state() {
        // Play a handful of moves, then check that the board survives the
        // wire_state -> from_wire_state -> wire_state round trip.
        let mut g = Game::new();
        for n in [1, 3, 5, 7] {
            g.play(n).unwrap();
        }
        let wire = g.wire_state();
        let back = Game::from_wire_state(&wire).unwrap();
        assert_eq!(back.wire_state(), wire);
    }

    #[test]
    fn from_wire_state_works_out_the_turn() {
        // Fresh board: nobody has played, so it is X's turn.
        assert_eq!(
            Game::from_wire_state("123456789").unwrap().turn(),
            Player::X
        );
        // One X move: X has one more mark than O, so it is O's turn.
        assert_eq!(
            Game::from_wire_state("1234X6789").unwrap().turn(),
            Player::O
        );
        // One X and one O: equal counts, back to X.
        assert_eq!(
            Game::from_wire_state("X2O456789").unwrap().turn(),
            Player::X
        );
    }

    #[test]
    fn from_wire_state_reads_a_finished_board() {
        // X owns 1-2-3: the parser must see the win, not Playing.
        let g = Game::from_wire_state("XXXOO6789").unwrap();
        assert_eq!(g.state(), State::Won(Player::X, [1, 2, 3]));
    }

    #[test]
    fn from_wire_state_rejects_bad_strings() {
        // Wrong length.
        assert!(Game::from_wire_state("").is_none());
        assert!(Game::from_wire_state("12345678").is_none());
        assert!(Game::from_wire_state("1234567890").is_none());
        // A free cell must show its OWN number — "9" in cell 1 is invalid.
        assert!(Game::from_wire_state("923456789").is_none());
        // Unknown character.
        assert!(Game::from_wire_state("12345678?").is_none());
    }
}
