use crate::Card;
use crate::Suit;
use rand::{seq::SliceRandom, thread_rng};

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

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_deck_size() {
        let deck = Deck::new();

        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn check_deck_draw() {
        let mut deck = Deck::new();
        let card = deck.draw();
        let expected = Card::new(13, Suit::Clubs);
        assert!(card.is_some());
        assert_eq!(expected, card.unwrap());
    }
}
