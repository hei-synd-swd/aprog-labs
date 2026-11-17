//! The screen: everything the player sees. The controller owns it, and it
//! never touches a bus — it only draws what it is told.
//!
//! What is on screen is the last board the server pushed. There is no game
//! being played here: each board arrives as 9 characters, gets turned into
//! a `Game` just long enough to draw it, and is then forgotten. The only
//! things this struct remembers between updates are the seat the server
//! gave us and whether the game is still running.

// `IsTerminal` is a trait: importing it is what gives `stdout()` the
// `is_terminal()` method.
use std::io::IsTerminal;

use ttt_core::{Game, Player, State, Update};

pub(crate) struct ConsoleDisplay {
    /// Which seat we are, once the server has told us.
    me: Option<Player>,
    /// Whether the last board we drew was still being played.
    playing: bool,
}

impl ConsoleDisplay {
    pub(crate) fn new() -> Self {
        ConsoleDisplay { me: None, playing: true }
    }

    /// Which seat we are, once the server has told us — the same fact
    /// `on_update` already tracks, so nobody else has to keep a copy.
    pub(crate) fn seat(&self) -> Option<Player> {
        self.me
    }

    /// Draw one update — the full 4-way breakdown of `Update` lives here,
    /// and only here. (Arriving here from `controller.rs`: its
    /// `Event::FromServer` calls this, then asks a single further
    /// question of its own — was that `Bye`? — that is the only other
    /// place anything looks inside an `Update`.)
    ///
    /// `leaving` is true when we are the one quitting, which changes
    /// nothing but the wording of the goodbye.
    pub(crate) fn on_update(&mut self, update: &Update, leaving: bool) {
        match update {
            Update::Seat(player) => {
                self.me = Some(*player);
                println!("you are {player}.");
            }
            Update::Board(cells) => {
                // A board we cannot read is ignored rather than crashing.
                let Some(game) = Game::from_wire_state(cells) else {
                    return;
                };
                self.draw(&game);
                match game.verdict() {
                    Some(line) => println!("{line}"),
                    None => self.announce_turn(&game),
                }
                self.playing = game.state() == State::Playing;
            }
            Update::Refused(refusal) => println!("({refusal})"),
            Update::Bye => {
                // Everyone gets the same Bye. Only say someone left if the
                // game was still running and we are not the one leaving.
                if !leaving && self.playing {
                    println!("peer left.");
                }
            }
        }
    }

    /// Redraw the board. On a real terminal, clear the screen first; when
    /// the output is piped to a file, leave it alone so it stays readable.
    fn draw(&self, game: &Game) {
        if std::io::stdout().is_terminal() {
            // Cursor home, then erase the screen and the scrollback. This
            // order works the same in VS Code, iTerm and Windows Terminal.
            print!("\x1b[H\x1b[2J\x1b[3J"); // \x1b is the ESC character
            println!("{game}");
        } else {
            println!("\n{game}");
        }
    }

    /// Say whose turn it is. Before the server has told us our seat we do
    /// not know which side we are, so we say nothing.
    fn announce_turn(&self, game: &Game) {
        match self.me {
            Some(seat) if seat == game.turn() => {
                println!("{} to play [1-9, q quits]:", game.turn());
            }
            Some(_) => println!("waiting for peer ({})...", game.turn()),
            None => {}
        }
    }
}
