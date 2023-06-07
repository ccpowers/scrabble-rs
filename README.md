# scrabble-rs

Terminal based scrabble for one, just for fun. Based on scrabble description found on Wikipedia: https://en.wikipedia.org/wiki/Scrabble
Created using Rust and TUI library Cursive

## Use
To start:
`cargo run`

You should see:

![scrabble gui](https://raw.githubusercontent.com/ccpowers/scrabble-rs/main/scrabble-rs.png)

To play:
Select a space on the board using the arrow keys. The selected space will be highlighted. Press the key corresponding to the tile you wish to play to play a tile. You can draw tiles with the `Draw` button, or exchange all current tiles with the `Exchange` button.

There is no dictionary, or limit to where you can place your tiles. It's on the honor system. The score displayed doesn't actually account for double or triple word scores yet, sorry.

## Roadmap

* Game:
 ** Add word scores to scoring algorithm
 ** Add dictionary of allowed words
 ** Add wildcard tiles
 ** Add restrictions to only place tiles on allowed spaces
 ** Add game end conditions
