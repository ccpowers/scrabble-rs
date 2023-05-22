use cursive::Printer;
use cursive::view::View;
use crate::board::{Board, BoardCoordinates, BOARD_SIZE};

pub struct BoardView {
    pub board: Board,
    //pub selected: BoardCoordinates
}

impl cursive::view::View for BoardView {
    fn draw(&self, printer: &Printer) -> () {
        for (r, row) in self.board.spaces.iter().enumerate() {
            for (c, space) in row.iter().enumerate() {
                //println!("Printing at {} {}", r* 3, c);
                printer.print((r * 3, c), "[ ]");
            }
        }
    } 
    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        return cursive::Vec2::new(BOARD_SIZE * 3, BOARD_SIZE * 2);
    }
}