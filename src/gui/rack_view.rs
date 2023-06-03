use cursive::{Printer, CursiveRunnable, With, Cursive};
use cursive::direction::Direction;
use cursive::view::{View, CannotFocus, Nameable};
use cursive::theme::{Color, ColorStyle, BaseColor};
use cursive::views::{Button, LinearLayout, NamedView};
use crate::controller::exchange_tiles;
use crate::game::game::{ScrabbleGame, AttemptTilePlay};
use crate::game::tile_bag::{Tile, TileBag, ExchangeTiles};
use cursive::event::{Event, EventResult, MouseButton, MouseEvent};
use super::selectable::{Selectable, SetSelected};

pub struct TileView {
    pub tiles: [Option<Tile>; 7],
    pub tile_index: usize,
    pub selected: Selectable
}

impl View for TileView {
    fn draw(&self, printer: &Printer) -> () {
        // how to ensure index is in range?
        let text = match self.tiles[self.tile_index] {
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

pub fn generate_rack_views(user_tiles: &[Option<Tile>; 7]) -> NamedView<LinearLayout> {
    let mut rack_layout: NamedView<LinearLayout> = LinearLayout::horizontal().with_name("rack_wrapper");
    let mut rack: NamedView<LinearLayout> = LinearLayout::horizontal().with_name("rack");

    //let mut user_data = siv.user_data::<ScrabbleGame>();
    for tile_index in 0..6 {
        rack.get_mut().add_child(TileView {tiles: *user_tiles, tile_index, selected: Selectable {selected: false}});
    }

    rack_layout.get_mut().add_child(rack);
    rack_layout.get_mut().add_child(Button::new("Exchange", exchange_tiles));

    return rack_layout;
}