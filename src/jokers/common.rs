use colored::*;

use crate::Card;
use crate::Suit;
use crate::PokerHand;
use crate::jokers::base::JokerAbility;
use crate::determine_poker_hand;
use crate::pause_after_print;

pub struct JimboJoker {}

impl JokerAbility for JimboJoker {
    fn name(&self) -> &str {
        "Joker"
    }

    fn description(&self) -> String {
        format!("{} {}",
            "+4".red().bold(),
            "Mult".bold(),
        )
    }

    // +4 mult at end of the round
    fn end_of_round(&self, chips: &mut u64, mult: &mut u64, cards: &[Card], scoring_card_indeces: &Vec<usize>) {
        println!("{}: {} mult", self.name(), "+4".red());
        *mult += 4;
        pause_after_print(400);
    }
}

pub struct GreedyJoker {}

impl JokerAbility for GreedyJoker {
    fn name(&self) -> &str {
        "Greedy Joker"
    }

    fn description(&self) -> String {
        format!(
            "Played cards with {} suit give {} {} when scored",
            "♦Diamond".bright_blue().bold(),
            "+3".red().bold(),
            "Mult".bold(),
        )
    }

    // +3 mult for diamonds
    fn on_score(&self, card: &Card, chips: &mut u64, mult: &mut u64) {
        if card.suit == Suit::Diamonds {
            println!("{}: {} mult", self.name(), "+3".red());
            *mult += 3;
            pause_after_print(400);
        }
    }
}

pub struct LustyJoker {}

impl JokerAbility for LustyJoker {
    fn name(&self) -> &str {
        "Lusty Joker"
    }

    fn description(&self) -> String {
        format!(
            "Played cards with {} suit give {} {} when scored",
            "♥Heart".red().bold(),
            "+3".red().bold(),
            "Mult".bold(),
        )
    }

    // +3 mult for hearts
    fn on_score(&self, card: &Card, chips: &mut u64, mult: &mut u64) {
        if card.suit == Suit::Hearts {
            println!("{}: {} mult", self.name(), "+3".red());
            *mult += 3;
            pause_after_print(400);
        }
    }
}

pub struct WrathfulJoker {}

impl JokerAbility for WrathfulJoker {
    fn name(&self) -> &str {
        "Wrathful Joker"
    }

    fn description(&self) -> String {
        format!(
            "Played cards with {} suit give {} {} when scored",
            "♠Spade".bold(),
            "+3".red().bold(),
            "Mult".bold(),
        )
    }

    // +3 mult for spades
    fn on_score(&self, card: &Card, chips: &mut u64, mult: &mut u64) {
        if card.suit == Suit::Spades {
            println!("{}: {} mult", self.name(), "+3".red());
            *mult += 3;
            pause_after_print(400);
        }
    }
}

pub struct GluttonousJoker {}

impl JokerAbility for GluttonousJoker {
    fn name(&self) -> &str {
        "Gluttonous Joker"
    }

    fn description(&self) -> String {
        format!(
            "Played cards with {} suit give {} {} when scored",
            "♣Club".green().bold(),
            "+3".red().bold(),
            "Mult".bold(),
        )
    }

    // +3 mult for clubs
    fn on_score(&self, card: &Card, chips: &mut u64, mult: &mut u64) {
        if card.suit == Suit::Clubs {
            println!("{}: {} mult", self.name(), "+3".red());
            *mult += 3;
            pause_after_print(400);
        }
    }
}

pub struct JollyJoker {}

impl JokerAbility for JollyJoker {
    fn name(&self) -> &str {
        "Jolly Joker"
    }

    fn description(&self) -> String {
        format!(
            "{} {} if played hand contains a {}",
            "+8".red().bold(),
            "Mult".bold(),
            "Pair".bold()
        )
    }

    // +8 mult if hand has PAIR
    fn end_of_round(&self, chips: &mut u64, mult: &mut u64, cards: &[Card], scoring_card_indeces: &Vec<usize>) {
        // Get actual played cards that scored
        let scoring_cards: Vec<Card> = scoring_card_indeces.iter()
            .map(|&i| cards[i].clone())
            .collect();
        let (hand_type, _) = determine_poker_hand(&scoring_cards);
        match hand_type {
            PokerHand::Pair | PokerHand::TwoPair | PokerHand::ThreeOfAKind | PokerHand::FullHouse | PokerHand::FourOfAKind | PokerHand::FiveOfAKind | PokerHand::FlushHouse | PokerHand::FlushFive => {
                println!("{}: {} mult", self.name(), "+8".red());
                *mult += 8;
                pause_after_print(400);
            },
            _ => {},
        };
    }
}

pub struct ZanyJoker {}

impl JokerAbility for ZanyJoker {
    fn name(&self) -> &str {
        "Zany Joker"
    }

