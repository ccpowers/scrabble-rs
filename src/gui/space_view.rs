use cursive::{Printer, Cursive};
use cursive::theme::{Color, ColorStyle, BaseColor};
use cursive::direction::Direction;
use cursive::event::{EventResult, Event, MouseButton, MouseEvent};
use cursive::view::{View, CannotFocus};
use crate::game::board::{SpaceValue};
use crate::gui::selectable::{Selectable, SetSelected};
use crate::game::tile_bag::{Tile};

use super::gui;

pub struct SpaceView {
    pub value: SpaceValue,
    pub tile: Option<Tile>,
    pub selected: Selectable,
    pub playable: bool
}

pub trait PlayTile {
    fn play_tile(&mut self, tile: Tile) -> ();
}

impl PlayTile for SpaceView {
    fn play_tile(&mut self, tile: Tile) -> () {
        self.tile = Some(tile);
        self.playable = false;
    } 
}

impl View for SpaceView {
    fn draw(&self, printer: &Printer) -> () {
        let text = match self.tile {
            Some(c) => format!("[{}]", c.character),
            None => "[ ]".to_string()
        };

        let color: Color = match self.value {
            SpaceValue::SingleLetter => Color::RgbLowRes(3,3,3),
            SpaceValue::TripleWord => Color::RgbLowRes(5,3,3),
            SpaceValue::DoubleLetter => Color::RgbLowRes(3, 3, 4),
            SpaceValue::TripleLetter => Color::RgbLowRes(4, 3, 3),
            SpaceValue::DoubleWord => Color::RgbLowRes(3, 3, 5)
        };

        let selected_color: Color = match (self.selected.selected, printer.focused) {
            (false, false) => color,
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

    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        self.playable.then(EventResult::consumed).ok_or(CannotFocus)
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