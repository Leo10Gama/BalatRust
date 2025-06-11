use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Clone, Debug, PartialEq)]
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

enum PokerHand {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    FiveOfAKind,
    FlushHouse,  // e.g. 5♠, 5♠, 5♠, 8♠, 8♠
    FlushFive,  // e.g. five 7♣ cards
}

impl std::fmt::Display for PokerHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PokerHand::HighCard => write!(f, "High Card"),
            PokerHand::Pair => write!(f, "Pair"),
            PokerHand::TwoPair => write!(f, "Two Pair"),
            PokerHand::ThreeOfAKind => write!(f, "Three of a Kind"),
            PokerHand::Straight => write!(f, "Straight"),
            PokerHand::Flush => write!(f, "Flush"),
            PokerHand::FullHouse => write!(f, "Full House"),
            PokerHand::FourOfAKind => write!(f, "Four of a Kind"),
            PokerHand::StraightFlush => write!(f, "Straight Flush"),
            PokerHand::FiveOfAKind => write!(f, "Five of a Kind"),
            PokerHand::FlushHouse => write!(f, "Flush House"),
            PokerHand::FlushFive => write!(f, "Flush Five"),
        }
    }
}

// Placeholder for Joker struct
#[derive(Clone)]
struct Joker {
    name: String,
    description: String,
}

trait JokerAbility {
    fn name(&self) -> &str;
    fn description(&self) -> &str;

    // Joker ability that triggers when a hand is played
    fn on_play(&self, chips: &mut u64, mult: &mut u64) {
        // Default implementation is empty
    }

    // Joker ability that triggers when a card is scored
    fn on_score(&self, chips: &mut u64, mult: &mut u64) {
        // Default implementation is empty
    }

    // Joker ability that triggers at the end of the round
    fn end_of_round(&self, chips: &mut u64, mult: &mut u64) {
        // Default implementation is empty
    }
}

struct JimboJoker {
    base: Joker,
}

impl JokerAbility for JimboJoker {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
    }

    // +4 mult at end of the round
    fn end_of_round(&self, chips: &mut u64, mult: &mut u64) {
        println!("{}: +4 Mult", self.base.name);
        *mult += 4;
    }
}

struct JokerFactory {}

impl JokerFactory {
    fn create_joker(name: &str) -> Box<dyn JokerAbility> {
        match name {
            "Joker" => Box::new(JimboJoker {
                base: Joker {
                    name: "Joker".to_string(),
                    description: "+4 Mult".to_string(),
                },
            }),
            _ => Box::new(JimboJoker {  // default to Jimbo
                base: Joker {
                    name: "Joker".to_string(),
                    description: "+4 Mult".to_string(),
                },
            }),
        }
    }
}

struct Player {
    // Passive game stats
    money: i32,  // start the run with $4
    deck: Vec<Card>,  // start with standard 52
    jokers: Vec<Box<dyn JokerAbility>>,
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

        // Print jokers and their abilities (description)
        println!("\nJokers:");
        if &self.player.jokers.len() == 0 {
            println!("None");
        }
        for joker in &self.player.jokers {
            println!("[{}]: {}", joker.name(), joker.description());
        }

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
                let (hand_type, scoring_cards) = self.determine_poker_hand(&played_cards);
                println!("\nHand type: {}", hand_type);
                
                // Calculate score for this hand
                let (chips, mult) = self.calculate_hand_score(&played_cards, &hand_type, &scoring_cards);
                
                // Add to total score
                let round_score = chips * mult;
                println!("Round score: {} x {} = {}", chips, mult, round_score);
                self.current_round.score += round_score;
                println!("Total score: {}", self.current_round.score);
                
                // Remove played cards from hand
                self.player.discard_cards(&indices);
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
    
