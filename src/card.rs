use std::fmt;

use crate::card::Rank::Number;
//tels Rust that we can treat them
// like primatives (kinda not rlly)
// an enum is a custom data type
#[derive(Copy, Clone, Debug)]
pub enum Rank {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}
#[derive(Copy, Clone, Debug)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}
//for the toString
impl fmt::Display for Rank {
    // fn fmt is the name of the function to display
    // &self gives it the rank object that called this (read only)\
    // f: &mut fmt::Formatter<'_> is the output buffer (actually writes the string)
    // fmt::Result is an enum that gives ok or an error
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rank::Number(num) => write!(f, "{}", num),
            Rank::Jack => write!(f, "Jack"),
            Rank::Queen => write!(f, "Queen"),
            Rank::King => write!(f, "King"),
            Rank::Ace => write!(f, "Ace"),
        }
    }
}
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Spades => write!(f, "♠"),
            Suit::Hearts => write!(f, "♥"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Clubs => write!(f, "♣"),
        }
    }
}
//holds the enums
#[derive(Copy, Clone)] //add this bc we need to copy card in our shuffle method
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    //like a contstructor
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }
    //to String method ... debating implimenting
    // this here, may remove
    pub fn to_string(&self) -> String {
        format!("{} of {}", self.rank, self.suit)
    }
    //will for card coutning
    pub fn get_count(&self) -> i32 {
        match self.rank {
            Rank::Number(2..=6) => 1,
            Rank::Number(7..=9) => 0,
            Rank::Number(10) => -1,
            Rank::Ace | Rank::Jack | Rank::King | Rank::Queen => -1,
            _ => 0,
        }
    }
    pub fn value(&self) -> u8 {
        match self.rank {
            Rank::Number(n) => n,
            Rank::Jack | Rank::Queen | Rank::King => 10,
            Rank::Ace => 11,
        }
    }
    //aces are weird and I need to handle that now
    pub fn is_ace(&self) -> bool {
        match self.rank {
            Rank::Ace => true,
            _ => false,
        }
    }
}
