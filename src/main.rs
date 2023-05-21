use crate::tile_bag::TileBag;
use crate::tile_bag::Tile;
use crate::tile_bag::AddTile;
use crate::tile_bag::DrawTile;
use crate::tile_bag::PrintTiles;
use crate::board::Board;
use crate::board::print_board;
use crate::board::create_classic_board;

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
}
