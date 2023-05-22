use crate::tile_bag::{TileBag, Tile, AddTile, DrawTile, PrintTiles};
use crate::board::{Board, PlaceTiles, BoardCoordinates, BoardDirection, print_board, create_classic_board};
use rand::rngs::ThreadRng;
use tile_bag::classic_tile_bag;
use crate::gui::{generate_scrabble_gui};

pub mod tile_bag;
pub mod board;
pub mod gui;
pub mod board_view;
fn main() {
    //create and initilize the tile bag
    let mut tile_bag = classic_tile_bag();
    let tile_rng: ThreadRng = rand::thread_rng();

    // create the board
    let mut board: Board = create_classic_board();
    print_board(&board);

    // place our tile
    let mut tile_vec: Vec<Tile> = Vec::new();
    tile_vec.push(Tile {character: 'c', value: 100});
    tile_vec.push(Tile {character: 'd', value: 100});
    tile_vec.push(Tile {character: 'e', value: 100});
    &mut board.place_tiles(&tile_vec, BoardCoordinates {x: 7, y: 7}, BoardDirection::North);

    let mut tile_vec: Vec<Tile> = Vec::new();
    tile_vec.push(Tile {character: 'c', value: 100});
    tile_vec.push(Tile {character: 'd', value: 100});
    tile_vec.push(Tile {character: 'e', value: 100});
    &mut board.place_tiles(&tile_vec, BoardCoordinates {x: 7, y: 7}, BoardDirection::West);

    let mut tile_vec: Vec<Tile> = Vec::new();
    tile_vec.push(Tile {character: 'f', value: 100});
    tile_vec.push(Tile {character: 'g', value: 100});
    tile_vec.push(Tile {character: 'f', value: 100});
    &mut board.place_tiles(&tile_vec, BoardCoordinates {x: 6, y: 8}, BoardDirection::South);
    print_board(&board);

    let mut scrabble_gui = generate_scrabble_gui(board);
    scrabble_gui.run();

}
