mod deck;
mod game;
mod player;

use game::{choose_name, handle_bets, handle_choice, handle_distribution, setup_game};

fn main() {
    // Choose a name for the player
    let name = choose_name();

    let mut tokens = 100;
    // Main game loop
    loop {
        // Initialize the game by setting up the player, dealer, and deck
        let (mut player, mut dealer, mut deck) = setup_game(&name, &tokens);

        // Handle the betting phase and capture the bet amount
        let bet = handle_bets(&mut player);

        // Distribute initial cards to player and dealer
        handle_distribution(&mut player, &mut dealer, &mut deck);

        // Handle the player's choices: hit or stand
        handle_choice(&mut player, &mut dealer, &mut deck, bet);

        // Check if the player has any tokens left
        if player.tokens.unwrap_or(0) == 0 {
            println!("\nYou have no more tokens! Game over!");
            break;
        }

        // New token amount
        println!("\nYou now have {} tokens.", player.tokens.unwrap());
        tokens = player.tokens.unwrap();

        // Ask the player if they want to play another round
        println!("Do you want to play another round? (y/N)");
        let mut answer = String::new();
        std::io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        let answer = answer.trim();

        if answer.to_lowercase() != "y" {
            println!(
                "Thanks for playing! You leave with {} tokens.",
                player.tokens.unwrap_or(0)
            );
            break;
        }
    }
}
