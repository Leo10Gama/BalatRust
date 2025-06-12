use crate::Card;
use crate::Suit;
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
    fn end_of_round(&self, chips: &mut u64, mult: &mut u64) {
        println!("{}: +4 mult", self.base.name);
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
            println!("{}: +3 mult", self.base.name);
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
            println!("{}: +3 mult", self.base.name);
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
            println!("{}: +3 mult", self.base.name);
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
            println!("{}: +3 mult", self.base.name);
            *mult += 3;
            pause_after_print(400);
        }
    }
}
