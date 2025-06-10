use rand::seq::SliceRandom;
use std::io::{self, Write};

#[derive(Clone, Debug)]
enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Spades => write!(f, "♠"),
            Suit::Hearts => write!(f, "♥"),
            Suit::Clubs => write!(f, "♣"),
            Suit::Diamonds => write!(f, "♦"),
        }
    }
}

#[derive(Clone, Debug)]
struct Card {
    suit: Suit,
    rank: String,  // 2, 3, 4, 5, 6, 7, 8, 9, 10, J, Q, K, A
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>2}{}", self.rank, self.suit)
    }
}

// Placeholder for Joker struct
#[derive(Clone)]
struct Joker {
    name: String,
}

struct Player {
    // Passive game stats
    money: i32,  // start the run with $4
    deck: Vec<Card>,  // start with standard 52
    jokers: Vec<Joker>,
    // consumables: Vec<Consumable>,  // TBD; things like planet cards, tarot cards, and spectral cards

    // Change per round
    current_deck: Vec<Card>,
    cards_in_hand: Vec<Card>,
    hands: u8,  // how many hands per round
    discards: u8,

    // Settings
    max_cards_in_hand: u8,  // starts at 8
    max_discards: u8,  // starts at 3
    max_hands: u8,  // starts at 4
    max_jokers: u8,  // starts at 5
    // max_consumables: u8,  // starts at 2; implementation TBD
}

impl Player {
    fn deal_hand(&mut self) {
        while self.cards_in_hand.len() < self.max_cards_in_hand as usize {
            if let Some(card) = self.current_deck.pop() {
                self.cards_in_hand.push(card);
            } else {
                break;
            }
        }
    }

    fn shuffle_deck(&mut self) {
        self.current_deck = self.deck.clone();
        let mut rng = rand::thread_rng();
        self.current_deck.shuffle(&mut rng);
    }

    fn start_round(&mut self) {
        self.shuffle_deck();
        self.deal_hand();
        self.hands = self.max_hands;
        self.discards = self.max_discards;
    }
    
    fn discard_cards(&mut self, indices: &[usize]) {
        // Sort indices in descending order to avoid shifting issues when removing
        let mut sorted_indices = indices.to_vec();
        sorted_indices.sort_by(|a, b| b.cmp(a));
        
        // Remove cards at the specified indices
        for &idx in &sorted_indices {
            if idx < self.cards_in_hand.len() {
                self.cards_in_hand.remove(idx);
            }
        }
        
        // Deal new cards to replace the discarded ones
        self.deal_hand();
    }
}

const ANTES: [u64; 15] = [
    100, 300, 800, 2000, 5000, 11_000, 20_000, 35_000, 50_000,
    110_000, 560_000, 7_200_000, 300_000_000, 47_000_000_000, 29_000_000_000_000, // additional antes cut off for sake of simplicity
];

#[derive(Clone, Copy)]
enum BlindType {
    Small,
    Big,
    Boss,
}

impl std::fmt::Display for BlindType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlindType::Small => write!(f, "Small Blind"),
            BlindType::Big => write!(f, "Big Blind"),
            BlindType::Boss => write!(f, "Boss Blind"),
        }
    }
}

struct Blind {
    name: String,
    score: u64,
}

impl Blind {
    fn new(blind_type: BlindType, ante: u8) -> Self {
        let (name, score) = match blind_type {
            BlindType::Small => (
                "Small Blind".to_string(),
                ANTES[ante as usize]  // despite there being an ante 0, we start at 1
            ),
            BlindType::Big => (
                "Big Blind".to_string(), 
                (ANTES[ante as usize] as f64 * 1.5) as u64
            ),
            BlindType::Boss => (
                "Boss Blind".to_string(),
                ANTES[ante as usize] * 2
                // TODO: Boss blind effects
            ),
        };
        Self { name, score }
    }
}

struct Round {
    blind: Blind,
    score: u64,
}

impl Round {
    fn new(ante: u8, blind_type: BlindType) -> Self {
        let blind = Blind::new(blind_type, ante);
        Self {
            blind,
            score: 0,
        }
    }
}

struct GameManager {
    ante: u8, // begins at 1
    current_blind: BlindType,
    current_round: Round,
    player: Player,
}

impl GameManager {
    fn new(mut player: Player) -> Self {
        let ante = 1;
        let current_blind = BlindType::Small;
        player.start_round();
        let current_round = Round::new(ante, current_blind);
        
        Self {
            ante,
            current_blind,
            current_round,
            player
        }
    }

    fn next_round(&mut self) {
        self.current_blind = match self.current_blind {
            BlindType::Small => BlindType::Big,
            BlindType::Big => BlindType::Boss,
            BlindType::Boss => {
                self.ante += 1;
                BlindType::Small
            }
        };
        self.current_round = Round::new(self.ante, self.current_blind);
        self.player.start_round();
    }

