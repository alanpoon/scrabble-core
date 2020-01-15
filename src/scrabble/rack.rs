use crate::scrabble::util::BLANK_TILE_CHAR;

const N_LETTERS: usize = 26;
const N_TILES: usize = N_LETTERS + 1;
const BLANK_TILE_INDEX: usize = 26;

#[derive(Debug, Clone)]
pub struct ScrabbleRack {
    tile_counts: [u8; N_TILES],
}

impl ScrabbleRack {
    pub fn new(tiles: &str) -> ScrabbleRack {
        let mut rack = ScrabbleRack {
            tile_counts: [0; N_TILES],
        };
        rack.add_tiles(tiles);
        rack
    }

    pub fn add_tile(&mut self, tile: char) {
        let index = ScrabbleRack::tile_index(tile);
        *&mut self.tile_counts[index] += 1;
    }

    pub fn take_tile(&mut self, tile: char) -> Result<char, ()> {
        let index = ScrabbleRack::tile_index(tile);
        if self.tile_counts[index] > 0 {
            *&mut self.tile_counts[index] -= 1;
            Ok(tile)
        } else if self.tile_counts[BLANK_TILE_INDEX] > 0 {
            *&mut self.tile_counts[BLANK_TILE_INDEX] -= 1;
            Ok(BLANK_TILE_CHAR)
        } else {
            Err(())
        }
    }

    fn tile_index(tile: char) -> usize {
        const A_INDEX: u8 = 97;
        (match tile {
            tile if tile.is_ascii_lowercase() => tile as u8 - A_INDEX,
            BLANK_TILE_CHAR => BLANK_TILE_INDEX as u8,
            _ => panic!(
                "Invalid rack tile: '{}'. Should be in 'a'-'z', or '_' for blank",
                tile
            ),
        }) as usize
    }

    fn add_tiles(&mut self, tiles: &str) {
        for tile in tiles.chars() {
            self.add_tile(tile);
        }
    }
}
