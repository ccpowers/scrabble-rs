use std::cmp::{max, min};

use log::{info, trace, debug};

use crate::tile_bag::Tile;

#[derive(Copy, Clone)]
pub enum SpaceValue {
    SingleLetter,
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord    
}

impl SpaceValue {
    pub fn letter_multiplier(&self) -> u32 {
        match self {
            Self::DoubleLetter => 2,
            Self::TripleLetter => 3,
            _ => 1
        }
    }
}

#[derive(Copy, Clone)]
pub struct Space {
    pub value: SpaceValue,
    occupied: bool,
    pub current_tile: Option<Tile>
}

// should be 14 for classic
pub const BOARD_SIZE: usize = 15;
#[derive(Copy, Clone)]
pub struct Board {
    pub spaces: [[Space; BOARD_SIZE]; BOARD_SIZE]
}

// are we allowed to put restrictions on this? i.e. less than 14
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BoardCoordinates {
    pub x: usize,
    pub y: usize
}

#[derive(Copy, Clone)]
pub enum BoardDirection {
    North,
    South,
    East,
    West
}

// is there a way to do generic traits that return T instead of BoardCoordinates?
// feels a little silly to define this as a trait when it returns this
pub trait Increment {
    fn increment(&self, direction: BoardDirection) -> BoardCoordinates;
}

// this could be static
impl Increment for BoardCoordinates {
    fn increment(&self, direction: BoardDirection) -> BoardCoordinates {
        let mut x: i32 = self.x.try_into().unwrap();
        let mut y: i32 = self.y.try_into().unwrap();

        match direction {
            BoardDirection::East => {x = x - 1;},
            BoardDirection::West => {x = x + 1},
            BoardDirection::North => {y = y -1},
            BoardDirection::South => {y = y + 1}
        };
        // make sure we don't go out of bounds
        let board_max: i32 = TryInto::<i32>::try_into(BOARD_SIZE).unwrap() - 1;
        x = min(max(x, 0), board_max);
        y = min(max(y, 0), board_max);
        trace!("Board coordinates are {} {}", x, y);
        return BoardCoordinates {x: x.try_into().unwrap(), y: y.try_into().unwrap()};
    }
}
pub struct Score {
    value: u32
}

pub trait PlaceTiles {
    fn place_tiles(&mut self, tiles: &Vec<Tile>, start: BoardCoordinates, direction: BoardDirection) -> Option<Score>;
}

impl PlaceTiles for Board {
    fn place_tiles(&mut self, tiles: &Vec<Tile>, start: BoardCoordinates, direction: BoardDirection) -> Option<Score> {
        // first move MUST place tiles in H8

        let mut current_coords: BoardCoordinates = start;
        let mut current_space: Space = self.spaces[start.x][start.y];
        for tile in tiles {
            let mut tile_placed: bool = false;
            while !tile_placed {
                if !current_space.occupied {
                    current_space.current_tile = Some(*tile);
                    current_space.occupied = true;
                    tile_placed = true;
                    self.spaces[current_coords.x][current_coords.y] = current_space;
                    info!("Placed tile {} at space {} {}", tile.character, current_coords.x, current_coords.y);
                }
                // move to next space
                current_coords = current_coords.increment(direction);
                current_space = self.spaces[current_coords.x][current_coords.y];
            }
        }
        print_board(self);
        // tiles must be adjacent to at least one other tile 

        // 
        return Some(Score {value: 0});
    }
}

