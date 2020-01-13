use std::fs;

use crate::data_structures::{Dawg, DawgEdge};
use std::mem::size_of;

pub fn load_dawg() -> Dawg {
    let bytes = fs::read("assets/dawg.bin").expect("Couldn't load asserts/dawg.bin");
    let mut edges: Vec<DawgEdge> = Vec::new();
    let u64_size = size_of::<u64>();
    for i in 0..bytes.len() / u64_size {
        let mut num: u64 = 0;
        for j in 0..u64_size {
            let byte_index = u64_size * i + j;
            num += (bytes[byte_index] as u64) << (8 * j) as u64;
        }
        edges.push(num.into());
    }
    Dawg { edges }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data_structures::DawgNodeIndex;

    #[test]
    fn test_load_dawg_data() {
        let dawg_data = load_dawg().edges;
        assert_eq!(dawg_data.len(), 190446);
        assert_eq!(
            dawg_data[0],
            DawgEdge {
                letter: 'a',
                word_terminator: false,
                target: Some(DawgNodeIndex(26)),
                node_terminator: false,
            }
        );
        assert_eq!(
            dawg_data[1000],
            DawgEdge {
                letter: 'l',
                word_terminator: false,
                target: Some(DawgNodeIndex(136)),
                node_terminator: false,
            }
        );
    }
}
