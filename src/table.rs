use crate::deck::Deck;
use crate::player::Player;

pub struct Table {
    players: Vec<Player>,
    deck: Deck,
    count: i32,
}

impl Table {
    pub fn new() -> Self {
        Table {
            // treat the dealer as a bot w/o any money while initialiing the playerlist
            // we say the 0th player is the dealer
            // index one will be the player
            players: vec![Player::new(0, true)],
            deck: Deck::new(),
            count: 0,
        }
    }
    //no remove players bc lazyness
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
    pub fn get_player(&self, player_index: usize) -> &Player {
        &self.players[player_index]
    }
    pub fn deal_to_player(&mut self, player_index: usize) {
        let card = self.deck.deal().unwrap();
        self.count += card.get_count();
        self.players[player_index].recieve(card);
    }
    pub fn get_count(&self) -> i32 {
        self.count
    }
    pub fn start_round(&mut self) {
        //reset all the player's hands
        for player in &mut self.players {
            player.clear_hand();
        }
        // shuffle the deck between rounds
        if self.deck.needs_shuffling() {
            //might change
            println!("Shuffled deck");
            self.deck = Deck::new();
            self.count = 0;
        }

        //deal in the way blackjack is delt
        //start left of the dealer then the dealer
        for _ in 0..2 {
            for i in 1..self.players.len() {
                self.deal_to_player(i);
            }
            self.deal_to_player(0);
        }
    }
}
