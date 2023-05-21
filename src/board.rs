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
pub struct Board {
    spaces: [[Space; BOARD_SIZE]; BOARD_SIZE]
}

pub struct Score {
    value: u32
}

pub trait PlaceTiles {
    fn place_tiles(&self, tiles: Vec<Tile>) -> Option<Score>;
}

impl PlaceTiles for Board {
    fn place_tiles(&self, tiles: Vec<Tile>) -> Option<Score> {
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