    // Return values:
    // 0 = Game end, win
    // 1 = Game end, lose
    // 2 = Game still in progress
    fn take_turn(&mut self) -> u8 {
        // Print the ante and target points
        println!("\n=== {} - Ante {} ===", self.current_blind, self.ante);
        println!("Target: {} points", self.current_round.blind.score);
        println!("Current score: {}", self.current_round.score);
        
        // Print the cards in the player's hand (plus indices for selection)
        println!("\nYour hand:");
        for (i, card) in self.player.cards_in_hand.iter().enumerate() {
            println!("[{}] {}", i, card);
        }
        
        // Print available actions
        println!("\nHands remaining: {}", self.player.hands);
        println!("Discards remaining: {}", self.player.discards);
        
        // Prompt player to select cards and action
        println!("\nSelect cards (comma-separated indices) and action (d for discard, p for play)");
        println!("Example: '0,1,2,3,4 p' to play the first 5 cards");
        
        // Get user input
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        // Parse input
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 2 {
            println!("Invalid input! Please try again.");
            return 2;
        }
        
        // Parse card indices
        let indices_str = parts[0];
        let indices: Vec<usize> = indices_str
            .split(',')
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();
        
        // Check if indices are valid
        if indices.is_empty() || indices.len() > 5 {
            println!("You must select between 1 and 5 cards!");
            return 2;
        }
        
        for &idx in &indices {
            if idx >= self.player.cards_in_hand.len() {
                println!("Invalid card index: {}", idx);
                return 2;
            }
        }
        
        // Parse action
        let action = parts[1];
        match action {
            "d" | "D" => {
                // Check if player has discards left
                if self.player.discards == 0 {
                    println!("No discards remaining!");
                    return 2;
                }
                
                // Discard selected cards
                self.player.discard_cards(&indices);
                self.player.discards -= 1;
                println!("Cards discarded. New hand:");
                for (i, card) in self.player.cards_in_hand.iter().enumerate() {
                    println!("[{}] {}", i, card);
                }
            },
            "p" | "P" => {
                // Decrement hands counter
                self.player.hands -= 1;
                
                // Play selected cards
                println!("Playing cards:");
                let mut played_cards = Vec::new();
                for &idx in &indices {
                    let card = &self.player.cards_in_hand[idx];
                    println!("{}", card);
                    played_cards.push(card.clone());
                }
                
                // Determine poker hand
                let hand_type = self.determine_poker_hand(&played_cards);
                println!("\nHand type: {}", hand_type);
                
                // Calculate score for this hand
                let hand_score = self.calculate_hand_score(&played_cards, &hand_type);
                println!("Hand score: {}", hand_score);
                
                // Add to total score
                self.current_round.score += hand_score;
                println!("Total score: {}", self.current_round.score);
                
                // Remove played cards from hand
                self.player.discard_cards(&indices);
                
                // Check if round is complete
                if self.current_round.score >= self.current_round.blind.score {
                    println!("\nCongratulations! You've beaten the {}!", self.current_blind);
                    return 0;
                } else if self.player.hands == 0 {
                    println!("\nYou've run out of hands! Game over.");
                    return 1;
                }
            },
            _ => {
                println!("Invalid action! Use 'd' for discard or 'p' for play.");
            }
        }
        return 2;
    }

    fn play_round(&mut self) {
        if let turn_status = self.take_turn() {
            match turn_status {
                0 => {
                    println!("You win!");
                    return;
                },
                1 => {
                    println!("You lose!");
                    return;
                },
                _ => {
                    self.play_round();
                }
            }
        }
    }
    
    // Helper method to determine the poker hand type
    fn determine_poker_hand(&self, cards: &[Card]) -> String {
        // This is a simplified version - you'll want to implement proper poker hand evaluation
        // For now, just return a placeholder
        "High Card".to_string()
    }
    
    // Helper method to calculate the score for a hand
    fn calculate_hand_score(&self, cards: &[Card], hand_type: &str) -> u64 {
        // This is a simplified version - you'll want to implement proper scoring
        // For now, just return a placeholder score
        100
    }
}

/**
 * === GAMEPLAY FLOW ===
 * 1. Start round
 * 2. Deal 8 cards
 * 3. Player selects 1-5 cards, and either discards or plays
 *   a. If discards, remove selected cards from hand and replace with new cards, repeat
 *   b. If plays, play selected cards
 * 4. "HAND PLAYED" Determine the type of hand that has been played, as well as which of the cards will be counted for scoring
 * 5. Activate any Jokers' "HAND PLAYED" abilities (e.g. clone card, remove enhancement)
 * 6. Iterate through each card, scoring as follows:
 *   a. If the card is debuffed, skip
 *   b. Add the card's face value to "chips"
 *   c. If the card has any enhancements add those to either "chips" or "mult"
 *   d. Activate any Jokers' "CARD SCORED" abilities (e.g. +Mult for suit, +Chips for rank, etc)
 * 7. After all cards have been scored, iterate through the cards left in the player's hand for cards that trigger in-hand (e.g. steel cards)
 * 8. Activate any Jokers' "END OF ROUND" abilities (e.g. x3 mult if enhanced cards, +Chips if hand is two pair, etc)
 * 9. Calculate hand score via "chips x mult", add to total score, check if we've won or need to keep playing and action accordingly
 */

fn main() {
    println!("Welcome to BalatRust! Press Enter to continue");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Create player and their deck
    let mut deck = Vec::new();
    for suit in [Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds].iter() {
        for rank in ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"] {
            deck.push(Card { suit: suit.clone(), rank: rank.to_string() });
        }
    }
    let player = Player {
        money: 4,
        deck,
        jokers: Vec::new(),
        current_deck: Vec::new(),
        cards_in_hand: Vec::new(),
        hands: 4,
        discards: 3,
        max_cards_in_hand: 8,
        max_hands: 4,
        max_discards: 3,
        max_jokers: 5,
    };

    let mut game_manager = GameManager::new(player);

    // BEGIN GAME LOOP
    loop {
        let game_status = game_manager.play_round();
        
        // Check for quit
        println!("\nPress Enter to continue or type 'q' to exit");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() == "q" {
            break;
        } else {
            game_manager.next_round();
        }
    }
}
