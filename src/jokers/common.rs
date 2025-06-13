use colored::*;

use crate::Card;
use crate::Suit;
use crate::determine_poker_hand;
use crate::PokerHand;
use crate::jokers::base::{Joker, JokerAbility};
use crate::pause_after_print;

pub struct JimboJoker {
    pub base: Joker,
}

impl JokerAbility for JimboJoker {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
    }

    // +4 mult at end of the round
    fn end_of_round(&self, chips: &mut u64, mult: &mut u64, cards: &[Card], scoring_card_indeces: &Vec<usize>) {
        println!("{}: {} mult", self.base.name, "+4".red());
        *mult += 4;
        pause_after_print(400);
    }
}

pub struct GreedyJoker {
    pub base: Joker,
}

impl JokerAbility for GreedyJoker {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
    }

    // +3 mult for diamonds
    fn on_score(&self, card: &Card, chips: &mut u64, mult: &mut u64) {
        if card.suit == Suit::Diamonds {
            println!("{}: {} mult", self.base.name, "+3".red());
            *mult += 3;
            pause_after_print(400);
        }
    }
}

pub struct LustyJoker {
    pub base: Joker,
}

impl JokerAbility for LustyJoker {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
    }

    // +3 mult for hearts
    fn on_score(&self, card: &Card, chips: &mut u64, mult: &mut u64) {
        if card.suit == Suit::Hearts {
            println!("{}: {} mult", self.base.name, "+3".red());
            *mult += 3;
            pause_after_print(400);
        }
    }
}

pub struct WrathfulJoker {
    pub base: Joker,
}

impl JokerAbility for WrathfulJoker {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
    }

    // +3 mult for spades
    fn on_score(&self, card: &Card, chips: &mut u64, mult: &mut u64) {
        if card.suit == Suit::Spades {
            println!("{}: {} mult", self.base.name, "+3".red());
            *mult += 3;
            pause_after_print(400);
        }
    }
}

pub struct GluttonousJoker {
    pub base: Joker,
}

impl JokerAbility for GluttonousJoker {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
    }

    // +3 mult for clubs
    fn on_score(&self, card: &Card, chips: &mut u64, mult: &mut u64) {
        if card.suit == Suit::Clubs {
            println!("{}: {} mult", self.base.name, "+3".red());
            *mult += 3;
            pause_after_print(400);
        }
    }
}

pub struct JollyJoker {
    pub base: Joker,
}

impl JokerAbility for JollyJoker {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
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
                println!("{}: {} mult", self.base.name, "+8".red());
                *mult += 8;
                pause_after_print(400);
            },
            _ => {},
        };
    }
}

pub struct ZanyJoker {
    pub base: Joker,
}

impl JokerAbility for ZanyJoker {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
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
                println!("{}: {} mult", self.base.name, "+12".red());
                *mult += 12;
                pause_after_print(400);
            },
            _ => {},
        };
    }
}

pub struct MadJoker {
    pub base: Joker,
}

impl JokerAbility for MadJoker {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
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
                println!("{}: {} mult", self.base.name, "+10".red());
                *mult += 10;
                pause_after_print(400);
            },
            _ => {},
        };
    }
}

pub struct CrazyJoker {
    pub base: Joker,
}

impl JokerAbility for CrazyJoker {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
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
                println!("{}: {} mult", self.base.name, "+12".red());
                *mult += 12;
                pause_after_print(400);
            },
            _ => {},
        };
    }
}

pub struct DrollJoker {
    pub base: Joker,
}

impl JokerAbility for DrollJoker {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
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
                println!("{}: {} mult", self.base.name, "+10".red());
                *mult += 10;
                pause_after_print(400);
            },
            _ => {},
        };
    }
}

pub struct SlyJoker {
    pub base: Joker,
}

impl JokerAbility for SlyJoker {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
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
                println!("{}: {} chips", self.base.name, "+50".cyan());
                *chips += 50;
                pause_after_print(400);
            },
            _ => {},
        };
    }
}

pub struct WilyJoker {
    pub base: Joker,
}

impl JokerAbility for WilyJoker {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
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
                println!("{}: {} chips", self.base.name, "+100".cyan());
                *chips += 100;
                pause_after_print(400);
            },
            _ => {},
        };
    }
}

pub struct CleverJoker {
    pub base: Joker,
}

impl JokerAbility for CleverJoker {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
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
                println!("{}: {} chips", self.base.name, "+80".cyan());
                *chips += 80;
                pause_after_print(400);
            },
            _ => {},
        };
    }
}

pub struct DeviousJoker {
    pub base: Joker,
}

impl JokerAbility for DeviousJoker {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
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
                println!("{}: {} chips", self.base.name, "+100".cyan());
                *chips += 100;
                pause_after_print(400);
            },
            _ => {},
        };
    }
}

pub struct CraftyJoker {
    pub base: Joker,
}

impl JokerAbility for CraftyJoker {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn description(&self) -> &str {
        &self.base.description
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
                println!("{}: {} chips", self.base.name, "+80".cyan());
                *chips += 80;
                pause_after_print(400);
            },
            _ => {},
        };
    }
}
