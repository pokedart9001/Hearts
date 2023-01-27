use std::{
    cell::{Cell, Ref, RefCell},
    fmt::Display,
};

use super::card::Card;
use derivative::Derivative;

#[derive(Debug, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Player {
    #[derivative(PartialOrd = "ignore")]
    #[derivative(Ord = "ignore")]
    pub name: String,

    #[derivative(PartialOrd = "ignore")]
    #[derivative(Ord = "ignore")]
    hand: RefCell<Vec<Card>>,

    score: Cell<u8>,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player { name, hand: RefCell::new(vec![]), score: Cell::new(0) }
    }

    pub fn hand(&self) -> Ref<Vec<Card>> {
        self.hand.borrow()
    }

    pub fn score(&self) -> u8 {
        self.score.get()
    }

    pub fn add_score(&self, score: u8) {
        self.score.set(self.score.get() + score);
    }

    pub fn has_two_of_clubs(&self) -> bool {
        self.hand.borrow().iter().any(|card| card.is_two_of_clubs())
    }

    pub fn pass(&self, choices: &[Card]) -> Vec<Card> {
        let (to_pass, to_keep) = self.hand.borrow().iter().partition(|card| choices.contains(card));
        self.hand.swap(&RefCell::new(to_keep));
        to_pass
    }

    pub fn take(&self, cards: Vec<Card>) {
        self.hand.borrow_mut().extend(cards);
    }

    pub fn place(&self, choice: &Card) -> Option<Card> {
        let position = self.hand.borrow().iter().position(|card| card == choice)?;
        Some(self.hand.borrow_mut().remove(position))
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
