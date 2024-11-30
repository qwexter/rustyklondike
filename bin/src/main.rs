use deck::{models::deck::Deck, models::table::Table};

fn main() {
    println!("Here will be a card game!");
    let mut deck = Deck::new();
    deck.shuffle();
    let table = Table::new(&mut deck);
    println!("{}", table);
}
