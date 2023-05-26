use cursive::Printer;
use cursive::theme::{Color, ColorStyle, BaseColor};
use cursive::views::{LinearLayout, TextView};
use cursive::direction::Direction;
use cursive::event::{EventResult, Event, MouseButton, MouseEvent};
use cursive::view::{View, CannotFocus};
use crate::board::{Board, SpaceValue, BOARD_SIZE};
use crate::tile_bag::{Tile};
use crate::gui::space_view::{SpaceView};
use crate::gui::selectable::Selectable;
pub struct BoardView {
    pub board: Board,
    //pub selected: BoardCoordinates
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

                let color: Color = match space.value {
                    SpaceValue::SingleLetter => Color::RgbLowRes(3,3,3),
                    SpaceValue::TripleWord => Color::RgbLowRes(5,3,3),
                    SpaceValue::DoubleLetter => Color::RgbLowRes(3, 3, 4),
                    SpaceValue::TripleLetter => Color::RgbLowRes(4, 3, 3),
                    SpaceValue::DoubleWord => Color::RgbLowRes(3, 3, 5)
                };

                printer.with_color(
                    ColorStyle::new(Color::Dark(BaseColor::Black), color),
                    |printer| printer.print(((r * 3) + 2, c + 1), &text),
                );
            }
        }
    } 
    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        return cursive::Vec2::new((BOARD_SIZE * 3) + 3, BOARD_SIZE + 1);
    }
}

// generate views to create a cohesive board
pub fn generate_board_views(board: Board) -> LinearLayout {
    let mut linear_layout: LinearLayout = LinearLayout::vertical()
    .child(TextView::new("   A  B  C  D  E  F  G  H  I  J  K  L  M  N  O "));

    let llr = &mut linear_layout;
    for (r, row) in board.spaces.iter().enumerate() {
        let mut row_layout = LinearLayout::horizontal();
        row_layout.add_child(TextView::new(format!("{}",r)));
        let rl = &mut row_layout;
        for space in row {
            //let space_view = SpaceView {value: space.value, tile: space.current_tile, selected: false, playable: true};
            //space_view.on_event(cursive::event::MouseButton::Left);
            rl.add_child(SpaceView {value: space.value, tile: space.current_tile, selected: Selectable {selected: false}, playable: true});
        }

        llr.add_child(row_layout);
    }

    return linear_layout;
}