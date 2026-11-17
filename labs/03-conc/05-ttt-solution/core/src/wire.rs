//! The wire format: how a message becomes a line of text, and back.
//!
//! One message per line, plain ASCII, `\n` at the end:
//!
//! ```text
//! client -> server            server -> client
//! HELLO                       SEAT X | SEAT O
//! MOVE <1-9>                  STATE <9 characters>
//! STATE                       REFUSED turn|range|taken|over
//! BYE                         BYE
//! ```
//!
//! Encoding always works. Decoding returns `None` when the line makes no
//! sense — there is no error frame in this protocol, so both sides simply
//! ignore a line they cannot read and carry on.

use crate::frame::{Refusal, Request, Update};
use crate::game::{PlayError, Player};

/// A `Request` as the exact bytes to send.
///
/// The `\n` is part of the returned string on purpose: the caller writes
/// the whole frame in ONE go, so the newline never travels on its own.
pub fn encode_request(request: &Request) -> String {
    let line = match request {
        Request::Hello => "HELLO".to_string(),
        Request::Move(n) => format!("MOVE {n}"),
        Request::GetBoard => "STATE".to_string(),
        Request::Bye => "BYE".to_string(),
    };
    format!("{line}\n")
}

/// One line from the wire as a `Request`. `None` if it is not one of the
/// four frames we know.
pub fn decode_request(line: &str) -> Option<Request> {
    let line = line.trim();

    // "MOVE <n>" is the only frame that carries something.
    if let Some(rest) = line.strip_prefix("MOVE ") {
        return match rest.trim().parse::<u8>() {
            Ok(n) => Some(Request::Move(n)),
            Err(_) => None,
        };
    }

    match line {
        "HELLO" => Some(Request::Hello),
        "STATE" => Some(Request::GetBoard),
        "BYE" => Some(Request::Bye),
        _ => None,
    }
}

/// An `Update` as the exact bytes to send, `\n` included.
pub fn encode_update(update: &Update) -> String {
    let line = match update {
        Update::Seat(player) => format!("SEAT {player}"),
        Update::Board(cells) => format!("STATE {cells}"),
        Update::Refused(refusal) => format!("REFUSED {}", refusal_token(refusal)),
        Update::Bye => "BYE".to_string(),
    };
    format!("{line}\n")
}

/// One line from the wire as an `Update`. `None` if we cannot read it.
///
/// The 9 board characters are passed through as they are: if they are
/// nonsense, `Game::from_wire_state` is the one that notices.
pub fn decode_update(line: &str) -> Option<Update> {
    let line = line.trim();

    if let Some(seat) = line.strip_prefix("SEAT ") {
        return match seat {
            "X" => Some(Update::Seat(Player::X)),
            "O" => Some(Update::Seat(Player::O)),
            _ => None,
        };
    }

    if let Some(cells) = line.strip_prefix("STATE ") {
        return Some(Update::Board(cells.to_string()));
    }

    if let Some(token) = line.strip_prefix("REFUSED ") {
        // Give up and return None if the word is not one of ours.
        let Some(refusal) = refusal_from_token(token) else {
            return None;
        };
        return Some(Update::Refused(refusal));
    }

    match line {
        "BYE" => Some(Update::Bye),
        _ => None,
    }
}

/// The short word each refusal travels as. The meaning crosses the wire;
/// each client puts it into its own words on screen.
fn refusal_token(refusal: &Refusal) -> &'static str {
    match refusal {
        Refusal::WrongTurn => "turn",
        Refusal::Rule(PlayError::OutOfRange) => "range",
        Refusal::Rule(PlayError::CellTaken) => "taken",
        Refusal::Rule(PlayError::GameOver) => "over",
    }
}

fn refusal_from_token(token: &str) -> Option<Refusal> {
    match token {
        "turn" => Some(Refusal::WrongTurn),
        "range" => Some(Refusal::Rule(PlayError::OutOfRange)),
        "taken" => Some(Refusal::Rule(PlayError::CellTaken)),
        "over" => Some(Refusal::Rule(PlayError::GameOver)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Golden vectors, client -> server: the exact bytes on the wire, and
    /// the same bytes read back.
    #[test]
    fn request_golden_vectors() {
        let cases = [
            (Request::Hello, "HELLO\n"),
            (Request::Move(5), "MOVE 5\n"),
            (Request::GetBoard, "STATE\n"),
            (Request::Bye, "BYE\n"),
        ];
        for (request, wire) in cases {
            assert_eq!(encode_request(&request), wire);
            assert_eq!(decode_request(wire), Some(request));
        }
    }

    /// Golden vectors, server -> client.
    #[test]
    fn update_golden_vectors() {
        let cases = [
            (Update::Seat(Player::X), "SEAT X\n"),
            (Update::Seat(Player::O), "SEAT O\n"),
            (Update::Board("X2O456789".to_string()), "STATE X2O456789\n"),
            (Update::Refused(Refusal::WrongTurn), "REFUSED turn\n"),
            (
                Update::Refused(Refusal::Rule(PlayError::OutOfRange)),
                "REFUSED range\n",
            ),
            (
                Update::Refused(Refusal::Rule(PlayError::CellTaken)),
                "REFUSED taken\n",
            ),
            (
                Update::Refused(Refusal::Rule(PlayError::GameOver)),
                "REFUSED over\n",
            ),
            (Update::Bye, "BYE\n"),
        ];
        for (update, wire) in cases {
            assert_eq!(encode_update(&update), wire);
            assert_eq!(decode_update(wire), Some(update));
        }
    }

    #[test]
    fn move_parses_a_plain_number() {
        // A leading '+' is accepted when parsing; encoding writes it back
        // without one.
        assert_eq!(decode_request("MOVE +5"), Some(Request::Move(5)));
        // Spaces around the line are trimmed.
        assert_eq!(decode_request("  MOVE 9  "), Some(Request::Move(9)));
    }

    #[test]
    fn lines_we_cannot_read_give_none() {
        // Requests.
        for line in [
            "",
            "NOPE",
            "MOVE",
            "MOVE x",
            "MOVE -1",
            "MOVE 9 9",
            "STATE X2O456789",
        ] {
            assert_eq!(decode_request(line), None, "request: {line:?}");
        }
        // Updates.
        for line in [
            "",
            "NOPE",
            "STATE",
            "MOVE 5",
            "REFUSED",
            "REFUSED nope",
            "SEAT",
            "SEAT ?",
        ] {
            assert_eq!(decode_update(line), None, "update: {line:?}");
        }
    }
}
