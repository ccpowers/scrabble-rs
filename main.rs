use crate::game::{ScrabbleGame, generate_scrabble_for_one};
use crate::gui::generate_scrabble_gui;
extern crate log;
use log::{info};
extern crate log4rs;
//use log4rs;

pub mod game;
pub mod gui;
pub mod board;
pub mod board_view;
pub mod rack_view;
pub mod tile_bag;
fn main() {
    // initialize logs
    log4rs::init_file("logging.yaml", Default::default()).unwrap();
    info!("Starting scrabble game");
    
    //create and initilize the game
    let scrabble_game: ScrabbleGame = generate_scrabble_for_one();
    let mut scrabble_gui = generate_scrabble_gui(scrabble_game);
    scrabble_gui.run();
}
