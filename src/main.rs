pub mod card;
use std::cmp::Ordering;
use clap::Parser;

/// Generate Texas Hold'em Poker hands and check for winner.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of hands to generate
    #[arg(short, long, default_value_t = 1)]
    count: u32,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,

   //TODO vary number of players
}

fn main() {
    let args = Args::parse();

    for n in 0..args.count {
        if args.count > 1 {
            println!("Deal {}", n+1);
        }

        let mut d = card::Deck::new();
        d.shuffle();

        // draw hands
        let mut h1 = card::Hand::new();
        let mut h2 = card::Hand::new();
        for _ in 0..2 {
            if let Some(card) = d.draw() {
                h1.add( card );
            }
            if let Some(card) = d.draw() {
                h2.add( card );
            }
        }

        // dealer cards
        let mut hd = card::Hand::new();
        for _ in 0..5 {
            if let Some(card) = d.draw() {
                hd.add( card );
            }
        }

        println!("Player 1: {}", h1.to_string());
        println!("Player 2: {}", h2.to_string());
        println!("Dealer: {}", hd.to_string());

        let show1 = card::find_best_hand( h1, hd.clone(), args.verbose );
        println!("  Best hand for Player 1: {}", show1.to_string());
        let show2 = card::find_best_hand( h2, hd, args.verbose );
        println!("  Best hand for Player 2: {}", show2.to_string());
        let res = match show1.cmp(&show2) {
            Ordering::Greater => String::from("Player 1 wins."),
            Ordering::Equal   => String::from("TIED"),
            Ordering::Less    => String::from("Player 2 wins."),
        };
        println!("{}", res);
    }
}
