use rand::rngs::ThreadRng;

use super::tile_bag::{Tile, TileBag, classic_tile_bag, DrawTile};
use super::board::{Board, BoardCoordinates, create_classic_board, PlaceTiles, BoardDirection};


pub struct ScrabbleGame {
    pub tile_bag: TileBag,
    pub user_tiles: [Option<Tile>; 7],
    pub board: Board
}

pub fn generate_scrabble_for_one() -> ScrabbleGame {
    //create and initilize the tile bag
    let mut tile_bag = classic_tile_bag();
    let mut tile_rng: ThreadRng = rand::thread_rng();

    // draw tiles for user
    let mut user_tiles: [Option<Tile>; 7] = [None; 7];
    for i in 0..7 {
        user_tiles[i] = tile_bag.draw_tile();
    }

    // create the board
    let mut board: Board = create_classic_board();

    // place our tile
    let mut tile_vec: Vec<Tile> = Vec::new();
    tile_vec.push(Tile {character: 'c', value: 100});
    tile_vec.push(Tile {character: 'd', value: 100});
    tile_vec.push(Tile {character: 'e', value: 100});
    &mut board.place_tiles(&tile_vec, BoardCoordinates {x: 7, y: 7}, BoardDirection::North);

    return ScrabbleGame { tile_bag: tile_bag, user_tiles: user_tiles, board: board }

}