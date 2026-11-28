use crate::card::{Card, Rank, Suit};
use rand::RngExt;
pub struct Deck {
    playable_cards: Vec<Card>,
    played_cards: Vec<Card>,
    total_cards: usize,
}

impl Deck {
    pub fn new(num_decks: usize) -> Self {
        let mut playable_cards = Vec::with_capacity(52 * num_decks);
        //even tho we mutate later we don't act need to give played_cards the mut keyword
        let played_cards = Vec::with_capacity(52);
        let suits = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];
        let face_ranks = [Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];
        //act account for num decks now
        for _ in 0..num_decks {
            for suit in suits {
                for val in 2..=10 {
                    playable_cards.push(Card::new(suit, Rank::Number(val)));
                }
                for rank in face_ranks {
                    playable_cards.push(Card::new(suit, rank));
                }
            }
        }
        //need to return the deck struct
        // usually rust is smart enough to
        // auto return stuff but not in this case
        let total_cards = 52 * num_decks;
        Deck {
            playable_cards,
            played_cards,
            total_cards,
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
    //this is useful for shoe logic now
    // w/ 75% pen
    pub fn needs_shuffling(&self) -> bool {
        self.played_cards.len() * 4 >= self.total_cards * 3
    }
    pub fn cards_remaining(&self) -> usize {
        self.playable_cards.len()
    }

    pub fn decks_remaining(&self) -> f32 {
        self.playable_cards.len() as f32 / 52.0
    }
}
