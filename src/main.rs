use game::{game::{ScrabbleGame, generate_scrabble_for_one}, tile_bag::{ExchangeTiles, Tile}};
use crate::gui::gui::{generate_scrabble_gui};

pub mod game;
pub mod gui;

fn main() {
    //create and initilize the game
    let mut scrabble_game: ScrabbleGame = generate_scrabble_for_one();

    // create some callbacks on scrabble game
    let exchange_cb = |tiles: [Option<Tile>;7] | -> [Option<Tile>; 7] {
        return scrabble_game.tile_bag.exchange_tiles(tiles);
    };

    
    let mut scrabble_gui = generate_scrabble_gui(scrabble_game);
    scrabble_gui.run();

}