    // Helper method to determine the poker hand type. Returns the hand type and a list of indexes of the cards that have scored
    fn determine_poker_hand(&self, cards: &[Card]) -> (PokerHand, Vec<usize>) {
        // Start from the highest hand type and work down

        let is_flush = cards.len() == 5 && cards.iter().all(|card| match (&card.suit, &cards[0].suit) {
            (Suit::Spades, Suit::Spades) => true,
            (Suit::Hearts, Suit::Hearts) => true,
            (Suit::Clubs, Suit::Clubs) => true,
            (Suit::Diamonds, Suit::Diamonds) => true,
            _ => false,
        });
        let is_straight = {
            if cards.len() != 5 { 
                false  // can only have a straight with 5 cards
            } else {
                let char_to_num = |c: &String| match c.as_str() {
                    "A" => 14,
                    "K" => 13, 
                    "Q" => 12,
                    "J" => 11,
                    _ => c.parse::<u8>().unwrap()
                };
                let mut ranks = cards.iter()
                    .map(|card| char_to_num(&card.rank))
                    .collect::<Vec<_>>();
                ranks.sort();
                // Numbers should be exactly one away from each other; exception is A,2,3,4,5 since A=1 or A=14
                ranks.windows(2).all(|w| w[1] - w[0] == 1) || ranks == vec![2, 3, 4, 5, 14]
            }
        };

        // For hands that use all cards, we'll use this
        let all_indices: Vec<usize> = (0..cards.len()).collect();
        let mut rank_indices: HashMap<&String, Vec<usize>> = HashMap::new();
        for (i, card) in cards.iter().enumerate() {
            rank_indices.entry(&card.rank).or_insert(Vec::new()).push(i);
        }

        // FLUSH FIVE [7♣, 7♣, 7♣, 7♣, 7♣]
        if cards.iter().all(|card| card.rank == cards[0].rank && card.suit == cards[0].suit) {
            return (PokerHand::FlushFive, all_indices.clone());
        }

        // util: `ranks` will store the count of each rank
        let mut ranks = HashMap::new();
        for card in cards {
            *ranks.entry(card.rank.clone()).or_insert(0) += 1;
        }

        // FLUSH HOUSE [5♠, 5♠, 5♠, 8♠, 8♠]
        if is_flush {
            if ranks.len() == 2 && ranks.values().any(|&count| count == 3) && ranks.values().any(|&count| count == 2) {
                return (PokerHand::FlushHouse, all_indices.clone());
            }
        }

        // FIVE OF A KIND [7♣, 7♥, 7♠, 7♣, 7♦]
        if cards.iter().all(|card| card.rank == cards[0].rank) && cards.len() == 5 {
            return (PokerHand::FiveOfAKind, all_indices.clone());
        }

        // STRAIGHT FLUSH [8♥, 9♥, 10♥, J♥, Q♥]
        if is_flush && is_straight {
            return (PokerHand::StraightFlush, all_indices.clone());
        }

        // FOUR OF A KIND [6♣, 6♥, 6♠, 6♦, 8♣]
        if let Some((rank, _)) = ranks.iter().find(|(_, &count)| count == 4) {
            return (PokerHand::FourOfAKind, rank_indices.get(rank).unwrap().clone());
        }

        // FULL HOUSE [Q♠, Q♣, Q♥, 7♥, 7♦]
        if ranks.len() == 2 && ranks.values().any(|&count| count == 3) && ranks.values().any(|&count| count == 2) {
            return (PokerHand::FullHouse, all_indices.clone());
        }

        // FLUSH [A♣, 4♣, 7♣, 8♣, 10♣]
        if is_flush {
            return (PokerHand::Flush, all_indices.clone());
        }

        // STRAIGHT [8♣, 9♦, 10♠, J♠, Q♣]
        if is_straight {
            return (PokerHand::Straight, all_indices.clone());
        }

        // THREE OF A KIND [2♠, 2♥, 2♦, 6♠, 9♦]
        if let Some((rank, _)) = ranks.iter().find(|(_, &count)| count == 3) {
            return (PokerHand::ThreeOfAKind, rank_indices.get(rank).unwrap().clone());
        }

        // TWO PAIR [5♠, 5♥, 8♠, 8♦, 10♣]
        if ranks.values().filter(|&&count| count == 2).count() == 2 {
            let mut pair_indices = Vec::new();
            for (rank, &count) in &ranks {
                if count == 2 {
                    pair_indices.extend(rank_indices.get(rank).unwrap());
                }
            }
            return (PokerHand::TwoPair, pair_indices);
        }

        // PAIR [A♠, A♦, 4♣, 7♥, 9♥]
        if let Some((rank, _)) = ranks.iter().find(|(_, &count)| count == 2) {
            return (PokerHand::Pair, rank_indices.get(rank).unwrap().clone());
        }

        // HIGH CARD [A♣, 4♦, 7♥, 8♣, K♦]
        let char_to_num = |c: &String| match c.as_str() {
            "A" => 14,
            "K" => 13, 
            "Q" => 12,
            "J" => 11,
            _ => c.parse::<u8>().unwrap()
        };
        
        let mut highest_idx = 0;
        let mut highest_val = 0;
        for (i, card) in cards.iter().enumerate() {
            let val = char_to_num(&card.rank);
            if val > highest_val {
                highest_val = val;
                highest_idx = i;
            }
        }
        
        (PokerHand::HighCard, vec![highest_idx])
    }
    
