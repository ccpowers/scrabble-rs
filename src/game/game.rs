use rand::rngs::ThreadRng;

use super::tile_bag::{Tile, TileBag, classic_tile_bag, DrawTile};
use super::board::{Board, BoardCoordinates, create_classic_board, PlaceTiles, BoardDirection};


pub struct ScrabbleGame {
    pub tile_bag: TileBag,
    pub user_tiles: [Option<Tile>; 7],
    pub board: Board,
}

pub trait AttemptTilePlay {
    fn attempt_tile_play(&mut self, c: char) -> bool;
}

impl AttemptTilePlay for ScrabbleGame {
    fn attempt_tile_play(&mut self, c: char) -> bool {
        let mut played = false;
        let mut play_tile = None;
        // check if character is in user tiles
        let mut tile_ind = 8; // todo this should be a max index or something
        for (ind, tile) in self.user_tiles.iter().enumerate() {
           if tile.is_some() && c == tile.unwrap().character {
               tile_ind = ind;
           }
        }
        if tile_ind < 8 {
           play_tile = self.user_tiles[tile_ind];
           self.user_tiles[tile_ind] = None;
        }

        if play_tile.is_some() {
            self.board.spaces[0][0].current_tile = play_tile;
            played = true;
        }

        return played;
    }
}

pub fn generate_scrabble_for_one() -> ScrabbleGame {
    //create and initilize the tile bag
    let mut tile_bag = classic_tile_bag();
    let _tile_rng: ThreadRng = rand::thread_rng();

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
    let _ = &mut board.place_tiles(&tile_vec, BoardCoordinates {x: 7, y: 7}, BoardDirection::North);

    return ScrabbleGame { tile_bag: tile_bag, user_tiles: user_tiles, board: board }

}