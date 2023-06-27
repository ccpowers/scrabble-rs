use std::collections::HashMap;

extern crate rand;
extern crate log;
use rand::rngs::ThreadRng;
use log::{info, debug, trace};
use crate::board::print_board;

use crate::tile_bag::{Tile, TileBag, classic_tile_bag, DrawTile, print_user_tiles};
use crate::board::{Board, create_classic_board, BoardCoordinates, BoardDirection, Increment};


pub struct ScrabbleGame {
    pub tile_bag: TileBag,
    pub user_tiles: [Option<Tile>; 7],
    pub board: Board
}

pub trait PlayableScrabbleGame {
    fn attempt_tile_play(&mut self, c: char, row: usize, col: usize) -> bool;
    fn draw_tiles(&mut self) -> bool;
    fn score(&self) -> u32;
}

impl PlayableScrabbleGame for ScrabbleGame {
    fn draw_tiles(& mut self) -> bool {
        let drawn = false;

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
        debug!("Attempting to play tile {} at {} {}", c, row, col);
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
        debug!("Tile index: {}", tile_ind);

        // check if space is available
        // TODO make this a static const
        if tile_ind < max_tile_ind {
            let mut space = self.board.spaces[row][col];

            if space.current_tile.is_none() {
                let tile = self.user_tiles[tile_ind];
                if tile.is_some() { trace!("Tile is {:?}", tile)};
                self.user_tiles[tile_ind] = None;
                print_user_tiles(self.user_tiles);
                space.current_tile = tile;
                played = true;
                trace!("Space has tile {}", space.current_tile.unwrap().character);
                print_board(&self.board);
                self.board.spaces[row][col] = space;
                print_board(&self.board);

            }
        } else {
            info!("Tile {} not found in user tiles", c);
        }


        return played;
    }

    fn score(&self) -> u32 {
        let mut score = 0;

        let start = BoardCoordinates {x: 8, y: 8};
        let mut to_check: Vec<BoardCoordinates> = [start].to_vec();
        let mut checked: HashMap<BoardCoordinates, bool> = HashMap::new();

        info!("Going to start checks. len: {}", to_check.len());
        while to_check.len() > 0 {
            // pop the first element out of the vector
            let coords = to_check.pop();

            if coords.is_some() && !checked.contains_key(&coords.unwrap()) {
                info!("Checking {} {}", coords.unwrap().x, coords.unwrap().y);
                checked.insert(coords.unwrap(), true);
                let space = self.board.spaces[coords.unwrap().x][coords.unwrap().y];
                if space.current_tile.is_some() {
                    score = score + space.current_tile.unwrap().value * space.value.letter_multiplier();
                }

                // add the neighbors to the vector
                for direction in [BoardDirection::North, BoardDirection::South, BoardDirection::East, BoardDirection::West] {
                    to_check.push(coords.unwrap().increment(direction));
                }
            }

        }

        return score;
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
    let board: Board = create_classic_board();

    return ScrabbleGame { tile_bag: tile_bag, user_tiles: user_tiles, board: board }
}

