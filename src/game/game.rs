use rand::rngs::ThreadRng;
use log::info;
use super::tile_bag::{Tile, TileBag, classic_tile_bag, DrawTile};
use super::board::{Board, BoardCoordinates, create_classic_board, PlaceTiles, BoardDirection};


pub struct ScrabbleGame {
    pub tile_bag: TileBag,
    pub user_tiles: [Option<Tile>; 7],
    pub board: Board
}

pub trait PlayableScrabbleGame {
    fn attempt_tile_play(&mut self, c: char, row: usize, col: usize) -> bool;
}

impl PlayableScrabbleGame for ScrabbleGame {
    fn attempt_tile_play(&mut self, c: char, row: usize, col: usize) -> bool {
        let mut played = false;

        // check if character exists on bag
        let mut tile_ind: usize = 9;
        for (ind, tile) in self.user_tiles.iter().enumerate() {
            if tile.is_some() && tile.unwrap().character == c {
                tile_ind = ind;
            }
        }

        // check if space is available
        if tile_ind < 9 {
            let mut space = self.board.spaces[row][col];
            if space.current_tile.is_none() {
                let tile = self.user_tiles[tile_ind];
                self.user_tiles[tile_ind] = None;
                space.current_tile = tile;
                played = true;
            }
        } else {
            info!("Tile {} not found in user tiles", c);
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

