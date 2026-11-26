mod card;
mod deck;
mod player;
mod table;

use crate::player::Player;
use crate::table::Table;
use std::io;
//temp prog for testing
fn main() {
    let num_decks: usize = loop {
        println!("Num decks? (1-6)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<usize>() {
            Ok(n) if (1..=6).contains(&n) => break n,
            _ => println!("Enter a number between 1 and 6."),
        }
    };
    let mut table = Table::new(num_decks);
    table.add_player(Player::new(100, false));
    loop {
        table.start_round();
        table.reveal_dealer();
        while table.get_player(1).get_total() < 21 {
            println!(
                "Dealer: {} | You: {} ({}) | Count: {}",
                table.get_player(0).show_upcard(),
                table.get_player(1).show_hand(),
                table.get_player(1).get_total(),
                table.get_count()
            );
            println!("(h)it or (s)tand?");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "h" {
                table.deal_to_player(1);
            } else {
                break;
            }
        }
        let p_total = table.get_player(1).get_total();
        if p_total > 21 {
            println!(
                "Bust:( ({}) | Dealer had: {} | Count: {}",
                p_total,
                table.get_player(0).show_hand(),
                table.get_count()
            );
        } else {
            while table.get_player(0).get_total() < 17 {
                table.deal_to_player(0);
            }
            let d_total = table.get_player(0).get_total();
            println!("Dealer: {} ({})", table.get_player(0).show_hand(), d_total);

            if d_total > 21 || p_total > d_total {
                println!("You win!");
            } else if p_total < d_total {
                println!("Dealer wins.");
            } else {
                println!("Push.");
            }
        }
        println!("Play again? (y/n)");
        let mut again = String::new();
        io::stdin().read_line(&mut again).unwrap();
        if again.trim() != "y" {
            break;
        }
    }
}
