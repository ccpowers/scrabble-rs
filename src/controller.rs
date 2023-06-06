use cursive::views::{LinearLayout, NamedView};
use crate::{ScrabbleGame, gui::{rack_view::RackView, selectable::Selectable}, game::{tile_bag::{ExchangeTiles, Tile}, game::AttemptTilePlay}};
use cursive::Cursive;


pub fn play_tile(siv: &mut Cursive) {
    // take control of the game
    let mut game = siv.take_user_data::<ScrabbleGame>().unwrap();

    game.attempt_tile_play('e');

    // make a copy of the new tiles
    let user_tiles = game.user_tiles.clone();
    siv.set_user_data(game);

    // set content on board and rack

    siv.call_on_name("rack", |view: &mut NamedView<RackView>| {
        view.get_mut().set_tiles(user_tiles);
    });

}

pub fn exchange_tiles(s: &mut Cursive) {
    // exchange tiles
    let mut user_tiles: [Option<Tile>; 7] = [None; 7];
    s.with_user_data(|scrabble_game: &mut ScrabbleGame| {
        scrabble_game.user_tiles = scrabble_game.tile_bag.exchange_tiles(scrabble_game.user_tiles);
        user_tiles = scrabble_game.user_tiles.clone();
    });
    
    // get list of selected tiles from rack view
    s.call_on_name("rack", |view: &mut NamedView<RackView>| {
        view.get_mut().set_tiles(user_tiles);
    });
}

pub fn calculate_score(s: &mut Cursive) {
    // grab the board view
    println!("Going to calculate score");

    // set the score
}