// accessible from this file and private_game.rs, but not from outside
mod private_game;

// publicly accessible
pub mod guess_number;
pub mod hangman;

// this is a function that will get inserted into the mod `games`
pub fn run_game_from_games_itself() {
    println!("Running all of the games!");
    private_game::run_private_game();
}
