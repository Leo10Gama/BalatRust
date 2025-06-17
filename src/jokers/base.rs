use colored::*;

use crate::Card;
use crate::Suit;
use crate::jokers::*;

pub trait JokerAbility {
    fn name(&self) -> &str;
    fn description(&self) -> String;

    // Joker ability that triggers when a hand is played
    fn on_play(&self, chips: &mut u64, mult: &mut u64, cards: &[Card], played_cards: &Vec<usize>) {
        // Default implementation is empty
    }

    // Joker ability that triggers when a card is scored
    fn on_score(&self, card: &Card, chips: &mut u64, mult: &mut u64) {
        // Default implementation is empty
    }

    // Joker ability that triggers at the end of the round
    fn end_of_round(&self, chips: &mut u64, mult: &mut u64, cards: &[Card], scoring_card_indeces: &Vec<usize>) {
        // Default implementation is empty
    }
}

pub struct JokerFactory {}

impl JokerFactory {
    pub fn create_joker(name: &str) -> Box<dyn JokerAbility> {
        match name {
            "Joker" => Box::new(JimboJoker {}),
            "Greedy Joker" => Box::new(GreedyJoker {}),
            "Lusty Joker" => Box::new(LustyJoker {}),
            "Wrathful Joker" => Box::new(WrathfulJoker {}),
            "Gluttonous Joker" => Box::new(GluttonousJoker {}),
            "Jolly Joker" => Box::new(JollyJoker {}),
            "Zany Joker" => Box::new(ZanyJoker {}),
            "Mad Joker" => Box::new(MadJoker {}),
            "Crazy Joker" => Box::new(CrazyJoker {}),
            "Droll Joker" => Box::new(DrollJoker {}),
            "Sly Joker" => Box::new(SlyJoker {}),
            "Wily Joker" => Box::new(WilyJoker {}),
            "Clever Joker" => Box::new(CleverJoker {}),
            "Devious Joker" => Box::new(DeviousJoker {}),
            "Crafty Joker" => Box::new(CraftyJoker {}),
            _ => Box::new(JimboJoker {}),  // default to Jimbo
        }
    }
}
