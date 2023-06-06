use cursive::views::{TextView, Dialog, LinearLayout, DummyView, Panel};
use cursive::{CursiveRunnable};
use cursive::align::HAlign;
use cursive::view::{Resizable, Nameable};
use crate::game::game::{ScrabbleGame};
use crate::gui::rack_view::{generate_rack_views};
use crate::gui::board_view::generate_board_view;

pub fn generate_scrabble_gui(game: ScrabbleGame) -> CursiveRunnable {
    // show some stuff with cursive
    let mut siv = cursive::default();
    let board = game.board.clone();
    siv.set_user_data::<ScrabbleGame>(game);

    // create the board view
    let board_view = generate_board_view(board).with_name("board");
    let board_panel = Panel::new(board_view);
     
    // create rack view
    let rack_panel = Panel::new(generate_rack_views(&mut siv));
    
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