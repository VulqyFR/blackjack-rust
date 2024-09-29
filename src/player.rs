use crate::deck::Card;
use crate::game::calculate_score;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    pub tokens: Option<u32>,
    pub is_dealer: bool,
    pub hidden_card: Option<Card>,
}

impl Player {
    // Create a new player or dealer.
    pub fn new(name: String, tokens: Option<u32>, is_dealer: bool) -> Player {
        Player {
            name,
            hand: Vec::new(),
            tokens,
            is_dealer,
            hidden_card: None,
        }
    }

    // Add a card to the player's hand.
    pub fn add_card(&mut self, card: Card) {
        if self.is_dealer {
            // Only set the first card as hidden
            if self.hidden_card.is_none() {
                self.hidden_card = Some(card.clone());
            } else {
                // Add subsequent cards normally
                self.hand.push(card.clone());
            }
        } else {
            // For players, just add the card to the hand
            self.hand.push(card.clone());
        }
    }

    // Reveal the dealer's hidden card.
    pub fn reveal_dealer_card(&mut self) {
        if self.is_dealer && self.hidden_card.is_some() {
            // Reveal hidden card and add to the dealer's hand
            if let Some(card) = self.hidden_card.take() {
                println!("\nDealer's hidden card revealed: {}", format_card(&card));
                self.hand.push(card);
            }
        }
    }

    pub fn show_cards(&self) {
        println!("\n{}'s hand:", self.name);

        // Create a temporary vector to store the visible cards
        let mut visible_cards: Vec<Card> = Vec::new();

        if self.is_dealer {
            // Show only visible cards
            for card in &self.hand {
                if let Some(hidden_card) = &self.hidden_card {
                    // Skip adding the hidden card to visible_cards
                    if *card == *hidden_card {
                        continue;
                    }
                }
                // Push a clone of the card to visible_cards
                visible_cards.push(card.clone());
                println!("{}", format_card(card)); // Show each visible card
            }

            // Print the hidden card indication without showing it
            if self.hidden_card.is_some() {
                println!("(Hidden card)");
            }
        } else {
            // Show all cards for player
            for card in &self.hand {
                println!("{}", format_card(card));
                visible_cards.push(card.clone()); // Add to visible cards for player
            }
        }

        // Calculate the hand value based on visible cards
        let hand_value = calculate_score(&visible_cards);
        println!("Total value: {}\n", hand_value);
    }
}

// Helper function to format a card in a readable way, e.g., "4 of Spades"
fn format_card(card: &Card) -> String {
    format!("{} of {}", card.value, card.suit)
}
