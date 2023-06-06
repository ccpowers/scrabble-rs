use cursive::{Printer, CursiveRunnable, Cursive};
use cursive::direction::Direction;
use cursive::view::{View, CannotFocus, Nameable};
use cursive::theme::{Color, ColorStyle, BaseColor};
use cursive::views::{Button, LinearLayout, NamedView};
use crate::game::game::{ScrabbleGame, PlayableScrabbleGame};
use crate::game::tile_bag::{Tile, ExchangeTiles};
use cursive::event::{EventResult};


pub struct RackView {
    pub tiles: [Option<Tile>; 7],
}

impl RackView {
    pub fn new(tiles: [Option<Tile>; 7]) -> Self {
        return RackView {tiles: tiles};
    }

    pub fn set_tiles(&mut self, tiles: [Option<Tile>; 7]) {
        self.tiles = tiles;
    }
}

impl View for RackView {
    fn draw(&self, printer: &Printer) -> () {
        for (ind, tile) in self.tiles.iter().enumerate() {
            let text = match tile {
                Some(c) => format!("[{}]", c.character),
                None => "[ ]".to_string()
            };

            printer.with_color(
                ColorStyle::new(Color::Dark(BaseColor::Black), Color::RgbLowRes(3, 3, 3)),
                |printer| printer.print((ind * 3, 0), &text),
            );
        }
    } 

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        return cursive::Vec2::new(3 * self.tiles.len(), 1);
    }

    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        return Ok(EventResult::Consumed(None));
    }
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

pub fn draw_tiles(s: &mut Cursive) {
      // exchange tiles
      let mut user_tiles: [Option<Tile>; 7] = [None; 7];
      s.with_user_data(|scrabble_game: &mut ScrabbleGame| {
          scrabble_game.draw_tiles();
          user_tiles = scrabble_game.user_tiles.clone();
      });
      
      // get list of selected tiles from rack view
      s.call_on_name("rack", |view: &mut NamedView<RackView>| {
          view.get_mut().set_tiles(user_tiles);
      });  
}

pub fn generate_rack_views(siv: &mut CursiveRunnable) -> NamedView<LinearLayout> {
    let mut rack_layout: NamedView<LinearLayout> = LinearLayout::horizontal().with_name("rack_wrapper");
    let mut rack: NamedView<RackView> = RackView::new([None;7]).with_name("rack");

    let user_data = siv.user_data::<ScrabbleGame>();
    if user_data.is_some() {
        rack.get_mut().set_tiles(user_data.unwrap().user_tiles.clone());
    };

    rack_layout.get_mut().add_child(rack);
    rack_layout.get_mut().add_child(Button::new("Exchange", exchange_tiles));
    rack_layout.get_mut().add_child(Button::new("Draw", draw_tiles));
    return rack_layout;
}