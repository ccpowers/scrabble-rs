use cursive::Printer;
use cursive::view::View;
use cursive::theme::{Color, ColorStyle, BaseColor};
use crate::tile_bag::{Tile};

pub struct RackView {
    pub tiles: [Option<Tile>; 7]
}

impl cursive::view::View for RackView {
    fn draw(&self, printer: &Printer) -> () {
        for (t, tile) in self.tiles.iter().enumerate() {
            let text = match tile {
                Some(t) => format!("[{}]", t.character),
                None => String::from("[ ]")
            };

            printer.with_color(
                ColorStyle::new(Color::Dark(BaseColor::Black), Color::RgbLowRes(3, 3, 3)),
                |printer| printer.print((t * 3, 0), &text),
            );
        }
    }
    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        return cursive::Vec2::new(7 * 3, 1);
    }
}