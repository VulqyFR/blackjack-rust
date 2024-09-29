use std::process;

use crate::deck::Deck;
use crate::player::Player;

/// Initialize the game by creating the player, dealer, and shuffling the deck.
pub fn setup_game(name: &String, tokens: &u32) -> (Player, Player, Deck) {
    let name = name.clone();
    let start = std::time::Instant::now();
    let mut deck = Deck::new();
    deck.shuffle();

    // Initialize player with Some(100) tokens
    let player = Player::new(name, Some(tokens.clone()), false);
    // Initialize dealer with None tokens (dealer doesn't use tokens)
    let dealer = Player::new("Dealer".to_string(), None, true);

    println!("Game setup in {:?} \n", start.elapsed());

    (player, dealer, deck)
}

pub fn choose_name() -> String {
    println!("Welcome to the game of Blackjack! Please enter your name:");

    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Hello, {}! Let's start the game! \n", name.trim());

    name.trim().to_string()
}

/// Handle the betting phase. Prompt the player for a bet and return the bet amount.
pub fn handle_bets(player: &mut Player) -> u32 {
    // Check if player has tokens
    match player.tokens {
        Some(0) => {
            println!("You have no more tokens! Game over!");
            process::exit(0);
        }
        Some(_) => {} // Player has tokens
        None => {
            println!("Dealer does not have tokens.");
            process::exit(0);
        }
    }

    println!(
        "You currently have {:?} tokens. Type ':q' to quit the game.",
        player.tokens.unwrap()
    );

    let mut bet = String::new();
    std::io::stdin()
        .read_line(&mut bet)
        .expect("Failed to read line");

    let bet = bet.trim();

    if bet == ":q" {
        process::exit(0);
    }

    match bet.parse::<u32>() {
        Ok(num) if num <= player.tokens.unwrap() => {
            // Deduct the bet from player's tokens
            player.tokens = Some(player.tokens.unwrap() - num);
            println!(
                "You bet {} tokens, resulting in a new total of {} tokens.\n ",
                num,
                player.tokens.unwrap()
            );
            num // Return the bet value for use in the game
        }
        Ok(_) => {
            println!("You cannot bet more tokens than you have!");
            handle_bets(player) // Retry on invalid bet
        }
        Err(_) => {
            println!("Please type a valid number!");
            handle_bets(player) // Retry on invalid input
        }
    }
}

// Handle the initial distribution of cards to the player and the dealer.
pub fn handle_distribution(player: &mut Player, dealer: &mut Player, deck: &mut Deck) {
    // Deal two cards to player and dealer
    player.add_card(deck.cards.pop().unwrap());
    dealer.add_card(deck.cards.pop().unwrap()); // Dealer's first card is hidden
    player.add_card(deck.cards.pop().unwrap());
    dealer.add_card(deck.cards.pop().unwrap()); // Dealer's second card is visible

    // Show the player's cards
    player.show_cards();

    // Show dealer's cards, indicating the hidden one
    dealer.show_cards(); // This shows the hidden card
}

// Handle the player's choices: hit or stand.
// Takes the bet as a parameter to adjust tokens in check_win.
pub fn handle_choice(player: &mut Player, dealer: &mut Player, deck: &mut Deck, bet: u32) {
    loop {
        println!("What do you want to do? (h)it or (s)tand? Type ':q' to quit the game.\n");
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "h" => {
                // Player chooses to hit: add a card
                player.add_card(deck.cards.pop().unwrap());
                player.show_cards(); // Show updated player hand

                // Check if player busted
                if calculate_score(&player.hand) > 21 {
                    println!("You busted!");
                    check_win(player, dealer, bet);
                    return; // End the player's turn
                }
            }
            "s" => {
                // Player chooses to stand: dealer reveals hidden card and plays
                dealer.reveal_dealer_card(); // Reveal the hidden card
                dealer.show_cards(); // Show dealer's cards

                // Dealer hits until reaching at least 17
                while calculate_score(&dealer.hand) < 17 {
                    dealer.add_card(deck.cards.pop().unwrap());
                    println!("\nDealer hits!\n");
                    dealer.show_cards(); // Show dealer's cards after hitting
                }

                // Determine the winner
                check_win(player, dealer, bet);
                return; // End the player's turn
            }
            ":q" => {
                // Quit the game
                process::exit(0);
            }
            _ => {
                // Invalid choice: prompt again
                println!("Invalid choice! Please type 'h' to hit or 's' to stand or :q to quit the game.");
                continue;
            }
        }
    }
}

/// Determine the winner and adjust player's tokens based on the bet.
fn check_win(player: &mut Player, dealer: &mut Player, bet: u32) {
    let player_score = calculate_score(&player.hand);
    let dealer_score = calculate_score(&dealer.hand);

    // Player busts: dealer wins
    if player_score > 21 {
        println!("You busted! Dealer wins!");
        return;
    }

    // Dealer busts: player wins double the bet
    if dealer_score > 21 {
        println!("Dealer busted! You win {} tokens.", bet * 2);
        player.tokens = Some(player.tokens.unwrap() + bet * 2);
        return;
    }

    // Push: player gets their bet back
    if player_score == dealer_score {
        println!("It's a push!");
        player.tokens = Some(player.tokens.unwrap() + bet);
        return;
    }

    // Player wins: gains double the bet
    if player_score > dealer_score {
        println!("You win! You gain {} tokens.", bet * 2);
        player.tokens = Some(player.tokens.unwrap() + bet * 2);
    } else {
        // Dealer wins: player already lost their bet
        println!("Dealer wins!");
    }
}

// Calculate the score of a hand.
pub fn calculate_score(hand: &Vec<crate::deck::Card>) -> u8 {
    let mut score: u8 = 0;
    let mut number_of_aces: u8 = 0;

    for card in hand {
        match card.value.as_str() {
            "Ace" => {
                number_of_aces += 1;
                score += 11;
            }
            "Jack" | "Queen" | "King" => score += 10,
            _ => score += card.value.parse::<u8>().unwrap(),
        }
    }

    // Adjust for aces if score > 21
    while score > 21 && number_of_aces > 0 {
        score -= 10;
        number_of_aces -= 1;
    }

    score
}
