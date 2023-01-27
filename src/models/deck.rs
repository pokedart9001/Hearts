use super::{
    card::{Card, Rank, Suit},
    player::Player,
};

use iter_tools::Itertools;
use rand::prelude::SliceRandom;
use strum::IntoEnumIterator;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Self {
            cards: Rank::iter()
                .cartesian_product(Suit::iter())
                .map(|(rank, suit)| Card::new(rank, suit))
                .collect(),
        }
    }

    fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }

    pub fn deal(&mut self, players: &[Player]) {
        self.shuffle();
        for (i, player) in players.iter().enumerate() {
            player.take(self.cards[(i * 13)..((i + 1) * 13)].to_vec());
        }
    }
}
