use crate::deck::Deck;
use crate::player::Player;

pub struct Table {
    players: Vec<Player>,
    deck: Deck,
    //count tracks the count of the info the player has
    count: i32,
    //pending_count tracks the count of the info the player doesn't have/will get at the end of the round
    pending_count: i32,
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
            pending_count: 0,
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
    //need to hide 1/2 cards the dealer gets
    fn deal_hidden_to_dealer(&mut self) {
        let card = self.deck.deal().unwrap();
        self.pending_count += card.get_count();
        self.players[0].recieve(card);
    }
    pub fn get_count(&self) -> i32 {
        self.count
    }
    pub fn start_round(&mut self) {
        //reset all the player's hands and game states
        self.pending_count = 0;
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
        // deal the first card face down
        for pass in 0..2 {
            for i in 1..self.players.len() {
                self.deal_to_player(i);
            }
            if pass == 0 {
                self.deal_to_player(0);
            } else {
                self.deal_hidden_to_dealer();
            }
        }
    }

    //bc the player can't see the dealer's card
    pub fn reveal_dealer(&mut self) {
        self.count += self.pending_count;
        self.pending_count = 0;
    }
}
