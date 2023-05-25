use cursive::Printer;
use cursive::view::View;
use cursive::theme::{Color, ColorStyle, BaseColor};
use cursive::views::{Button, LinearLayout};
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


pub struct TileView {
    pub tile: Option<Tile>,
    pub selected: bool
}

impl cursive::view::View for TileView {
    fn draw(&self, printer: &Printer) -> () {
        let text = match self.tile {
            Some(c) => format!("[{}]", c.character),
            None => "[ ]".to_string()
        };

        let selected_color: Color = match (self.selected, printer.focused) {
            (false, false) => Color::Dark(BaseColor::Black),
            (true, false) => Color::Dark(BaseColor::Green),
            (false, true) => Color::Dark(BaseColor::Red),
            (true, true) => Color::Dark(BaseColor::Cyan)
        };

        printer.with_color(
            ColorStyle::new(Color::Dark(BaseColor::Black), selected_color),
            |printer| printer.print((0, 0), &text),
        );
    } 

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        return cursive::Vec2::new(3, 1);
    }

}

pub fn generate_rack_views(tiles: [Option<Tile>; 7]) -> LinearLayout {
    let mut rack_layout: LinearLayout = LinearLayout::horizontal();

    for tile in tiles {
        rack_layout.add_child(TileView {tile, selected: false});
    }
    rack_layout.add_child(Button::new("Exchange", |s| s.quit()));

    return rack_layout;

}