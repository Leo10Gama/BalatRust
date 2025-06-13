use colored::*;

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
            "Joker" => Box::new(JimboJoker {
                base: Joker {
                    name: "Joker".to_string(),
                    description: format!("{} {}",
                        "+4".red().bold(),
                        "Mult".bold(),
                    ),
                },
            }),
            "Greedy Joker" => Box::new(GreedyJoker {
                base: Joker {
                    name: "Greedy Joker".to_string(),
                    description: format!(
                        "Played cards with {} suit give {} {} when scored",
                        "♦Diamond".bright_blue().bold(),
                        "+3".red().bold(),
                        "Mult".bold(),
                    ),
                }
            }),
            "Lusty Joker" => Box::new(LustyJoker {
                base: Joker {
                    name: "Lusty Joker".to_string(),
                    description: format!(
                        "Played cards with {} suit give {} {} when scored",
                        "♥Heart".red().bold(),
                        "+3".red().bold(),
                        "Mult".bold(),
                    ),
                }
            }),
            "Wrathful Joker" => Box::new(WrathfulJoker {
                base: Joker {
                    name: "Wrathful Joker".to_string(),
                    description: format!(
                        "Played cards with {} suit give {} {} when scored",
                        "♠Spade".bold(),
                        "+3".red().bold(),
                        "Mult".bold(),
                    ),
                }
            }),
            "Gluttonous Joker" => Box::new(GluttonousJoker {
                base: Joker {
                    name: "Gluttonous Joker".to_string(),
                    description: format!(
                        "Played cards with {} suit give {} {} when scored",
                        "♣Club".green().bold(),
                        "+3".red().bold(),
                        "Mult".bold(),
                    ),
                }
            }),
            "Jolly Joker" => Box::new(JollyJoker {
                base: Joker {
                    name: "Jolly Joker".to_string(),
                    description: format!(
                        "{} {} if played hand contains a {}",
                        "+8".red().bold(),
                        "Mult".bold(),
                        "Pair".bold()
                    ),
                }
            }),
            "Zany Joker" => Box::new(ZanyJoker {
                base: Joker {
                    name: "Zany Joker".to_string(),
                    description: format!(
                        "{} {} if played hand contains a {}",
                        "+12".red().bold(),
                        "Mult".bold(),
                        "Three of a Kind".bold()
                    ),
                }
            }),
            "Mad Joker" => Box::new(MadJoker {
                base: Joker {
                    name: "Mad Joker".to_string(),
                    description: format!(
                        "{} {} if played hand contains a {}",
                        "+10".red().bold(),
                        "Mult".bold(),
                        "Two Pair".bold()
                    ),
                }
            }),
            "Crazy Joker" => Box::new(CrazyJoker {
                base: Joker {
                    name: "Crazy Joker".to_string(),
                    description: format!(
                        "{} {} if played hand contains a {}",
                        "+12".red().bold(),
                        "Mult".bold(),
                        "Straight".bold()
                    ),
                }
            }),
            "Droll Joker" => Box::new(DrollJoker {
                base: Joker {
                    name: "Droll Joker".to_string(),
                    description: format!(
                        "{} {} if played hand contains a {}",
                        "+10".red().bold(),
                        "Mult".bold(),
                        "Flush".bold()
                    ),
                }
            }),
            "Sly Joker" => Box::new(SlyJoker {
                base: Joker {
                    name: "Sly Joker".to_string(),
                    description: format!(
                        "{} {} if played hand contains a {}",
                        "+50".cyan().bold(),
                        "Chips".bold(),
                        "Pair".bold()
                    ),
                }
            }),
            "Wily Joker" => Box::new(WilyJoker {
                base: Joker {
                    name: "Wily Joker".to_string(),
                    description: format!(
                        "{} {} if played hand contains a {}",
                        "+100".cyan().bold(),
                        "Chips".bold(),
                        "Three of a Kind".bold()
                    ),
                }
            }),
            "Clever Joker" => Box::new(CleverJoker {
                base: Joker {
                    name: "Clever Joker".to_string(),
                    description: format!(
                        "{} {} if played hand contains a {}",
                        "+80".cyan().bold(),
                        "Chips".bold(),
                        "Two Pair".bold()
                    ),
                }
            }),
            "Devious Joker" => Box::new(DeviousJoker {
                base: Joker {
                    name: "Devious Joker".to_string(),
                    description: format!(
                        "{} {} if played hand contains a {}",
                        "+100".cyan().bold(),
                        "Chips".bold(),
                        "Straight".bold()
                    ),
                }
            }),
            "Crafty Joker" => Box::new(CraftyJoker {
                base: Joker {
                    name: "Crafty Joker".to_string(),
                    description: format!(
                        "{} {} if played hand contains a {}",
                        "+80".cyan().bold(),
                        "Chips".bold(),
                        "Flush".bold()
                    ),
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
