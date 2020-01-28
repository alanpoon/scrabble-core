#![cfg_attr(all(test, feature = "unstable"), feature(test))]
#[macro_use]
extern crate lazy_static;

pub use crate::dawg::Dawg;
pub use crate::game::{
    Direction, PlayGenerator, Position, ScoredScrabblePlay, ScrabbleBoard, ScrabblePlay,
    ScrabbleRack,
};
pub use crate::loading::load_dawg;

mod dawg;
mod game;
mod loading;

pub fn board_from_contents(contents: &str) -> ScrabbleBoard {
    ScrabbleBoard::from_contents(&contents).expect("Invalid input")
}

pub fn board_from_plays(plays: &Vec<ScrabblePlay>) -> ScrabbleBoard {
    let mut board = ScrabbleBoard::default();
    for play in plays {
        board.add_play(play);
    }
    board
}

pub fn generate_plays(
    rack_contents: &str,
    board: &ScrabbleBoard,
    max_n_plays: usize,
) -> Vec<ScoredScrabblePlay> {
    let dawg = load_dawg();
    let rack = ScrabbleRack::new(rack_contents);
    let checked_board = board.to_checked_board(dawg);
    let generator = PlayGenerator {
        dawg,
        checked_board,
        rack,
    };
    let mut plays = generator.plays();
    plays.sort_by_key(|x| -x.score);
    plays.truncate(max_n_plays);
    plays
}

#[cfg(test)]
mod test {
    use crate::{Direction, Position, ScrabblePlay};

    use super::*;

    #[test]
    pub fn test_play_gen_1() {
        let best_play = best_play_for_test_board("abcdefg");
        assert_eq!(best_play.play.start, Position { row: 6, col: 6 });
        assert_eq!(best_play.play.direction, Direction::Horizontal);
        assert_eq!(best_play.play.word, "cafe");
        assert_eq!(best_play.score, 32);
    }

    #[test]
    pub fn test_play_gen_2() {
        let best_play = best_play_for_test_board("abcde__");
        assert_eq!(best_play.play.start, Position { row: 3, col: 10 });
        assert_eq!(best_play.play.direction, Direction::Vertical);
        assert_eq!(best_play.play.word, "becalmed");
        assert_eq!(best_play.score, 110);
    }

    fn best_play_for_test_board(rack_contents: &str) -> ScoredScrabblePlay {
        let board = get_test_board();
        best_play_for_board(rack_contents, &board)
    }

    fn get_test_board() -> ScrabbleBoard {
        let existing_plays: Vec<ScrabblePlay> = vec![ScrabblePlay {
            start: Position { row: 7, col: 7 },
            direction: Direction::Horizontal,
            word: "hello".to_string(),
        }];
        board_from_plays(&existing_plays)
    }

    fn best_play_for_board(rack_contents: &str, board: &ScrabbleBoard) -> ScoredScrabblePlay {
        let max_n_plays = 5;
        let plays = generate_plays(rack_contents, board, max_n_plays);
        dbg!(&plays);
        assert_eq!(plays.len(), max_n_plays);
        plays[0].clone()
    }
}

#[cfg(all(test, feature = "unstable"))]
mod benches {
    extern crate test;

    use crate::load_dawg;

    use super::test::*;

    #[bench]
    pub fn bench_play_gen_1(b: &mut test::Bencher) {
        load_dawg();
        b.iter(|| test_play_gen_1());
    }

    #[bench]
    pub fn bench_play_gen_2(b: &mut test::Bencher) {
        load_dawg();
        b.iter(|| test_play_gen_2());
    }
}
