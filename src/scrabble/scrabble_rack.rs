const N_LETTERS: usize = 26;
const N_TILES: usize = N_LETTERS + 1;
const BLANK_TILE_INDEX: usize = 26;

#[derive(Debug, Clone)]
pub struct ScrabbleRack {
    tile_counts: [u8; N_TILES],
}

impl ScrabbleRack {
    pub fn new() -> ScrabbleRack {
        ScrabbleRack {
            tile_counts: [0; N_TILES],
        }
    }

    fn tile_index(tile: char) -> usize {
        const A_INDEX: u8 = 97;
        let char_offset = (tile as u8).wrapping_sub(A_INDEX) as usize;
        match char_offset {
            offset if offset < N_LETTERS => offset,
            _ => BLANK_TILE_INDEX,
        }
    }

    pub fn add_tile(&mut self, tile: char) {
        let index = ScrabbleRack::tile_index(tile);
        *&mut self.tile_counts[index] += 1;
    }

    pub fn add_tiles(&mut self, tiles: &str) {
        for tile in tiles.chars() {
            self.add_tile(tile);
        }
    }

    pub fn take_tile(&mut self, tile: char) -> Result<char, ()> {
        let index = ScrabbleRack::tile_index(tile);
        if self.tile_counts[index] > 0 {
            *&mut self.tile_counts[index] -= 1;
            Ok(tile)
        } else if self.tile_counts[BLANK_TILE_INDEX] > 0 {
            *&mut self.tile_counts[BLANK_TILE_INDEX] -= 1;
            Ok(' ')
        } else {
            Err(())
        }
    }
}
