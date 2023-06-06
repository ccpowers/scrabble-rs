use rand::rngs::ThreadRng;
use log::info;
use crate::game::board::print_board;

use super::tile_bag::{Tile, TileBag, classic_tile_bag, DrawTile, print_user_tiles};
use super::board::{Board, BoardCoordinates, create_classic_board, PlaceTiles, BoardDirection};


pub struct ScrabbleGame {
    pub tile_bag: TileBag,
    pub user_tiles: [Option<Tile>; 7],
    pub board: Board
}

pub trait PlayableScrabbleGame {
    fn attempt_tile_play(&mut self, c: char, row: usize, col: usize) -> bool;
    fn draw_tiles(&mut self) -> bool;
}

impl PlayableScrabbleGame for ScrabbleGame {
    fn draw_tiles(& mut self) -> bool {
        let mut drawn = false;

        print_user_tiles(self.user_tiles);
        for ind in 0..self.user_tiles.len() {
            if self.user_tiles[ind].is_none() {
                self.user_tiles[ind] = self.tile_bag.draw_tile();
            }
        }
        print_user_tiles(self.user_tiles);
        return drawn; // todo fix this

    }
    fn attempt_tile_play(&mut self, c: char, row: usize, col: usize) -> bool {
        info!("Attempting to play tile {} at {} {}", c, row, col);
        let mut played = false;

        // check if character exists on bag
        print_user_tiles(self.user_tiles);
        let max_tile_ind: usize = self.user_tiles.len() + 1;
        let mut tile_ind: usize = max_tile_ind;
        for (ind, tile) in self.user_tiles.iter().enumerate() {
            if tile.is_some() && tile.unwrap().character == c {
                tile_ind = ind;
            }
        }
        info!("Tile index: {}", tile_ind);

        // check if space is available
        // TODO make this a static const
        if tile_ind < max_tile_ind {
            let mut space = self.board.spaces[row][col];

            if space.current_tile.is_none() {
                let tile = self.user_tiles[tile_ind];
                if tile.is_some() { info!("Tile is {:?}", tile)};
                self.user_tiles[tile_ind] = None;
                print_user_tiles(self.user_tiles);
                space.current_tile = tile;
                played = true;
                info!("Space has tile {}", space.current_tile.unwrap().character);
                print_board(&self.board);
                self.board.spaces[row][col] = space;
                print_board(&self.board);

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

