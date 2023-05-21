use crate::tile_bag::{TileBag, Tile, AddTile, DrawTile, PrintTiles};
use crate::board::{Board, PlaceTiles, BoardCoordinates, BoardDirection, print_board, create_classic_board};
use rand::rngs::ThreadRng;

pub mod tile_bag;
pub mod board;

fn main() {
    //create and initilize the tile bag
    let mut tile_bag = TileBag { tiles: Vec::new() };

    tile_bag.add_tile(Tile {character: 'a', value: 2});
    tile_bag.add_tile(Tile {character: 'b', value: 2});
    tile_bag.add_tile(Tile {character: 'c', value: 2});
    tile_bag.print_tiles();
    let tile_rng: ThreadRng = rand::thread_rng();
    let tile: Option<Tile> = tile_bag.draw_tile(tile_rng);
    println!("Drew tile");
    tile_bag.print_tiles();

    // create the board
    let mut board: Board = create_classic_board();
    print_board(&board);

    // place our tile
    let mut tile_vec: Vec<Tile> = Vec::new();
    tile_vec.push(Tile {character: 'c', value: 100});
    tile_vec.push(Tile {character: 'd', value: 100});
    tile_vec.push(Tile {character: 'e', value: 100});
    &mut board.place_tiles(&tile_vec, BoardCoordinates {x: 7, y: 7}, BoardDirection::North);
    print_board(&board);
}
