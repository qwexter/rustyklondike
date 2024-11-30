use crate::Suit;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    pub rank: u8, // 1-13 Ace->2->3...King
    pub suit: Suit,
    pub is_showed: bool,
}

impl Card {
    pub fn new(rank: u8, suit: Suit) -> Self {
        let card_range = 1..=13;
        assert!(card_range.contains(&rank), "rank must be in range of cards");
        Self {
            rank: rank,
            suit: suit,
            is_showed: false,
        }
    }

    pub fn flip(&mut self) {
        self.is_showed = !self.is_showed;
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = if self.is_showed {
            match self.rank {
                1 => "A".to_string(),
                11 => "J".to_string(),
                12 => "Q".to_string(),
                13 => "K".to_string(),
                _ => self.rank.to_string(),
            }
        } else {
            "XX".to_string()
        };

        if self.is_showed {
            write!(f, "{}{}", text, self.suit)
        } else {
            write!(f, "{}", text)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_card_in_range() {
        let suit = Suit::Hearts;
        let rank = 1; // check an ace

        let result = Card::new(rank, suit);

        let expected = Card {
            rank: 1,
            suit: Suit::Hearts,
            is_showed: false,
        };
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn check_card_out_of_range() {
        Card::new(100, Suit::Diamonds);
    }

    #[test]
    fn check_flip() {
        let mut card = Card::new(1, Suit::Diamonds);
        card.flip();
        assert_eq!(card.is_showed, true);
    }
}
