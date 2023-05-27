use cursive::Printer;
use cursive::direction::Direction;
use cursive::view::{View, CannotFocus};
use cursive::theme::{Color, ColorStyle, BaseColor};
use cursive::views::{Button, LinearLayout};
use crate::game::tile_bag::{Tile, TileBag};
use cursive::event::{Event, EventResult, MouseButton, MouseEvent};
use super::selectable::{Selectable, SetSelected};

pub struct TileView {
    pub tile: Option<Tile>,
    pub selected: Selectable
}

impl View for TileView {
    fn draw(&self, printer: &Printer) -> () {
        let text = match self.tile {
            Some(c) => format!("[{}]", c.character),
            None => "[ ]".to_string()
        };

        let selected_color: Color = match (self.selected.selected, printer.focused) {
            (false, false) => Color::RgbLowRes(3, 3, 3),
            (true, false) => Color::RgbLowRes(5, 3, 3),
            (false, true) => Color::RgbLowRes(3, 5, 3),
            (true, true) => Color::RgbLowRes(5, 5, 3)
        };

        printer.with_color(
            ColorStyle::new(Color::Dark(BaseColor::Black), selected_color),
            |printer| printer.print((0, 0), &text),
        );
    } 

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        return cursive::Vec2::new(3, 1);
    }

    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        return Ok(EventResult::Consumed(None));
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        let mut consumed: bool = false;

        match event {
            Event::Mouse {offset: _, position: _, event: MouseEvent::Press(MouseButton::Left)} => { self.selected.set_selected(true); consumed =true},
            _ => ()
        };

        if consumed {
            return EventResult::Consumed(None);
        } else {
            return EventResult::Ignored;
        }
    }

}

pub fn generate_rack_views(tiles: [Option<Tile>; 7], tile_bag: TileBag) -> LinearLayout {
    let mut rack_layout: LinearLayout = LinearLayout::horizontal();

    for tile in tiles {
        rack_layout.add_child(TileView {tile, selected: Selectable {selected: false}});
    }
    rack_layout.add_child(Button::new("Exchange", |s| s.quit()));

    return rack_layout;

}