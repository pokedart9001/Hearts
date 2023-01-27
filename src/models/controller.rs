use inquire::{validator::ExactLengthValidator, MultiSelect, Select, Text};
use iter_tools::Itertools;
use thiserror::Error;

use super::{card::Card, game::{HeartsPlayedState, PassingOrder}, player::Player};

#[derive(Debug, Error)]
#[error("Controller failure.")]
pub struct ControllerError;

type ControllerResult<T> = Result<T, ControllerError>;

pub trait Controller {
    fn get_names(&self) -> ControllerResult<Vec<String>>;

    fn get_cards_to_pass(&self, from: &Player, to: &Player) -> ControllerResult<Vec<Card>>;

    fn get_card_to_place(
        &self, player: &Player, table: &[(usize, Card)], is_first_move: bool,
        hearts_played_state: &HeartsPlayedState,
    ) -> ControllerResult<Card>;

    fn display_passing_order(&self, passing_order: &PassingOrder);

    fn display_round_start(&self);

    fn display_winner(&self, player: &Player, card: Card, score: u8);

    fn display_scores(&self, players: &[Player]);
}

pub struct CLIController;

impl Controller for CLIController {
    fn get_names(&self) -> ControllerResult<Vec<String>> {
        (1..=4)
            .map(|i| {
                Text::new(&format!("Player {i}, enter your name:")).prompt().map_err(|_| ControllerError)
            })
            .collect()
    }

    fn get_cards_to_pass(&self, from: &Player, to: &Player) -> ControllerResult<Vec<Card>> {
        MultiSelect::new(
            &format!("{}, select 3 cards to pass to {}.", &from.name, &to.name),
            from.hand().iter().copied().sorted().collect(),
        )
        .with_validator(ExactLengthValidator::new(3))
        .with_page_size(13)
        .prompt()
        .map_err(|_| ControllerError)
    }

    fn get_card_to_place(
        &self, player: &Player, table: &[(usize, Card)], is_first_move: bool,
        hearts_played_state: &HeartsPlayedState,
    ) -> ControllerResult<Card> {
        let filtered_card_options = player
            .hand()
            .iter()
            .filter(|card| !is_first_move || card.is_two_of_clubs())
            .filter(|card| match table.first() {
                Some((_, first_card)) => card.suit == first_card.suit,
                None => true,
            })
            .filter(|card| match hearts_played_state {
                HeartsPlayedState::NoHeartsPlayed => !card.is_hearts(),
                _ => true,
            })
            .copied()
            .sorted()
            .collect_vec();

        Select::new(
            &format!("{}, select a card.", &player.name),
            if filtered_card_options.is_empty() {
                player.hand().iter().copied().sorted().collect()
            } else {
                filtered_card_options
            },
        )
        .with_page_size(13)
        .prompt()
        .map_err(|_| ControllerError)
    }

    fn display_round_start(&self) {
        println!();
    }

    fn display_passing_order(&self, passing_order: &PassingOrder) {
        println!("\nPassing order: {passing_order}\n");
    }

    fn display_winner(&self, player: &Player, card: Card, score: u8) {
        println!("\n{player} wins this trick with the {card} for {score} points.\n");
    }

    fn display_scores(&self, players: &[Player]) {
        println!("{:-^20}", "Scores");
        for player in players.iter().sorted() {
            println!("{player}: {} points", player.score());
        }
    }
}