pub fn create_classic_board() -> Board {
    // create empty board with all single letter value spaces
    let mut board: Board = {Board {spaces: [[Space {value: SpaceValue::SingleLetter, occupied: false, current_tile: None}; BOARD_SIZE]; BOARD_SIZE]}};
    
    // update board with different values
    let triple_word_coordinates: [BoardCoordinates; 8] = [ 
        BoardCoordinates {x:0, y:0}, 
        BoardCoordinates {x:0, y:7},
        BoardCoordinates {x:0, y:14},
        BoardCoordinates {x:7, y:0},
        BoardCoordinates {x:7, y:14},
        BoardCoordinates {x:14, y:0},
        BoardCoordinates {x:14, y:7},
        BoardCoordinates {x:14, y:14}
        ];

    for coordinate in triple_word_coordinates {
        board.spaces[coordinate.x][coordinate.y].value = SpaceValue::TripleWord;
    }

    let double_word_coordinates: [BoardCoordinates; 17] = [
        BoardCoordinates {x:1, y:1}, 
        BoardCoordinates {x:2, y:2},
        BoardCoordinates {x:3, y:3},
        BoardCoordinates {x:4, y:4},
        BoardCoordinates {x:7, y:7},
        BoardCoordinates {x:13, y:1},
        BoardCoordinates {x:12, y:2},
        BoardCoordinates {x:11, y:3},
        BoardCoordinates {x:10, y:4}, 
        BoardCoordinates {x:1, y:13},
        BoardCoordinates {x:2, y:12},
        BoardCoordinates {x:3, y:11},
        BoardCoordinates {x:4, y:10},
        BoardCoordinates {x:11, y:11},
        BoardCoordinates {x:13, y:13},
        BoardCoordinates {x:12, y:12},
        BoardCoordinates {x: 10, y: 10}
    ];

    for coordinate in double_word_coordinates {
        board.spaces[coordinate.x][coordinate.y].value = SpaceValue::DoubleWord;
    }

    let double_letter_coordinates = [
        BoardCoordinates {x:0, y:3},
        BoardCoordinates {x:0, y:11},
        BoardCoordinates {x:2, y:6},
        BoardCoordinates {x:2, y:8},
        BoardCoordinates {x:3, y:0},
        BoardCoordinates {x:3,y:7},
        BoardCoordinates {x:3, y:14},
        BoardCoordinates {x:6, y:2},
        BoardCoordinates {x:6, y:6},
        BoardCoordinates {x:6,y:8},
        BoardCoordinates {x:6, y:12},
        BoardCoordinates {x:7, y:3},
        BoardCoordinates {x:7, y:11},
        BoardCoordinates {x:8, y:2},
        BoardCoordinates {x:8, y:6},
        BoardCoordinates {x:8,y:8},
        BoardCoordinates {x:8, y:12},
        BoardCoordinates {x:11, y:0},
        BoardCoordinates {x:11, y:7},
        BoardCoordinates {x:11, y:14},
        BoardCoordinates {x:12, y:6},
        BoardCoordinates {x:12, y:8},
        BoardCoordinates {x:14, y:3},
        BoardCoordinates {x:14, y:11},
    ];

    for coordinate in double_letter_coordinates {
        board.spaces[coordinate.x][coordinate.y].value = SpaceValue::DoubleLetter;
    }

    let triple_letter_coordinates = [
        BoardCoordinates {x:1, y:5},
        BoardCoordinates {x:1, y:9},
        BoardCoordinates {x:5, y:1},
        BoardCoordinates {x:5, y:5},
        BoardCoordinates {x:5, y:9},
        BoardCoordinates {x:5, y:13}, 
        BoardCoordinates {x:13, y:5},
        BoardCoordinates {x:13, y:9},
        BoardCoordinates {x:9, y:1},
        BoardCoordinates {x:9, y:5},
        BoardCoordinates {x:9, y:9},
        BoardCoordinates {x:9, y:13},
    ];

    for coordinate in triple_letter_coordinates {
        board.spaces[coordinate.x][coordinate.y].value = SpaceValue::TripleLetter;
    }
    
    return board;
}
pub fn print_board(board: &Board) -> () {
    let mut board_string: String = String::from("");

    for row in &board.spaces {
        for space in row {
            let space_string: String = match &space.current_tile {
                Some(c) => format!("[{}]", c.character),
                None => "[ ]".to_string()
            };

            board_string.push_str(&space_string);
        }
        board_string.push('\n');
    }

    debug!("{}", board_string);
}