    fn description(&self) -> String {
        format!(
            "{} {} if played hand contains a {}",
            "+12".red().bold(),
            "Mult".bold(),
            "Three of a Kind".bold()
        )
    }

    // +12 mult if hand has THREE OF A KIND
    fn end_of_round(&self, chips: &mut u64, mult: &mut u64, cards: &[Card], scoring_card_indeces: &Vec<usize>) {
        // Get actual played cards that scored
        let scoring_cards: Vec<Card> = scoring_card_indeces.iter()
            .map(|&i| cards[i].clone())
            .collect();
        let (hand_type, _) = determine_poker_hand(&scoring_cards);
        match hand_type {
            PokerHand::ThreeOfAKind | PokerHand::FullHouse | PokerHand::FourOfAKind | PokerHand::FiveOfAKind | PokerHand::FlushHouse | PokerHand::FlushFive => {
                println!("{}: {} mult", self.name(), "+12".red());
                *mult += 12;
                pause_after_print(400);
            },
            _ => {},
        };
    }
}

pub struct MadJoker {}

impl JokerAbility for MadJoker {
    fn name(&self) -> &str {
        "Mad Joker"
    }

    fn description(&self) -> String {
        format!(
            "{} {} if played hand contains a {}",
            "+10".red().bold(),
            "Mult".bold(),
            "Two Pair".bold()
        )
    }

    // +10 mult if hand has TWO PAIR
    fn end_of_round(&self, chips: &mut u64, mult: &mut u64, cards: &[Card], scoring_card_indeces: &Vec<usize>) {
        // Get actual played cards that scored
        let scoring_cards: Vec<Card> = scoring_card_indeces.iter()
            .map(|&i| cards[i].clone())
            .collect();
        let (hand_type, _) = determine_poker_hand(&scoring_cards);
        match hand_type {
            PokerHand::TwoPair | PokerHand::FullHouse | PokerHand::FlushHouse => {
                println!("{}: {} mult", self.name(), "+10".red());
                *mult += 10;
                pause_after_print(400);
            },
            _ => {},
        };
    }
}

pub struct CrazyJoker {}

impl JokerAbility for CrazyJoker {
    fn name(&self) -> &str {
        "Crazy Joker"
    }

    fn description(&self) -> String {
        format!(
            "{} {} if played hand contains a {}",
            "+12".red().bold(),
            "Mult".bold(),
            "Straight".bold()
        )
    }

    // +12 mult if hand has STRAIGHT
    fn end_of_round(&self, chips: &mut u64, mult: &mut u64, cards: &[Card], scoring_card_indeces: &Vec<usize>) {
        // Get actual played cards that scored
        let scoring_cards: Vec<Card> = scoring_card_indeces.iter()
            .map(|&i| cards[i].clone())
            .collect();
        let (hand_type, _) = determine_poker_hand(&scoring_cards);
        match hand_type {
            PokerHand::Straight | PokerHand::StraightFlush => {
                println!("{}: {} mult", self.name(), "+12".red());
                *mult += 12;
                pause_after_print(400);
            },
            _ => {},
        };
    }
}

pub struct DrollJoker {}

impl JokerAbility for DrollJoker {
    fn name(&self) -> &str {
        "Droll Joker"
    }

    fn description(&self) -> String {
        format!(
            "{} {} if played hand contains a {}",
            "+10".red().bold(),
            "Mult".bold(),
            "Flush".bold()
        )
    }

    // +10 mult if hand has FLUSH
    fn end_of_round(&self, chips: &mut u64, mult: &mut u64, cards: &[Card], scoring_card_indeces: &Vec<usize>) {
        // Get actual played cards that scored
        let scoring_cards: Vec<Card> = scoring_card_indeces.iter()
            .map(|&i| cards[i].clone())
            .collect();
        let (hand_type, _) = determine_poker_hand(&scoring_cards);
        match hand_type {
            PokerHand::Flush | PokerHand::StraightFlush | PokerHand::FlushHouse | PokerHand::FlushFive => {
                println!("{}: {} mult", self.name(), "+10".red());
                *mult += 10;
                pause_after_print(400);
            },
            _ => {},
        };
    }
}

pub struct SlyJoker {}

impl JokerAbility for SlyJoker {
    fn name(&self) -> &str {
        "Sly Joker"
    }

    fn description(&self) -> String {
        format!(
            "{} {} if played hand contains a {}",
            "+50".cyan().bold(),
            "Chips".bold(),
            "Pair".bold()
        )
    }

