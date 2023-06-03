use cursive::event::{Event, EventTrigger, EventResult};
use cursive::views::{TextView, Dialog, LinearLayout, DummyView, Panel, OnEventView, NamedView};
use cursive::{CursiveRunnable, Cursive, wrap_impl, View};
use cursive::align::HAlign;
use cursive::view::{Resizable, Finder, Nameable, ViewWrapper};
use crate::game::game::{ScrabbleGame, AttemptTilePlay};
use crate::game::tile_bag::Tile;
use crate::gui::board_view::{generate_board_views};
use crate::gui::rack_view::{generate_rack_views};


pub struct ScrabbleGui<V> {
    pub view: V,
    pub game: ScrabbleGame
}

impl <V: View> ViewWrapper for ScrabbleGui<V> {
    wrap_impl!(self.view: V);

    fn wrap_on_event(&mut self, event: Event) -> EventResult {
        let mut consumed: bool = false;

        // find currently selected board space
        /*self.call_on_name("board", |&mut view: &mut NamedView<LinearLayout>| {

        });*/

        consumed = match event {
            Event::Char(c) => {
                println!("Got character {}", c);
                self.game.attempt_tile_play(c)
            },
            _ => false
        };

        // if not consumed, send view
        if !consumed {
             return self.view.on_event(event);
        }
        else {
            return EventResult::Consumed(None);
        }

    }
}

pub fn generate_scrabble_gui(game: ScrabbleGame) -> CursiveRunnable {
    // show some stuff with cursive
    let mut siv = cursive::default();
    //siv.set_user_data::<ScrabbleGame>(game);
    let mut scrabble_gui = ScrabbleGui {game: game, view: LinearLayout::vertical()};

    // create the board view
    let board_view = generate_board_views(&scrabble_gui.game.board);
    let board_panel = OnEventView::new(Panel::new(board_view).with_name("board"));
    let rack_panel = Panel::new(generate_rack_views(&scrabble_gui.game.user_tiles));
    
    // create the score view
    let score_panel = Panel::new(TextView::new("Score goes here").center());
    
    let scrabble_layout =  Dialog::around(
        LinearLayout::vertical()
            .child(TextView::new("Scrabble For One").h_align(HAlign::Center))
            .child(DummyView.fixed_height(1))
            .child(score_panel)
            .child(board_panel)
            .child(rack_panel)
    )
    .button("Quit", |s| s.quit())
    .h_align(HAlign::Center);

    // update the view
    scrabble_gui.view.add_child(scrabble_layout);

     // create the layout
     siv.add_layer(scrabble_gui);



     return siv;
}

pub fn attempt_tile_play(s: &mut Cursive, c: char, ) -> () {
    // see if the character is present in the user's tiles
     // exchange tiles
     let mut play_tile: Option<Tile> = None;
     s.with_user_data(|scrabble_game: &mut ScrabbleGame| {
        let mut tile_ind = 8; // todo this should be a max index or something
         for (ind, tile) in scrabble_game.user_tiles.iter().enumerate() {
            if tile.is_some() && c == tile.unwrap().character {
                tile_ind = ind;
            }
         }
         if tile_ind < 8 {
            play_tile = scrabble_game.user_tiles[tile_ind];
            scrabble_game.user_tiles[tile_ind] = None;
         }
     });
     
     // get list of selected tiles from rack view
    /*  s.call_on_name("board", |view: &mut NamedView<LinearLayout>| {
         for tile_index in 0..5 {
             view.get_mut().remove_child(tile_index);
             view.get_mut().add_child(TileView {tiles: user_tiles, tile_index, selected: Selectable {selected: false}});
         }
     });*/

    // see if the selected space is playable

    // play the tile

    // update which spaces are playable
}