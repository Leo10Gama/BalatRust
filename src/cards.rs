use colored::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Spades => write!(f, "♠"),
            Suit::Hearts => write!(f, "{}", "♥".red()),
            Suit::Clubs => write!(f, "{}", "♣".green()),
            Suit::Diamonds => write!(f, "{}", "♦".bright_blue()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: String,  // 2, 3, 4, 5, 6, 7, 8, 9, 10, J, Q, K, A
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>2}{}", self.rank, self.suit)
    }
}