use crate::card::{Card, Rank, Suit};
use rand::RngExt;
pub struct Player {
    cards: Vec<Card>,
    total: i32,
    money: i64, //decimals bad for money
}

impl Player {
    pub fn new(starting_money: i64) -> Self {
        Player {
            cards: Vec::new(),
            total: 0,
            money: starting_money,
        }
    }
    pub fn recieve(&card: Card) {
        self.total += card.value() as i64;
        self.cards.push(card);
    }
    pub fn add_Money(&mut self, money: i64) {
        if (money >= 0) {
            self.money += money;
        }
    }
    pub fn get_Money(&self) -> i64 {
        self.money
    }
    pub fn remove_Money(&mut self, money: i64) {
        if (money >= 0 && money <= self.money) {
            self.money -= money;
        }
    }
}
