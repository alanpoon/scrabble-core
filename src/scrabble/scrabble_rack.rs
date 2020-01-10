use crate::scrabble::scrabble_board_square::ScrabbleTile;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ScrabbleRack {
    pub tiles: HashMap<ScrabbleTile, usize>,
}

impl ScrabbleRack {
    pub fn take_tile(&mut self, tile: ScrabbleTile) -> Result<(), ()> {
        match self.tiles.entry(tile).or_insert(0) {
            count if *count > 0 => {
                *count -= 1;
                Ok(())
            }
            _ => Err(()),
        }
    }

    pub fn replace_tile(&mut self, tile: ScrabbleTile) {
        *self.tiles.entry(tile).or_insert(0) += 1;
    }
}
