//! What the two sides can say to each other.
//!
//! Two directions, so two enums: a `Request` travels from a client to the
//! server, an `Update` travels back. These say WHAT is meant; turning one
//! into a line of text is `wire.rs`'s job.
//!
//! `Msg` and `Dm` exist in both enums, with the same fields. That is on
//! purpose: the sender fills `from` in a `Request`, but the server does not
//! believe it — the `from` in an `Update` is always the server's word.

/// A message a client sends to the server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    /// Introduce yourself, once. The server puts you in "lobby".
    Hello { nick: String },
    /// Join a room. Membership is additive: you stay in the others.
    Join { room: String },
    /// Say something to a room.
    Msg {
        room: String,
        from: String,
        text: String,
    },
    /// Say something to one person.
    Dm {
        from: String,
        to: String,
        text: String,
    },
    /// Leaving.
    Bye,
}

/// A message the server sends to a client.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Update {
    /// A greeting or an info text.
    Welcome { text: String },
    /// A room message. `from` is set by the server; it is "server" for
    /// notices such as "bob joined".
    Msg {
        room: String,
        from: String,
        text: String,
    },
    /// A direct message. `from` is set by the server.
    Dm {
        from: String,
        to: String,
        text: String,
    },
    /// Something you asked for was refused.
    Error { text: String },
}

/// Nicks and room names: one word, no whitespace.
pub fn is_valid_name(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    for c in s.chars() {
        if c.is_whitespace() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_validation() {
        assert!(is_valid_name("alice"));
        assert!(!is_valid_name(""));
        assert!(!is_valid_name("two words"));
    }
}
