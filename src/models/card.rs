use std::fmt::Display;

use derivative::Derivative;
use strum_macros::{Display, EnumIter};

#[derive(Clone, Copy, Display, Debug, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Copy, Display, Debug, EnumIter, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Clubs,
    Diamonds,
    Spades,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Derivative)]
#[derivative(PartialOrd, Ord)]
pub struct Card {
    pub rank: Rank,
    #[derivative(PartialOrd = "ignore")]
    #[derivative(Ord = "ignore")]
    pub suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }

    pub fn score(&self) -> u8 {
        match (&self.rank, &self.suit) {
            (Rank::Queen, Suit::Spades) => 13,
            (_, Suit::Hearts) => 1,
            _ => 0,
        }
    }

    pub fn is_two_of_clubs(&self) -> bool {
        self.rank == Rank::Two && self.suit == Suit::Clubs
    }

    pub fn is_hearts(&self) -> bool {
        self.suit == Suit::Hearts
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}
