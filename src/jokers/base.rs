use crate::Card;
use crate::Suit;
use crate::jokers::*;

#[derive(Clone)]
pub struct Joker {
    pub name: String,
    pub description: String,
}

pub trait JokerAbility {
    fn name(&self) -> &str;
    fn description(&self) -> &str;

    // Joker ability that triggers when a hand is played
    fn on_play(&self, chips: &mut u64, mult: &mut u64) {
        // Default implementation is empty
    }

    // Joker ability that triggers when a card is scored
    fn on_score(&self, card: &Card, chips: &mut u64, mult: &mut u64) {
        // Default implementation is empty
    }

    // Joker ability that triggers at the end of the round
    fn end_of_round(&self, chips: &mut u64, mult: &mut u64) {
        // Default implementation is empty
    }
}

pub struct JokerFactory {}

impl JokerFactory {
    pub fn create_joker(name: &str) -> Box<dyn JokerAbility> {
        match name {
            "Joker" => Box::new(JimboJoker {
                base: Joker {
                    name: "Joker".to_string(),
                    description: "+4 Mult".to_string(),
                },
            }),
            "Greedy Joker" => Box::new(GreedyJoker {
                base: Joker {
                    name: "Greedy Joker".to_string(),
                    description: "Played cards with ♦Diamond suit give +3 Mult when scored".to_string(),
                }
            }),
            "Lusty Joker" => Box::new(LustyJoker {
                base: Joker {
                    name: "Lusty Joker".to_string(),
                    description: "Played cards with ♥Heart suit give +3 Mult when scored".to_string(),
                }
            }),
            "Wrathful Joker" => Box::new(WrathfulJoker {
                base: Joker {
                    name: "Wrathful Joker".to_string(),
                    description: "Played cards with ♠Spade suit give +3 Mult when scored".to_string(),
                }
            }),
            "Gluttonous Joker" => Box::new(GluttonousJoker {
                base: Joker {
                    name: "Gluttonous Joker".to_string(),
                    description: "Played cards with ♣Club suit give +3 Mult when scored".to_string(),
                }
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
