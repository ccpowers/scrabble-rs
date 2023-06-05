use std::rc::Rc;

use cursive::direction::Direction;
use cursive::event::{Key, EventResult, Callback};
use cursive::view::CannotFocus;
use cursive::{Printer, CursiveRunnable, Cursive, event::Event};
use cursive::theme::{Color, ColorStyle, BaseColor};
use cursive::views::{LinearLayout, TextView};
use log::info;



use crate::game::board::{Board, SpaceValue, BOARD_SIZE, BoardCoordinates, Increment, BoardDirection};

use crate::game::game::ScrabbleGame;
use crate::gui::space_view::{generate_space_view};

pub struct BoardView {
    pub board: Board,
    pub selected: BoardCoordinates
}

fn create_callback(c: char, coords: BoardCoordinates) -> /*Rc<dyn Fn(&mut Cursive)>*/ Callback {
    // Function implementation goes here
    // ...
    // Create the callback and return it
    Callback::from_fn(move |cursive: &mut Cursive| {
        println!("what {} {} {}", c, coords.x, coords.y);
    })
}

/*pub fn create_letter_cb(c: char, coords: BoardCoordinates) -> Callback {
    let cb = move |_s| {
        println!("{} {} {}",c, coords.x, coords.y);
    };

    return Callback::from_fn(Rc::new(|_s| {
        println!("{} {} {}",c, coords.x, coords.y);
    }));
}*/

pub fn place_letter(_siv: &mut Cursive, c: char, coordinates: BoardCoordinates) {
    info!("Gonna play {} at {} {}", c, coordinates.x, coordinates.y);
}
impl cursive::view::View for BoardView {
    fn draw(&self, printer: &Printer) -> () {
        printer.print((0,0), "   A  B  C  D  E  F  G  H  I  J  K  L  M  N  O ");
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

                let mut color: Color = match space.value {
                    SpaceValue::SingleLetter => Color::RgbLowRes(3,3,3),
                    SpaceValue::TripleWord => Color::RgbLowRes(5,3,3),
                    SpaceValue::DoubleLetter => Color::RgbLowRes(3, 3, 4),
                    SpaceValue::TripleLetter => Color::RgbLowRes(4, 3, 3),
                    SpaceValue::DoubleWord => Color::RgbLowRes(3, 3, 5)
                };

                if self.selected.x == r && self.selected.y == c {
                    color = Color::RgbLowRes(5, 5, 5)
                }

                printer.with_color(
                    ColorStyle::new(Color::Dark(BaseColor::Black), color),
                    |printer| printer.print(((r * 3) + 2, c + 1), &text),
                );
            }
        }
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        info!("Board got event");
        let mut consumed: bool = false;
        let mut cb = None;
        match event {
            Event::Key(Key::Left) => {consumed = true; self.selected = self.selected.increment(BoardDirection::East);},
            Event::Key(Key::Right) => {consumed = true; self.selected = self.selected.increment(BoardDirection::West);},
            Event::Key(Key::Up) => {consumed = true; self.selected = self.selected.increment(BoardDirection::North);},
            Event::Key(Key::Down) => {consumed = true; self.selected = self.selected.increment(BoardDirection::South);},
            Event::Char(c) => {consumed = true; cb = Some(create_callback(c, self.selected.clone())); }
            _ => ()
        };

        if consumed {
            return EventResult::Consumed(cb);
        } else {
            return EventResult::Ignored;
        }
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        return cursive::Vec2::new((BOARD_SIZE * 3) + 3, BOARD_SIZE + 1);
    }

    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        return Ok(EventResult::Consumed(None));
    }
}

// generate views to create a cohesive board
pub fn generate_board_views(siv: &mut CursiveRunnable) -> LinearLayout {
    let mut linear_layout: LinearLayout = LinearLayout::vertical()
    .child(TextView::new("   A  B  C  D  E  F  G  H  I  J  K  L  M  N  O "));

    let user_data = siv.user_data::<ScrabbleGame>();

    if user_data.is_some() {
        let llr = &mut linear_layout;
        for (r, row) in user_data.unwrap().board.spaces.iter().enumerate() {
            let mut row_layout = LinearLayout::horizontal();
            row_layout.add_child(TextView::new(format!("{}",r)));
            let rl = &mut row_layout;
            for (col, space) in row.iter().enumerate() {
                let space_view = generate_space_view(space.value, space.current_tile, BoardCoordinates { x: r, y: col });//SpaceView {value: space.value, tile: space.current_tile, selected: Selectable { selected: false }, playable: true, coordinates: BoardCoordinates { x: 0, y: 0 }};
                //space_view.on_event(cursive::event::MouseButton::Left);
                //rl.add_child(SpaceView {value: space.value, tile: space.current_tile, playable: false, selected: Selectable {selected: false}, coordinates: BoardCoordinates { x: r, y: col }});
                //rl.add_child(generate_space_view(space.value, space.current_tile, BoardCoordinates {x: r, y: col}));
                rl.add_child(space_view);
            }
    
            llr.add_child(row_layout);
        }
    }

    return linear_layout;
}

pub fn generate_board_view(board: Board) -> BoardView {
    return BoardView {board: board, selected: BoardCoordinates { x: 0, y: 0 }};
}