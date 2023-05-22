use cursive::views::{TextView, Button, Dialog, LinearLayout, DummyView, Panel};
use cursive::CursiveRunnable;
use cursive::align::HAlign;
use cursive::view::{Resizable};
use crate::board_view::BoardView;
use crate::board::{Board};

pub fn generate_scrabble_gui(board: Board) -> CursiveRunnable {
     // show some stuff with cursive
     let mut siv = cursive::default();

     // create the board view
     let board_panel = Panel::new(BoardView {board});
     
     // create the tile rack view
     let rack_panel = Panel::new(
        LinearLayout::horizontal()
            .child(TextView::new("Rack goes here"))
            .child(DummyView.fixed_width(1))
            .child(Button::new("Exchange", |s| s.quit()))
            .child(DummyView.fixed_width(1))
            .child(Button::new("Play", |s| s.quit()))
    );
    
    // create the score view
    let score_panel = Panel::new(TextView::new("Score goes here").center());
    
     // create the layout
     siv.add_layer(
         Dialog::around(
             LinearLayout::vertical()
                 .child(TextView::new("Scrabble For One").h_align(HAlign::Center))
                 .child(DummyView.fixed_height(1))
                 .child(score_panel)
                 .child(board_panel)
                 .child(rack_panel)
         )
         .button("Quit", |s| s.quit())
         .h_align(HAlign::Center),
     );
     return siv;
}