    // +50 chips if hand has PAIR
    fn end_of_round(&self, chips: &mut u64, mult: &mut u64, cards: &[Card], scoring_card_indeces: &Vec<usize>) {
        // Get actual played cards that scored
        let scoring_cards: Vec<Card> = scoring_card_indeces.iter()
            .map(|&i| cards[i].clone())
            .collect();
        let (hand_type, _) = determine_poker_hand(&scoring_cards);
        match hand_type {
            PokerHand::Pair | PokerHand::TwoPair | PokerHand::ThreeOfAKind | PokerHand::FullHouse | PokerHand::FourOfAKind | PokerHand::FiveOfAKind | PokerHand::FlushHouse | PokerHand::FlushFive => {
                println!("{}: {} chips", self.name(), "+50".cyan());
                *chips += 50;
                pause_after_print(400);
            },
            _ => {},
        };
    }
}

pub struct WilyJoker {}

impl JokerAbility for WilyJoker {
    fn name(&self) -> &str {
        "Wily Joker"
    }

    fn description(&self) -> String {
        format!(
            "{} {} if played hand contains a {}",
            "+100".cyan().bold(),
            "Chips".bold(),
            "Three of a Kind".bold()
        )
    }

    // +100 chips if hand has THREE OF A KIND
    fn end_of_round(&self, chips: &mut u64, mult: &mut u64, cards: &[Card], scoring_card_indeces: &Vec<usize>) {
        // Get actual played cards that scored
        let scoring_cards: Vec<Card> = scoring_card_indeces.iter()
            .map(|&i| cards[i].clone())
            .collect();
        let (hand_type, _) = determine_poker_hand(&scoring_cards);
        match hand_type {
            PokerHand::ThreeOfAKind | PokerHand::FullHouse | PokerHand::FourOfAKind | PokerHand::FiveOfAKind | PokerHand::FlushHouse | PokerHand::FlushFive => {
                println!("{}: {} chips", self.name(), "+100".cyan());
                *chips += 100;
                pause_after_print(400);
            },
            _ => {},
        };
    }
}

pub struct CleverJoker {}

impl JokerAbility for CleverJoker {
    fn name(&self) -> &str {
        "Clever Joker"
    }

    fn description(&self) -> String {
        format!(
            "{} {} if played hand contains a {}",
            "+80".cyan().bold(),
            "Chips".bold(),
            "Two Pair".bold()
        )
    }

    // +80 chips if hand has TWO PAIR
    fn end_of_round(&self, chips: &mut u64, mult: &mut u64, cards: &[Card], scoring_card_indeces: &Vec<usize>) {
        // Get actual played cards that scored
        let scoring_cards: Vec<Card> = scoring_card_indeces.iter()
            .map(|&i| cards[i].clone())
            .collect();
        let (hand_type, _) = determine_poker_hand(&scoring_cards);
        match hand_type {
            PokerHand::TwoPair | PokerHand::FullHouse | PokerHand::FlushHouse => {
                println!("{}: {} chips", self.name(), "+80".cyan());
                *chips += 80;
                pause_after_print(400);
            },
            _ => {},
        };
    }
}

pub struct DeviousJoker {}

impl JokerAbility for DeviousJoker {
    fn name(&self) -> &str {
        "Devious Joker"
    }

    fn description(&self) -> String {
        format!(
            "{} {} if played hand contains a {}",
            "+100".cyan().bold(),
            "Chips".bold(),
            "Straight".bold()
        )
    }

    // +100 chips if hand has STRAIGHT
    fn end_of_round(&self, chips: &mut u64, mult: &mut u64, cards: &[Card], scoring_card_indeces: &Vec<usize>) {
        // Get actual played cards that scored
        let scoring_cards: Vec<Card> = scoring_card_indeces.iter()
            .map(|&i| cards[i].clone())
            .collect();
        let (hand_type, _) = determine_poker_hand(&scoring_cards);
        match hand_type {
            PokerHand::Straight | PokerHand::StraightFlush => {
                println!("{}: {} chips", self.name(), "+100".cyan());
                *chips += 100;
                pause_after_print(400);
            },
            _ => {},
        };
    }
}

pub struct CraftyJoker {}

impl JokerAbility for CraftyJoker {
    fn name(&self) -> &str {
        "Crafty Joker"
    }

    fn description(&self) -> String {
        format!(
            "{} {} if played hand contains a {}",
            "+80".cyan().bold(),
            "Chips".bold(),
            "Flush".bold()
        )
    }

    // +80 chips if hand has FLUSH
    fn end_of_round(&self, chips: &mut u64, mult: &mut u64, cards: &[Card], scoring_card_indeces: &Vec<usize>) {
        // Get actual played cards that scored
        let scoring_cards: Vec<Card> = scoring_card_indeces.iter()
            .map(|&i| cards[i].clone())
            .collect();
        let (hand_type, _) = determine_poker_hand(&scoring_cards);
        match hand_type {
            PokerHand::Flush | PokerHand::StraightFlush | PokerHand::FlushHouse | PokerHand::FlushFive => {
                println!("{}: {} chips", self.name(), "+80".cyan());
                *chips += 80;
                pause_after_print(400);
            },
            _ => {},
        };
    }
}
