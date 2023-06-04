use game::{game::{ScrabbleGame, generate_scrabble_for_one}};
use crate::gui::gui::{generate_scrabble_gui};
use log::{info};
use log4rs;

pub mod game;
pub mod gui;

fn main() {
    // initialize logs
    log4rs::init_file("logging.yaml", Default::default()).unwrap();
    info!("Starting scrabble game");
    //create and initilize the game
    let scrabble_game: ScrabbleGame = generate_scrabble_for_one();
    let mut scrabble_gui = generate_scrabble_gui(scrabble_game);
    scrabble_gui.run();

}