    // Helper method to calculate the score for a hand. Returns (Chips, Mult)
    fn calculate_hand_score(&self, cards: &[Card], hand_type: &PokerHand, scoring_cards: &Vec<usize>) -> (u64, u64) {
        // Get base chips and mult
        let (mut chips, mut mult) = match hand_type {
            PokerHand::FlushFive => (160, 16),
            PokerHand::FlushHouse => (140, 14),
            PokerHand::FiveOfAKind => (120, 12),
            PokerHand::StraightFlush => (100, 8),
            PokerHand::FourOfAKind => (60, 7),
            PokerHand::FullHouse => (40, 4),
            PokerHand::Flush => (35, 4),
            PokerHand::Straight => (30, 4),
            PokerHand::ThreeOfAKind => (30, 3),
            PokerHand::TwoPair => (20, 2),
            PokerHand::Pair => (10, 2),
            PokerHand::HighCard => (5, 1),
        };
        println!("{} gives {} x {}", hand_type, chips, mult);

        // Add points for scoring cards
        for &i in scoring_cards.iter() {
            // Score face value
            let card = &cards[i];
            let card_score = match card.rank.as_str() {
                "A" => 11,
                "K" => 10,
                "Q" => 10,
                "J" => 10,
                _ => card.rank.parse::<u64>().unwrap()
            };
            println!("{} scores {}", card, card_score);
            chips += card_score;
            // Score any bonuses from jokers with ON SCORE abilities
            // TODO
        }

        // Score any bonuses from jokers with END OF ROUND abilities
        for joker in &self.player.jokers {
            joker.end_of_round(&mut chips, &mut mult);
        }

        return (chips, mult);
    }
}

/**
 * === GAMEPLAY FLOW ===
 * 1. Start round
 * 2. Deal 8 cards
 * 3. Player selects 1-5 cards, and either discards or plays
 *   a. If discards, remove selected cards from hand and replace with new cards, repeat
 *   b. If plays, play selected cards
 * 4. "ON PLAY" Determine the type of hand that has been played, as well as which of the cards will be counted for scoring
 * 5. Activate any Jokers' "ON PLAY" abilities (e.g. clone card, remove enhancement)
 * 6. Iterate through each card, scoring as follows:
 *   a. If the card is debuffed, skip
 *   b. Add the card's face value to "chips"
 *   c. If the card has any enhancements add those to either "chips" or "mult"
 *   d. Activate any Jokers' "ON SCORE" abilities (e.g. +Mult for suit, +Chips for rank, etc)
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
    let mut player = Player {
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

    // test joker
    let joker = JokerFactory::create_joker("Joker");
    player.jokers.push(joker);

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
