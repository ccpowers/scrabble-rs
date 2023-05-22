use cursive::Printer;
use cursive::view::View;
use cursive::theme::{Color, ColorStyle, BaseColor};
use crate::tile::{Tile};

pub struct RackView {
    pub tiles: Vec<Tile>,
    pub capacity: usize
    //pub selected: BoardCoordinates
}

impl cursive::view::View for BoardView {
    fn draw(&self, printer: &Printer) -> () {
        for (t, tile) in self.tiles.iter().enumerate() {
            let text = format!("[{}]", tile.character);

            printer.with_color(
                ColorStyle::new(Color::Dark(BaseColor::Black), Color::RgbLowRes(3, 3, 3)),
                |printer| printer.print((0, t), &text),
            );
        }
    }
    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        return cursive::Vec2::new((BOARD_SIZE * 3) + 3, BOARD_SIZE + 1);
    }
}