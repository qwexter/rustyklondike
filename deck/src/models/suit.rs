use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = match &self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Spades => "♠",
            Suit::Clubs => "♣",
        };
        write!(f, "{}", icon)
    }
}
