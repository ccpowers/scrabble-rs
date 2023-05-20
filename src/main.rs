use crate::tile_bag::TileBag;
use crate::tile_bag::Tile;
use crate::tile_bag::AddTile;
use crate::tile_bag::PrintTiles;
use rand::rngs::ThreadRng;
pub mod tile_bag;
fn main() {
    //create and initilize the tile bag
    let mut tileBag = TileBag { tiles: Vec::new() };

    tileBag.add_tile(Tile {char: 'a', value: 2});
    tileBag.add_tile(Tile {char: 'b', value: 2});
    tileBag.add_tile(Tile {char: 'c', value: 2});
    tileBag.print_tiles();
    //const tile_rng: ThreadRng = rand::thread_rng();
    //const tile: Tile = tileBag.drawTile(tile_rng);
    println!("Drew tile");
}
