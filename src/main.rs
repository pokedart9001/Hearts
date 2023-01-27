mod models;
use models::{controller::CLIController, game::Game};

const MAX_SCORE: u8 = 100;
fn main() -> anyhow::Result<()> {
    println!("{:-^30}\n", "HEARTS");
    let mut game = Game::new(CLIController)?;
    while game.round()? < MAX_SCORE {}

    Ok(())
}
