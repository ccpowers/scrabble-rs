use cursive::direction::Direction;
use cursive::event::{Key, Event, EventResult, Callback};
use cursive::view::CannotFocus;
use cursive::{Printer, Cursive};
use cursive::theme::{Color, ColorStyle, BaseColor};
use cursive::views::{NamedView, TextView};
use log::{info, trace, debug};
use crate::game::board::{Board, SpaceValue, BOARD_SIZE, BoardCoordinates, Increment, BoardDirection, print_board};
use crate::game::game::{ScrabbleGame, PlayableScrabbleGame};
use crate::game::tile_bag::Tile;
use super::rack_view::RackView;


pub struct BoardView {
    pub board: Board,
    pub selected: BoardCoordinates
}

impl BoardView {
    pub fn set_board(&mut self, board: Board) {
        self.board = board;
    }
}

fn create_play_callback(c: char, coords: BoardCoordinates) -> Callback {
    Callback::from_fn(move |cursive: &mut Cursive| {
        let mut user_tiles: [Option<Tile>; 7] = [None;7];
        let mut board: Option<Board> = None;
        let mut score: u32 = 0;
        cursive.with_user_data(|game: &mut ScrabbleGame| {
            game.attempt_tile_play(c, coords.x, coords.y);
            user_tiles = game.user_tiles.clone();
            board = Some(game.board.clone());
            score = game.score();
        });

        // re-draw rack and board
        cursive.call_on_name("rack", |view: &mut NamedView<RackView>| {
            view.get_mut().set_tiles(user_tiles);
        });

        cursive.call_on_name("board", |view: &mut BoardView| {
            if board.is_some() {
                trace!("Setting board");
                view.set_board(board.unwrap());
                //view.
            } else {
                debug!("No board to set");
            }
        });

        cursive.call_on_name("score", |view: &mut NamedView<TextView>| {
            view.get_mut().set_content(format!("Score: {}", score));
        });

    })
}

impl cursive::view::View for BoardView {
    fn draw(&self, printer: &Printer) -> () {
        //info!("Draw board");
        //print_board(&self.board);
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
                    SpaceValue::TripleLetter => Color::RgbLowRes(3, 3, 5),
                    SpaceValue::DoubleWord => Color::RgbLowRes(4, 3, 3)
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
        //info!("Board got event");
        let mut consumed: bool = false;
        let mut cb = None;
        match event {
            Event::Key(Key::Left) => {consumed = true; self.selected = self.selected.increment(BoardDirection::East);},
            Event::Key(Key::Right) => {consumed = true; self.selected = self.selected.increment(BoardDirection::West);},
            Event::Key(Key::Up) => {consumed = true; self.selected = self.selected.increment(BoardDirection::North);},
            Event::Key(Key::Down) => {consumed = true; self.selected = self.selected.increment(BoardDirection::South);},
            Event::Char(c) => {consumed = true; cb = Some(create_play_callback(c, self.selected.clone())); }
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


pub fn generate_board_view(board: Board) -> BoardView {
    return BoardView {board: board, selected: BoardCoordinates { x: 0, y: 0 }};
}