use crate::models::deck::Deck;
use crate::Card;
use std::fmt;

#[derive(Debug)]
pub struct Table {
    pub piles: Vec<Vec<Card>>,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.piles.len() {
            write!(f, "{:^6}", i + 1)?;
            write!(f, " ")?;
        }
        writeln!(f)?;
        writeln!(f)?;

        for i in 0..7 {
            for pile in &self.piles {
                if i < pile.len() {
                    let card = pile[i].to_string();
                    write!(f, "{:^6}", card)?;
                } else {
                    write!(f, "  --  ")?;
                }
                write!(f, " ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Table {
    pub fn new(deck: &mut Deck) -> Table {
        let mut piles = Vec::with_capacity(7);

        for i in 0..7 {
            let mut pile = Vec::new();
            for j in 0..=i {
                if let Some(mut card) = deck.draw() {
                    if j == i {
                        card.flip();
                    }
                    pile.push(card);
                }
            }
            piles.push(pile);
        }
        Table { piles: piles }
    }
}
