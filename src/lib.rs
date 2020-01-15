use crate::data_structures::Dawg;
pub use crate::game::play_generation::ScrabblePlay;
use crate::game::play_generation::{PlayGenerator, ScoredScrabblePlay};
pub use crate::game::util::{Direction, Position};
use crate::game::{ScrabbleBoard, ScrabbleRack};
pub use crate::loading::load_dawg;

mod data_structures;
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
    dawg: &Dawg,
    rack_contents: &str,
    board: &ScrabbleBoard,
    max_n_plays: usize,
) -> Vec<ScoredScrabblePlay> {
    let rack = ScrabbleRack::new(rack_contents);
    let checked_board = board.to_checked_board(&dawg);
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
    use crate::{load_dawg, Direction, Position, ScrabblePlay};

    use super::*;

    #[test]
    fn test_play_gen_1() {
        let best_play = best_play_for_test_board("abcdefg");
        assert_eq!(best_play.play.start, Position { row: 6, col: 6 });
        assert_eq!(best_play.play.direction, Direction::Horizontal);
        assert_eq!(best_play.play.word, "cafe");
        assert_eq!(best_play.score, 32);
    }

    #[test]
    fn test_play_gen_2() {
        let best_play = best_play_for_test_board("abcde__");
        assert_eq!(best_play.play.start, Position { row: 3, col: 10 });
        assert_eq!(best_play.play.direction, Direction::Vertical);
        assert_eq!(best_play.play.word, "becalmed");
        assert_eq!(best_play.score, 110);
    }

    fn best_play_for_test_board(rack_contents: &str) -> ScoredScrabblePlay {
        let dawg = load_dawg();
        let board = get_test_board();
        best_play_for_board(&dawg, rack_contents, &board)
    }

    fn get_test_board() -> ScrabbleBoard {
        let existing_plays: Vec<ScrabblePlay> = vec![ScrabblePlay {
            start: Position { row: 7, col: 7 },
            direction: Direction::Horizontal,
            word: "hello".to_string(),
        }];
        board_from_plays(&existing_plays)
    }

    fn best_play_for_board(
        dawg: &Dawg,
        rack_contents: &str,
        board: &ScrabbleBoard,
    ) -> ScoredScrabblePlay {
        let max_n_plays = 5;
        let plays = generate_plays(dawg, rack_contents, board, max_n_plays);
        dbg!(&plays);
        assert_eq!(plays.len(), max_n_plays);
        plays[0].clone()
    }
}
