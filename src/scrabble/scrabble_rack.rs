use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ScrabbleRack {
    tiles: HashMap<char, usize>,
}

impl ScrabbleRack {
    pub fn new() -> ScrabbleRack {
        ScrabbleRack {
            tiles: HashMap::new(),
        }
    }

    pub fn add_tile(&mut self, tile: char) {
        *self.tiles.entry(tile).or_insert(0) += 1;
    }

    pub fn take_tile(&mut self, tile: char) -> Result<(), ()> {
        match self.tiles.entry(tile).or_insert(0) {
            count if *count > 0 => {
                *count -= 1;
                Ok(())
            }
            _ => Err(()),
        }
    }
}
