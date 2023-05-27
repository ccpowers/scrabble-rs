use cursive::views::{TextView, Button, Dialog, LinearLayout, DummyView, Panel};
use cursive::{Cursive, CursiveRunnable};
use cursive::align::HAlign;
use cursive::view::{Resizable};
use crate::game::game::ScrabbleGame;
use crate::gui::board_view::{BoardView, generate_board_views};
use crate::game::board::{Board};
use crate::game::tile_bag::{Tile};
use crate::gui::rack_view::{RackView, generate_rack_views};

pub fn generate_scrabble_gui(game: ScrabbleGame) -> CursiveRunnable {
     // show some stuff with cursive
     let mut siv = cursive::default();

     // create the board view
     let board_view = generate_board_views(game.board);
     let board_panel = Panel::new(board_view);
     
     // create the tile rack view
     /*let rack_panel = Panel::new(
        LinearLayout::horizontal()
            .child(RackView {tiles: user_tiles})
            .child(DummyView.fixed_width(1))
            .child(Button::new("Exchange", |s| generate_exchange_pop(s)))
            .child(DummyView.fixed_width(1))
            .child(Button::new("Play", |s| generate_play_pop(s)))
    );*/

    let rack_panel = Panel::new(generate_rack_views(game.user_tiles));
    
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

fn generate_play_pop(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
            .child(TextView::new("Get ready to play a move"))
            .child(Button::new("Play", |s| { s.pop_layer(); }))
        )
        .button("Play", |s| {s.pop_layer();})
    );
}

fn generate_exchange_pop(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
            .child(TextView::new("Choose tiles to exchange"))
            .child(Button::new("Play", |s| { s.pop_layer(); }))
        )
        .button("Play", |s| {s.pop_layer();})
    );
}