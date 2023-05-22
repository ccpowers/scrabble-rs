use cursive::Printer;
use cursive::view::View;
use crate::board::{Board, BoardCoordinates, BOARD_SIZE};

pub struct BoardView {
    pub board: Board,
    //pub selected: BoardCoordinates
}

impl cursive::view::View for BoardView {
    fn draw(&self, printer: &Printer) -> () {
        printer.print((0,0), "   A  B  C  D  E  F  G  H  I  J  K  L  M  N ");
        for (r, row) in self.board.spaces.iter().enumerate() {
            let mut row_str = format!("{}", r + 1);
            if (r+1) < 10 {
                row_str.push(' ');
            }
            printer.print((0, r + 1), &row_str.to_string());
            for (c, space) in row.iter().enumerate() {
                let text = match space.current_tile {
                    Some(c) => format!("[{}]", c. character),
                    None => "[ ]".to_string()
                };

                printer.print(((r * 3) + 2, c + 1), &text);
            }
        }
    } 
    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        return cursive::Vec2::new((BOARD_SIZE * 3) + 3, BOARD_SIZE);
    }
}