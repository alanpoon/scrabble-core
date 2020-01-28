use crate::dawg::{Dawg, DawgEdge};
use std::mem::size_of;

pub const A_INDEX: u8 = 97;

lazy_static! {
    // Use &*DAWG to access a global instance of the dawg
    // this makes it easier to hide this implementation detail in an externally-facing API
    pub static ref DAWG: Dawg = parse_dawg();
}

pub fn load_dawg() -> &'static Dawg {
    &*DAWG
}

fn parse_dawg() -> Dawg {
    // The following results in a smaller binary, but requires the file to be locally available
    //use std::fs;
    //let bytes = fs::read("assets/dawg.bin").expect("Couldn't load asserts/dawg.bin");
    static DAWG_BYTES: &'static [u8] = include_bytes!("../assets/dawg.bin");
    let bytes = DAWG_BYTES;

    let u64_size = size_of::<u64>();
    let n_dawg_edges = bytes.len() / u64_size;
    let mut edges: Vec<DawgEdge> = Vec::with_capacity(n_dawg_edges);
    for i in 0..n_dawg_edges {
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
    use crate::dawg::DawgNodeIndex;

    #[test]
    fn test_load_dawg_data() {
        let dawg_data = &load_dawg().edges;
        assert_eq!(dawg_data.len(), 190446);
        assert_eq!(
            dawg_data[0],
            DawgEdge {
                letter: 'a',
                word_terminator: false,
                target: DawgNodeIndex(26),
                node_terminator: false,
            }
        );
        assert_eq!(
            dawg_data[1000],
            DawgEdge {
                letter: 'l',
                word_terminator: false,
                target: DawgNodeIndex(136),
                node_terminator: false,
            }
        );
    }
}

#[cfg(all(test, feature = "unstable"))]
mod benches {
    extern crate test;

    use super::*;

    #[bench]
    pub fn bench_parse_dawg(b: &mut test::Bencher) {
        let mut dawg: Option<Dawg> = None;
        b.iter(|| dawg = Some(parse_dawg()));
        // Actually the dawg to ensure the loading doesn't get compiled out; maybe this is unnecessary
        assert!(dawg.unwrap().contains("hello"))
    }
}
