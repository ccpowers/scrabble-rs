
use cursive::{Printer};
use cursive::theme::{Color, ColorStyle, BaseColor};
use cursive::direction::Direction;
use cursive::event::{EventResult};
use cursive::view::{View, CannotFocus};

use crate::game::board::{SpaceValue, BoardCoordinates};
use crate::gui::selectable::{Selectable};
use crate::game::tile_bag::{Tile};

pub struct SpaceView {
    pub value: SpaceValue,
    pub tile: Option<Tile>,
    pub selected: Selectable,
    pub playable: bool,
    pub coordinates: BoardCoordinates
}

impl SpaceView {
    pub fn new(value: SpaceValue, tile: Option<Tile>, coordinates: BoardCoordinates) -> Self {
        SpaceView {
            value,
            tile,
            selected: Selectable { selected: false },
            playable: false,
            coordinates
        }
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

    /*fn on_event(&mut self, event: Event) -> EventResult {
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
    }*/

}

pub fn generate_space_view(value: SpaceValue, tile: Option<Tile>, coordinates: BoardCoordinates) -> SpaceView {
    return SpaceView {value: value, tile: tile, coordinates: coordinates, playable: false, selected: Selectable {selected: false}};
    /*return SpaceView::new(value, tile, coordinates)
        .wrap_with(cursive::views::FocusTracker::new)
        /*.on_focus(|view| {
            view.selected.set_selected(true);
            info!("Selected space {} {}", view.coordinates.x, view.coordinates.y);
            return EventResult::Ignored;
        })
        .on_focus_lost(|view| {
            view.selected.set_selected(false);
            return EventResult::Ignored;
        })*/
        .with_name("space");*/
}