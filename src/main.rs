use crate::game::tile_bag::{Tile, DrawTile};
use crate::game::board::{Board, PlaceTiles, BoardCoordinates, BoardDirection, create_classic_board};
use rand::rngs::ThreadRng;
use crate::game::tile_bag::classic_tile_bag;
use crate::gui::gui::{generate_scrabble_gui};

pub mod game;
pub mod gui;

fn main() {
    //create and initilize the tile bag
    let mut tile_bag = classic_tile_bag();
    let mut tile_rng: ThreadRng = rand::thread_rng();

    // draw tiles for user
    let mut user_tiles: [Option<Tile>; 7] = [None; 7];
    for i in 0..7 {
        user_tiles[i] = tile_bag.draw_tile(&mut tile_rng);
    }

    // create the board
    let mut board: Board = create_classic_board();

    // place our tile
    let mut tile_vec: Vec<Tile> = Vec::new();
    tile_vec.push(Tile {character: 'c', value: 100});
    tile_vec.push(Tile {character: 'd', value: 100});
    tile_vec.push(Tile {character: 'e', value: 100});
    &mut board.place_tiles(&tile_vec, BoardCoordinates {x: 7, y: 7}, BoardDirection::North);


    let mut scrabble_gui = generate_scrabble_gui(board, user_tiles);
    scrabble_gui.run();

}
