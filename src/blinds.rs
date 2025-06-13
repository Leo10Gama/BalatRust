use crate::cards::{Card, Suit};
use rand::seq::SliceRandom;

pub const ANTES: [u64; 15] = [
    100, 300, 800, 2000, 5000, 11_000, 20_000, 35_000, 50_000,
    110_000, 560_000, 7_200_000, 300_000_000, 47_000_000_000, 29_000_000_000_000, // additional antes cut off for sake of simplicity
];

#[derive(Clone, Copy)]
pub enum BlindType {
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

pub struct Blind {
    pub name: String,
    pub score: u64,
    pub description: String,
    pub boss_ability: Option<Box<dyn BossBlindAbility>>,
}

pub trait BossBlindAbility {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn is_card_debuffed(&self, card: &Card) -> bool;
}

pub struct TheClub;

impl BossBlindAbility for TheClub {
    fn name(&self) -> &str {
        "The Club"
    }

    fn description(&self) -> &str {
        "All Club cards are debuffed"
    }

    fn is_card_debuffed(&self, card: &Card) -> bool {
        card.suit == Suit::Clubs
    }
}

pub struct TheGoad;

impl BossBlindAbility for TheGoad {
    fn name(&self) -> &str {
        "The Goad"
    }

    fn description(&self) -> &str {
        "All Spade cards are debuffed"
    }

    fn is_card_debuffed(&self, card: &Card) -> bool {
        card.suit == Suit::Spades
    }
}

pub struct TheWindow;

impl BossBlindAbility for TheWindow {
    fn name(&self) -> &str {
        "The Window"
    }

    fn description(&self) -> &str {
        "All Diamond cards are debuffed"
    }

    fn is_card_debuffed(&self, card: &Card) -> bool {
        card.suit == Suit::Diamonds
    }
}

pub struct TheHead;

impl BossBlindAbility for TheHead {
    fn name(&self) -> &str {
        "The Head"
    }

    fn description(&self) -> &str {
        "All Heart cards are debuffed"
    }

    fn is_card_debuffed(&self, card: &Card) -> bool {
        card.suit == Suit::Hearts
    }
}

pub struct BossBlindFactory;

impl BossBlindFactory {
    pub fn create_boss_blind(name: &str) -> Box<dyn BossBlindAbility> {
        match name {
            "The Club" => Box::new(TheClub {}),
            "The Goad" => Box::new(TheGoad {}),
            "The Window" => Box::new(TheWindow {}),
            "The Head" => Box::new(TheHead {}),
            _ => panic!("Unknown boss blind ability: {}", name),
        }
    }
}

impl Blind {
    pub fn new(blind_type: BlindType, ante: u8) -> Self {
        match blind_type {
            BlindType::Small => Self {
                name: "Small Blind".to_string(),
                score: ANTES[ante as usize],  // despite there being an ante 0, we start at 1
                description: "".to_string(),
                boss_ability: None,
            },
            BlindType::Big => Self {
                name: "Big Blind".to_string(), 
                score: (ANTES[ante as usize] as f64 * 1.5) as u64,
                description: "".to_string(),
                boss_ability: None,
            },
            BlindType::Boss => {
                let ability = {
                    let boss_blinds = vec![  // pick a boss blind at random
                        "The Club",
                        "The Goad",
                        "The Window",
                        "The Head",
                    ];
                    let mut rng = rand::thread_rng();
                    let random_boss = boss_blinds.choose(&mut rng).unwrap();
                    BossBlindFactory::create_boss_blind(random_boss)
                };
                Self {
                    name: format!("Boss Blind - {}", ability.name()),
                    score: ANTES[ante as usize] * 2,
                    description: ability.description().to_string(),
                    boss_ability: Some(ability),
                }
            },
        }
    }
}