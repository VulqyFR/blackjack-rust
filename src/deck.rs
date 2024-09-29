use rand::Rng;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

// Implementing Display allows for custom string representation
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Pattern matching for enum-to-string conversion
        let suit_str = match self {
            Suit::Spades => "Spades",
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
        };
        write!(f, "{}", suit_str)
    }
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)] // Suppresses warnings for unused fields
pub struct Card {
    pub suit: Suit,
    pub value: String,
}

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::new();
        let suits = vec![Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];
        let values = vec![
            "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King",
        ];
        // Nested loops create a full deck with all combinations
        for suit in &suits {
            for value in &values {
                cards.push(Card {
                    suit: *suit,
                    value: (*value).to_string(),
                })
            }
        }

        Deck { cards }
    }

    // Fisher-Yates shuffle: O(n) time complexity, in-place algorithm
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.cards.len() {
            // Generate random index from 0 to i (inclusive)
            let j = rng.gen_range(0..=i);
            // Swap current card with randomly selected card
            self.cards.swap(i, j);
        }
    }
}
