use crate::deck::Card;
use crate::game::calculate_score;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    pub tokens: Option<u32>,
    pub is_dealer: bool,
    pub hidden_card: Option<Card>,
    pub is_hidden_card_revealed: bool,
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
            is_hidden_card_revealed: false,
        }
    }

    // Add a card to the player's hand.
    pub fn add_card(&mut self, card: Card) {
        if self.is_dealer {
            // Only hide the first card, and only if it hasn't been revealed yet
            if self.hidden_card.is_none() && !self.is_hidden_card_revealed {
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
            self.is_hidden_card_revealed = true; // Set the flag to prevent future hiding
        }
    }

    pub fn show_cards(&self) {
        println!("\n{}'s hand:", self.name);

        if self.is_dealer {
            // Dealer logic: show all cards except the hidden one.
            for card in &self.hand {
                // Only skip printing the hidden card
                if Some(card) == self.hidden_card.as_ref() {
                    continue; // Skip the hidden card display
                }
                println!("{}", format_card(card)); // Show each visible card
            }

            // Print the hidden card indication without showing it
            if self.hidden_card.is_some() {
                println!("(Hidden card)");
            }
        } else {
            // Player logic: show all cards.
            for card in &self.hand {
                println!("{}", format_card(card)); // Show each card
            }
        }

        // Calculate the hand value based on visible cards for both player and dealer
        let hand_value = if self.is_dealer {
            // For dealer, calculate score excluding the hidden card
            let visible_cards: Vec<Card> = self
                .hand
                .iter()
                .filter(|card| Some(card) != self.hidden_card.as_ref().as_ref())
                .cloned()
                .collect();
            calculate_score(&visible_cards)
        } else {
            // For player, use all cards
            calculate_score(&self.hand)
        };

        println!("Total value: {}\n", hand_value);
    }
}

// Helper function to format a card in a readable way, e.g., "4 of Spades"
fn format_card(card: &Card) -> String {
    format!("{} of {}", card.value, card.suit)
}
