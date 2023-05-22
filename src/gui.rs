use cursive::views::{TextView, Dialog, LinearLayout, DummyView, Panel};
use cursive::CursiveRunnable;
use cursive::align::HAlign;
use cursive::view::{Resizable};
use crate::board_view::BoardView;
use crate::board::{Board};

pub fn generate_scrabble_gui(board: Board) -> CursiveRunnable {
     // show some stuff with cursive
     let mut siv = cursive::default();

     // create the board view
     let board_panel = Panel::new(BoardView {board}).fixed_height(16);
     
     // create the tile rack view
     let rack_panel = Panel::new(TextView::new("Rack goes here"));
     // create the layout
     siv.add_layer(
         Dialog::around(
             LinearLayout::vertical()
                 .child(TextView::new("Scrabble").h_align(HAlign::Center))
                 .child(DummyView.fixed_height(1))
                 .child(board_panel)
                 .child(rack_panel)
         )
         .button("Quit", |s| s.quit())
         .h_align(HAlign::Center),
     );
     return siv;
}