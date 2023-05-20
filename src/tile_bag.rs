use rand::rngs::ThreadRng;
use rand::Rng;
//use std::option;

pub struct Tile {
    pub char: char,
    pub value: u32
}

pub struct TileBag {
    pub tiles: Vec<Tile>
}

pub struct InfiniteTileBag {
    pub chars: Vec<char>,
    pub distribution: Vec<u32>
}

pub trait DrawTile {
    fn drawTile(&mut self, rng: ThreadRng) -> Option<Tile>;
}

pub trait AddTile {
    fn add_tile(&mut self, tile: Tile) -> ();
}

pub trait PrintTiles {
    fn print_tiles(&self) -> ();
}

impl AddTile for TileBag {
    fn add_tile(&mut self, tile: Tile) -> () {
        self.tiles.push(tile);
    }
}
impl DrawTile for TileBag {
    fn drawTile(&mut self, mut rng: ThreadRng) -> Option<Tile> {
        // if there are no tiles, return nothing
        if self.tiles.len() == 0 {
            return None;
        }

        // randomly choose some tiles
        let ind = rng.gen_range(0..self.tiles.len());

        // remove them from the bag
        return Some(self.tiles.remove(ind));
    }
}

impl PrintTiles for TileBag {
    fn print_tiles(&self) -> () {
        let mut str: String = String::from("TileBag: ");
        for tile in &self.tiles {
            str.push_str(&format!("({} {})", tile.char, tile.value));
        }
        println!("{}", str);
    }
}