use cursive::{Cursive};
use game::{game::{ScrabbleGame, generate_scrabble_for_one}};
use crate::gui::gui::{generate_scrabble_gui};

pub mod game;
pub mod gui;
pub mod controller;
fn main() {
    //create and initilize the game
    let scrabble_game: ScrabbleGame = generate_scrabble_for_one();
    
    let mut scrabble_gui = generate_scrabble_gui(scrabble_game);

    scrabble_gui.set_global_callback('e', |s| all_events(s, 'e'));

    scrabble_gui.run();

}

fn all_events(_s: &Cursive, c: char) -> () {
    println!("Got character {}", c);
    //return  None;
}