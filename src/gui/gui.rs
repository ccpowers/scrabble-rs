use cursive::views::{TextView, Dialog, LinearLayout, DummyView, Panel, NamedView};
use cursive::{CursiveRunnable, Cursive};
use cursive::align::HAlign;
use cursive::view::{Resizable, Nameable};
use crate::game::game::{ScrabbleGame, PlayableScrabbleGame};
use crate::game::tile_bag::Tile;
use crate::gui::board_view::{generate_board_views};
use crate::gui::rack_view::{generate_rack_views};

use super::board_view::generate_board_view;
use super::rack_view::TileView;
use super::selectable::Selectable;
use super::space_view::SpaceView;

pub fn generate_scrabble_gui(game: ScrabbleGame) -> CursiveRunnable {
    // show some stuff with cursive
    let mut siv = cursive::default();
    let board = game.board.clone();
    siv.set_user_data::<ScrabbleGame>(game);

    // create the board view
    let board_view = generate_board_view(board).with_name("board");
    let board_panel = Panel::new(board_view);
     

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

     // add global callbacks for each letter in the alphabet
     // can't find a way to do this with access to Cursive object and event 
     let letters = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z']; // could probably get this from tile bag?
     for letter in letters {
        siv.add_global_callback(letter, move |s| play_tile(s, letter));
     }

     return siv;
}

fn play_tile(siv: &mut Cursive, c: char) {
    // find selected row and col
    let mut row = 0;
    let mut col = 0;
    siv.call_on_all_named("space", |view: &mut SpaceView| {
        if view.selected.selected {
            row = view.coordinates.x;
            col = view.coordinates.y;
        }
    });

    // see if the tile exists in the game
    let mut game = siv.take_user_data::<ScrabbleGame>().unwrap();

    game.attempt_tile_play(c, row, col);

    // make a copy of the new tiles
    let user_tiles = game.user_tiles.clone();

    // make a copy of the new board
    let board = game.board.clone();

    siv.set_user_data(game);

    // set content on board and rack
    siv.call_on_name("rack", |view: &mut NamedView<LinearLayout>| {
        for tile_index in 0..6 {
            view.get_mut().remove_child(tile_index);
            view.get_mut().add_child(TileView {tiles: user_tiles, tile_index, selected: Selectable {selected: false}});
        }
    });

    // todo - find selected item in board if it exists
    siv.call_on_all_named("space", |view: &mut SpaceView| {
        view.tile = board.spaces[view.coordinates.x][view.coordinates.y].current_tile;
    })

}