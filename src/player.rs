use crate::card::{Card, Rank, Suit};
use rand::RngExt;
pub struct Player {
    cards: Vec<Card>,
    total: i32, //total card value
    money: i64, //decimals bad for money
    is_bot: bool,
}

impl Player {
    pub fn new(starting_money: i64, is_a_bot: bool) -> Self {
        Player {
            cards: Vec::new(),
            total: 0,
            money: starting_money,
            is_bot: is_a_bot,
        }
    }
    pub fn recieve(&mut self, card: Card) {
        self.cards.push(card);
        let mut total = 0;
        let mut aces = 0;
        //update total we need to re-add everything bc aces
        for c in &self.cards {
            total += c.value() as i32;
            if c.is_ace() {
                aces += 1;
            }
        }
        //handle aces
        while total > 21 && aces > 0 {
            total -= 10;
            aces -= 1;
        }
        self.total = total;
    }

    pub fn add_money(&mut self, money: i64) {
        if (money >= 0) {
            self.money += money;
        }
    }
    pub fn get_money(&self) -> i64 {
        self.money
    }
    pub fn remove_money(&mut self, money: i64) {
        if (money >= 0 && money <= self.money) {
            self.money -= money;
        }
    }
    pub fn is_bot(&self) -> bool {
        self.is_bot
    }
    pub fn clear_hand(&mut self) {
        self.cards.clear();
        self.total = 0
    }
    pub fn get_total(&self) -> i32 {
        self.total
    }

    pub fn show_hand(&self) -> String {
        self.cards
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn show_upcard(&self) -> String {
        self.cards[0].to_string()
    }
}
