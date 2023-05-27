use game::game::{ScrabbleGame, generate_scrabble_for_one};
use crate::gui::gui::{generate_scrabble_gui};

pub mod game;
pub mod gui;

fn main() {
    //create and initilize the game
    let mut scrabble_game: ScrabbleGame = generate_scrabble_for_one();

    let mut scrabble_gui = generate_scrabble_gui(scrabble_game);
    scrabble_gui.run();

}
