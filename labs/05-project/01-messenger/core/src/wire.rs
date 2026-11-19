//! The wire format: how a message becomes a line of text, and back.
//!
//! One message per line, plain ASCII, `\n` at the end. Names (nicks and
//! rooms) are one word; only the LAST field of a line may hold spaces.
//!
//! ```text
//! client -> server                  server -> client
//! HELLO <nick>                      WELCOME <text...>
//! JOIN <room>                       MSG <room> <from> <text...>
//! MSG <room> <from> <text...>       DM <from> <to> <text...>
//! DM <from> <to> <text...>          ERROR <text...>
//! BYE
//! ```
//!
//! Encoding always works. Decoding returns `None` when the line makes no
//! sense. What to DO about such a line is the caller's policy, not this
//! file's: the messenger server hangs up on it.

use crate::frame::{Request, Update, is_valid_name};

/// A `Request` as the exact bytes to send.
///
/// The `\n` is part of the returned string on purpose: the caller writes
/// the whole frame in ONE go, so the newline never travels on its own.
pub fn encode_request(request: &Request) -> String {
    let line = match request {
        Request::Hello { nick } => format!("HELLO {nick}"),
        Request::Join { room } => format!("JOIN {room}"),
        Request::Msg { room, from, text } => format!("MSG {room} {from} {text}"),
        Request::Dm { from, to, text } => format!("DM {from} {to} {text}"),
        Request::Bye => "BYE".to_string(),
    };
    format!("{line}\n")
}

/// One line from the wire as a `Request`. `None` if it is not one of the
/// five frames a client may send.
///
/// The line comes without its `\n` (that is what `BufRead::lines` gives).
pub fn decode_request(line: &str) -> Option<Request> {
    // split_once cuts at the FIRST space: (verb, everything after it).
    let (verb, rest) = match line.split_once(' ') {
        Some((verb, rest)) => (verb, rest),
        None => (line, ""), // no space: the whole line is the verb
    };

    match verb {
        "HELLO" => {
            let nick = rest.trim();
            if !is_valid_name(nick) {
                return None;
            }
            Some(Request::Hello { nick: nick.to_string() })
        }
        "JOIN" => {
            let room = rest.trim();
            if !is_valid_name(room) {
                return None;
            }
            Some(Request::Join { room: room.to_string() })
        }
        "MSG" => {
            let Some((room, from, text)) = two_names_and_text(rest) else {
                return None;
            };
            Some(Request::Msg { room, from, text })
        }
        "DM" => {
            let Some((from, to, text)) = two_names_and_text(rest) else {
                return None;
            };
            Some(Request::Dm { from, to, text })
        }
        "BYE" => Some(Request::Bye),
        _ => None,
    }
}

/// An `Update` as the exact bytes to send, `\n` included.
pub fn encode_update(update: &Update) -> String {
    let line = match update {
        Update::Welcome { text } => format!("WELCOME {text}"),
        Update::Msg { room, from, text } => format!("MSG {room} {from} {text}"),
        Update::Dm { from, to, text } => format!("DM {from} {to} {text}"),
        Update::Error { text } => format!("ERROR {text}"),
    };
    format!("{line}\n")
}

/// One line from the wire as an `Update`. `None` if we cannot read it.
pub fn decode_update(line: &str) -> Option<Update> {
    let (verb, rest) = match line.split_once(' ') {
        Some((verb, rest)) => (verb, rest),
        None => (line, ""), // no space: the whole line is the verb
    };

    match verb {
        "WELCOME" => Some(Update::Welcome { text: rest.to_string() }),
        "MSG" => {
            let Some((room, from, text)) = two_names_and_text(rest) else {
                return None;
            };
            Some(Update::Msg { room, from, text })
        }
        "DM" => {
            let Some((from, to, text)) = two_names_and_text(rest) else {
                return None;
            };
            Some(Update::Dm { from, to, text })
        }
        "ERROR" => Some(Update::Error { text: rest.to_string() }),
        _ => None,
    }
}

/// The shape shared by `MSG` and `DM`: two one-word names, then free text.
/// The text may be empty and keeps every space it has.
fn two_names_and_text(rest: &str) -> Option<(String, String, String)> {
    // splitn(3, ' '): at most 3 pieces — the last one keeps its spaces.
    let mut parts = rest.splitn(3, ' ');
    let first = parts.next().unwrap_or("");
    let second = parts.next().unwrap_or("");
    let text = parts.next().unwrap_or("");

    if !is_valid_name(first) || !is_valid_name(second) {
        return None;
    }
    Some((first.to_string(), second.to_string(), text.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Golden vectors, client -> server: the exact bytes on the wire, and
    /// the same bytes read back.
    #[test]
    fn request_golden_vectors() {
        let cases = [
            (Request::Hello { nick: "alice".into() }, "HELLO alice\n"),
            (Request::Join { room: "dev".into() }, "JOIN dev\n"),
            (
                Request::Msg {
                    room: "lobby".into(),
                    from: "bob".into(),
                    text: "hello world".into(),
                },
                "MSG lobby bob hello world\n",
            ),
            (
                Request::Dm {
                    from: "carol".into(),
                    to: "alice".into(),
                    text: "psst".into(),
                },
                "DM carol alice psst\n",
            ),
            (Request::Bye, "BYE\n"),
        ];
        for (request, wire) in cases {
            assert_eq!(encode_request(&request), wire);
            // lines() hands us the line without its '\n'.
            assert_eq!(decode_request(wire.trim_end_matches('\n')), Some(request));
        }
    }

    /// Golden vectors, server -> client.
    #[test]
    fn update_golden_vectors() {
        let cases = [
            (
                Update::Welcome { text: "hi alice, you are in #lobby".into() },
                "WELCOME hi alice, you are in #lobby\n",
            ),
            (
                Update::Msg {
                    room: "lobby".into(),
                    from: "server".into(),
                    text: "bob joined".into(),
                },
                "MSG lobby server bob joined\n",
            ),
            (
                Update::Dm {
                    from: "carol".into(),
                    to: "alice".into(),
                    text: "psst".into(),
                },
                "DM carol alice psst\n",
            ),
            (
                Update::Error { text: "no such user: bob".into() },
                "ERROR no such user: bob\n",
            ),
        ];
        for (update, wire) in cases {
            assert_eq!(encode_update(&update), wire);
            assert_eq!(decode_update(wire.trim_end_matches('\n')), Some(update));
        }
    }

    #[test]
    fn text_keeps_its_spaces_and_may_be_empty() {
        assert_eq!(
            decode_request("MSG lobby bob   spaced   out  "),
            Some(Request::Msg {
                room: "lobby".into(),
                from: "bob".into(),
                text: "  spaced   out  ".into()
            })
        );
        // No text at all is allowed (an empty message).
        assert_eq!(
            decode_request("MSG lobby bob"),
            Some(Request::Msg {
                room: "lobby".into(),
                from: "bob".into(),
                text: String::new()
            })
        );
    }

    #[test]
    fn lines_we_cannot_read_give_none() {
        // Requests: unknown verbs, missing or spaced names, and the
        // server's own frames (a client may not send those).
        for line in [
            "",
            "NOPE x",
            "HELLO",
            "HELLO two words",
            "JOIN ",
            "DM onlyone",
            "WELCOME hi",
            "ERROR nope",
        ] {
            assert_eq!(decode_request(line), None, "request: {line:?}");
        }
        // Updates.
        for line in ["", "NOPE x", "HELLO alice", "JOIN dev", "BYE", "DM onlyone"] {
            assert_eq!(decode_update(line), None, "update: {line:?}");
        }
    }
}
