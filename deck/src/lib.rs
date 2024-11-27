#[derive(Debug, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    rank: u8, // 1-13 Ace->2->3...King
    suit: Suit,
    is_showed: bool,
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

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for rank in 1..=13 {
            cards.push(Card::new(rank, Suit::Hearts));
            cards.push(Card::new(rank, Suit::Diamonds));
            cards.push(Card::new(rank, Suit::Spades));
            cards.push(Card::new(rank, Suit::Clubs));
        }
        Deck { cards: cards }
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
    fn check_deck_size() {
        let deck = Deck::new();

        assert_eq!(deck.cards.len(), 52);
    }
}
