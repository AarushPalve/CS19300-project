// let rust know where our card it
mod card;
mod deck;

// give it access to the enums and impl we made
// in rust we call this brining into scope
use card::{Card, Rank, Suit};

use crate::deck::Deck;

fn main() {
    let mut deck1 = Deck::new();
    //let my_card = deck1.deal();
    // this is where the match comes in ^
    match deck1.deal() {
        Some(my_card) => println!("{}", my_card.to_string()),
        None => println!("mt deck"),
    }
    //println!("{}", my_card.to_string());
}
