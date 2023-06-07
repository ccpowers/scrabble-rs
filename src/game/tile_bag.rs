use std::fmt::{self, Display};

use log::info;
use rand::rngs::ThreadRng;
use rand::Rng;
#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub character: char,
    pub value: u32
}

#[derive(Default)]
pub struct TileBag {
    pub tiles: Vec<Tile>,
    pub rng: ThreadRng
}

pub struct InfiniteTileBag {
    pub chars: Vec<char>,
    pub distribution: Vec<u32>
}

pub trait DrawTile {
    fn draw_tile(&mut self) -> Option<Tile>;
}

pub trait AddTile {
    fn add_tile(&mut self, tile: Tile) -> ();
}

pub trait PrintTiles {
    fn print_tiles(&self) -> ();
}

pub trait ExchangeTiles {
    fn exchange_tiles(&mut self, tiles: [Option<Tile>; 7]) -> [Option<Tile>; 7];
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.character, self.value)
    }
}
impl AddTile for TileBag {
    fn add_tile(&mut self, tile: Tile) -> () {
        info!("Added tile {} to bag", tile.character);
        self.tiles.push(tile);
    }
}

impl DrawTile for TileBag {
    fn draw_tile(&mut self) -> Option<Tile> {
        // if there are no tiles, return nothing
        if self.tiles.len() == 0 {
            return None;
        }

        // randomly choose some tiles
        let ind = self.rng.gen_range(0..self.tiles.len());

        // remove them from the bag
        let tile = self.tiles.remove(ind);
        info!("Drew tile {}", tile.character);
        return Some(tile);
    }
}

impl ExchangeTiles for TileBag {
    fn exchange_tiles(&mut self, tiles: [Option<Tile>; 7]) -> [Option<Tile>; 7] {
        info!("Exchanging tiles");
        // add all tiles to bag
        let mut num_tiles = 0;
        for tile_option in tiles {
            match tile_option {
                None => (),
                Some(t) => {
                    self.add_tile(t);
                    num_tiles = num_tiles + 1;
                }
            }
        }

        let mut ret_tiles: [Option<Tile>; 7] = [None; 7];
        for i in 0..num_tiles {
            ret_tiles[i] = self.draw_tile();
        }
    
        return ret_tiles;
    }
}

impl PrintTiles for TileBag {
    fn print_tiles(&self) -> () {
        let mut str: String = String::from("TileBag: ");
        for tile in &self.tiles {
            str.push_str(&format!("({} {})", tile.character, tile.value));
        }
        info!("{}", str);
    }
}

pub fn classic_tile_bag() -> TileBag {
    //create and initilize the tile bag
    let tile_rng: ThreadRng = rand::thread_rng();
    let mut tile_bag = TileBag { tiles: Vec::new(), rng: tile_rng };

    // tuple of (character, points, number in bag)
    let classic_tiles: [(char, u32, usize); 26] = [
        ('a', 1, 9),
        ('b', 3, 2),
        ('c', 3, 2),
        ('d', 2, 4),
        ('e', 1, 12),
        ('f', 4, 2),
        ('g', 2, 3),
        ('h', 4, 2),
        ('i', 1, 9),
        ('j', 8, 1),
        ('k', 5, 1),
        ('l', 1, 4),
        ('m', 3, 2),
        ('n', 1, 6),
        ('o', 1, 8),
        ('p', 3, 2),
        ('q', 10, 1),
        ('r', 1, 6),
        ('s', 1, 4),
        ('t', 1, 6),
        ('u', 1, 4),
        ('v', 4, 2),
        ('w', 4, 2),
        ('x', 8, 1),
        ('y', 4, 2),
        ('z', 10, 1)
    ];

    for tile_tuple in classic_tiles {
        for _i in 1..tile_tuple.2 {
            tile_bag.add_tile(Tile {character: tile_tuple.0, value: tile_tuple.1});
        }
    }

    return tile_bag;  
}

pub fn print_user_tiles(tiles: [Option<Tile>; 7]) {
    let mut tile_str = "Tiles: ".to_string();
    for tile in tiles {
        let t = match tile {
            None => "[ ]".to_string(),
            Some(t) => format!("[{}]", t.character)
        };
        tile_str = tile_str + &t;
    }
    info!("{}", tile_str);
}