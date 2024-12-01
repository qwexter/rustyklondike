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

    pub fn can_place_on_pile(&self, pile: &Vec<Card>) -> bool {
        let pile_last_card = pile.last();
        match pile_last_card {
            Some(card) => (self.rank + 1 == card.rank) && (self.is_red() != card.is_red()),
            None => self.rank == 13,
        }
    }

    pub fn is_red(&self) -> bool {
        match self.suit {
            Suit::Hearts | Suit::Diamonds => true,
            _ => false,
        }
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

    #[test]
    fn check_that_card_red() {
        let must_be_red = Card::new(10, Suit::Hearts);
        assert!(must_be_red.is_red());
        let must_be_red = Card::new(1, Suit::Diamonds);
        assert!(must_be_red.is_red());
        let must_be_black = Card::new(10, Suit::Clubs);
        assert_eq!(false, must_be_black.is_red());
        let must_be_black = Card::new(1, Suit::Spades);
        assert_eq!(false, must_be_black.is_red());
    }

    #[test]
    fn can_place_on_pile() {
        let card_to_check = Card::new(3, Suit::Hearts);
        let pile = vec![Card::new(4, Suit::Spades)];

        assert!(card_to_check.can_place_on_pile(&pile));
    }

    #[test]
    fn cant_be_placed_on_pile_because_rank() {
        let card_to_check = Card::new(13, Suit::Hearts);
        let pile = vec![Card::new(11, Suit::Spades)];

        let can_place = card_to_check.can_place_on_pile(&pile);

        assert_eq!(false, can_place);
    }

    #[test]
    fn cant_be_placed_on_pile_because_color() {
        let card_to_check = Card::new(2, Suit::Spades);
        let pile = vec![Card::new(3, Suit::Clubs)];

        let can_place = card_to_check.can_place_on_pile(&pile);

        assert_eq!(false, can_place);
    }

    #[test]
    fn can_be_placed_on_empty_pile() {
        let card_to_check = Card::new(13, Suit::Clubs);
        let pile: Vec<Card> = vec![];

        let can_be_placed = card_to_check.can_place_on_pile(&pile);

        assert!(can_be_placed);
    }

    #[test]
    fn cant_be_placed_on_empty_pile() {
        let card_to_check = Card::new(12, Suit::Hearts);
        let pile: Vec<Card> = vec![];

        let can_be_placed = card_to_check.can_place_on_pile(&pile);

        assert_eq!(false, can_be_placed);
    }
}
