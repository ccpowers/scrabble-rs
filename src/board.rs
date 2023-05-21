use crate::tile_bag::Tile;
use colored::Colorize;
use colored::ColoredString;

#[derive(Copy, Clone)]
pub enum SpaceValue {
    SingleLetter,
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord    
}

#[derive(Copy, Clone)]
pub struct Space {
    value: SpaceValue,
    occupied: bool,
    current_tile: Option<Tile>
}

// should be 14 for classic
const BOARD_SIZE: usize = 14;
#[derive(Copy, Clone)]
pub struct Board {
    spaces: [[Space; BOARD_SIZE]; BOARD_SIZE]
}

// are we allowed to put restrictions on this? i.e. less than 14
#[derive(Copy, Clone)]
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

impl Increment for BoardCoordinates {
    fn increment(&self, direction: BoardDirection) -> BoardCoordinates {
        return match direction {
            BoardDirection::North => BoardCoordinates {x: self.x - 1, y: self.y},
            BoardDirection::South => BoardCoordinates {x: self.x + 1, y: self.y},
            BoardDirection::East => BoardCoordinates {x: self.x, y: self.y - 1},
            BoardDirection::West => BoardCoordinates {x: self.x, y: self.y + 1}
        };
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
                    println!("Placed tile {} at space {} {}", tile.character, current_coords.x, current_coords.y);
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
    board.spaces[1][1].value = SpaceValue::TripleWord;
    
    return board;
}
pub fn print_board(board: &Board) -> () {
    let mut board_string: String = String::from("");

    for row in &board.spaces {
        for space in row {
            let mut space_string: String = String::from("");
            let space_string: String = match &space.current_tile {
                Some(c) => format!("[{}]", c.character),
                None => "[ ]".to_string()
            };

            /**let space_string_colored: ColoredString = match &space.value {
                SpaceValue::SingleLetter => space_string.normal(),
                SpaceValue::TripleWord => space_string.red(),
                &SpaceValue::DoubleLetter | &SpaceValue::TripleLetter | &SpaceValue::DoubleWord => space_string.blue()
            };**/
            //println!("Adding space string");
            board_string.push_str(&space_string);
            //println!("{}", &space_string_colored);
        }
        //println!("Adding return");
        board_string.push('\n');
    }

    println!("{}", board_string.blue());
}