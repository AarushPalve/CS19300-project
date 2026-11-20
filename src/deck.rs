use crate::card::{Card, Rank, Suit};
use rand::RngExt;
pub struct Deck {
    playable_cards: Vec<Card>,
    played_cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut playable_cards = Vec::with_capacity(52);
        //even tho we mutate later we don't act need to give played_cards the mut keyword
        let played_cards = Vec::with_capacity(52);
        let suits = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];
        let face_ranks = [Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];
        for suit in suits {
            for val in 2..=10 {
                playable_cards.push(Card::new(suit, Rank::Number(val)));
            }
            for rank in face_ranks {
                playable_cards.push(Card::new(suit, rank));
            }
        }
        //need to return the deck struct
        // usually rust is smart enought to
        // auto return stuff but not in this case
        Deck {
            playable_cards,
            played_cards,
        }
    }
    //am lazy and don't want to write a shuffle method
    // so I will be picking a random card to draw each time
    // we need to return Option<Card> bc the deck might be mt
    // we js need to remember to use a match when drawing a card
    pub fn deal(&mut self) -> Option<Card> {
        if (self.playable_cards.is_empty()) {
            return None;
        }
        //kindof a long line but Rng::thread_rng() is like the random object
        // then we get a random index value
        // this simulates shuffling rather than having to act shuffle
        let index = rand::rng().random_range(0..self.playable_cards.len());
        let card = self.playable_cards.swap_remove(index);
        //add to discard, we will need for card counting mechs later?
        self.played_cards.push(card);
        //just telling the compiler we will have a card val to return her
        return Some(card);
    }
    //this might be useful for shoe logic later
    pub fn is_empty(self) -> bool {
        self.playable_cards.is_empty()
    }
}
