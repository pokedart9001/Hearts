use std::{
    fmt::Display,
    iter::{zip, Cycle},
};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use thiserror::Error;

use super::{card::Card, controller::Controller, deck::Deck, player::Player};

pub enum HeartsPlayedState<'a> {
    NoHeartsPlayed,
    HeartsPlayedOne(&'a Player),
    HeartsPlayedMany,
}

#[derive(EnumIter)]
pub enum PassingOrder {
    Right,
    Across,
    Left,
    Hold,
}

impl Display for PassingOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Right => "P1 -> P2 -> P3 -> P4 -> P1",
                Self::Across => "P1 <-> P3, P2 <-> P4",
                Self::Left => "P1 <- P2 <- P3 <- P4 <- P1",
                Self::Hold => "Hold",
            }
        )
    }
}

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Could not start game.")]
    StartError,
    #[error("Could not pass cards.")]
    PassError,
    #[error("Could not complete turn.")]
    TurnError,
}

type GameResult<T> = Result<T, GameError>;

pub struct Game<C: Controller> {
    pub players: Vec<Player>,

    deck: Deck,
    passing_order: Cycle<PassingOrderIter>,
    controller: C,
}

impl<'a, C> Game<C>
where
    C: Controller,
{
    pub fn new(controller: C) -> GameResult<Self> {
        Ok(Self {
            players: controller
                .get_names()
                .map_err(|_| GameError::StartError)?
                .into_iter()
                .map(|name| Player::new(name))
                .collect(),
            deck: Deck::new(),
            passing_order: PassingOrder::iter().cycle(),
            controller,
        })
    }

    pub fn round(&mut self) -> GameResult<u8> {
        self.deck.deal(&self.players);

        let next_passing_order = self.passing_order.next().expect("Passing order should exist");
        self.controller.display_passing_order(&next_passing_order);
        self.pass_cards(&next_passing_order)?;

        self.controller.display_round_start();

        let mut hearts_played_state = HeartsPlayedState::NoHeartsPlayed;
        let mut starting_index = self
            .players
            .iter()
            .enumerate()
            .find_map(|(i, player)| if player.has_two_of_clubs() { Some(i) } else { None })
            .expect("At least one player should start with the Two of Clubs");

        let mut scores = [0; 4];
        for turn in 1..=13 {
            let (winner_index, winning_card, score, hearts_played) =
                self.turn(starting_index, turn == 1, &hearts_played_state)?;

            if hearts_played {
                hearts_played_state = match hearts_played_state {
                    HeartsPlayedState::NoHeartsPlayed => {
                        HeartsPlayedState::HeartsPlayedOne(&self.players[winner_index])
                    }
                    HeartsPlayedState::HeartsPlayedOne(player) if player == &self.players[winner_index] => {
                        HeartsPlayedState::HeartsPlayedOne(player)
                    }
                    _ => HeartsPlayedState::HeartsPlayedMany,
                };
            }

            self.controller.display_winner(&self.players[winner_index], winning_card, score);

            scores[winner_index] += score;
            starting_index = winner_index;
        }

        for (player, score) in zip(&self.players, scores) {
            if let HeartsPlayedState::HeartsPlayedOne(hearts_player) = hearts_played_state {
                if player != hearts_player {
                    player.add_score(26);
                }
            } else {
                player.add_score(score);
            }
        }

        self.controller.display_scores(&self.players);

        Ok(self.max_score())
    }

    fn turn(
        &'a self, starting_index: usize, is_first_turn: bool, hearts_played_state: &HeartsPlayedState<'a>,
    ) -> GameResult<(usize, Card, u8, bool)> {
        let mut table = vec![];

        let mut is_first_move = is_first_turn;
        for i in Self::round_order(starting_index) {
            let card_choice = self
                .controller
                .get_card_to_place(&self.players[i], &table, is_first_move, hearts_played_state)
                .map_err(|_| GameError::TurnError)?;

            let placed_card = self.players[i].place(&card_choice).ok_or(GameError::TurnError)?;
            table.push((i, placed_card));

            is_first_move = false;
        }

        let (winner_index, winning_card) = table
            .iter()
            .filter(|(_, card)| card.suit == table[0].1.suit)
            .copied()
            .max_by_key(|(_, card)| card.to_owned())
            .expect("Table should be filled");

        let score = table.iter().map(|(_, card)| card.score()).sum();
        let hearts_played = table.iter().any(|(_, card)| card.is_hearts());

        Ok((winner_index, winning_card, score, hearts_played))
    }

    pub fn pass_cards(&mut self, passing_order: &PassingOrder) -> GameResult<()> {
        let Some(passing_indices) = Self::passing_indices(&passing_order) else {
            return Ok(());
        };

        let mut cards_to_pass = vec![];
        for (a, b) in passing_indices {
            let card_choices = self
                .controller
                .get_cards_to_pass(&self.players[a], &self.players[b])
                .map_err(|_| GameError::PassError)?;
            cards_to_pass.push((b, self.players[a].pass(&card_choices)));
        }

        for (i, to_pass) in cards_to_pass {
            self.players[i].take(to_pass);
        }

        Ok(())
    }

    fn passing_indices(passing_order: &PassingOrder) -> Option<[(usize, usize); 4]> {
        match passing_order {
            PassingOrder::Right => Some([(0, 1), (1, 2), (2, 3), (3, 0)]),
            PassingOrder::Across => Some([(0, 2), (1, 3), (2, 0), (3, 1)]),
            PassingOrder::Left => Some([(0, 3), (1, 0), (2, 1), (3, 2)]),
            _ => None,
        }
    }

    fn max_score(&self) -> u8 {
        self.players
            .iter()
            .map(|player| player.score())
            .max_by_key(|score| score.to_owned())
            .expect("At least one player should exist")
    }

    fn round_order(starting_index: usize) -> [usize; 4] {
        [
            (0 + starting_index) % 4,
            (1 + starting_index) % 4,
            (2 + starting_index) % 4,
            (3 + starting_index) % 4,
        ]
    }
}
