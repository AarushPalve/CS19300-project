// let rust know where our card it
mod card;

// give it access to the enums and impl we made
// in rust we call this brining into scope
use card::{Card, Rank, Suit};

fn main() {
    let my_card = Card::new(Suit::Spades, Rank::Ace);
    println!("{}", my_card.to_string());
}
