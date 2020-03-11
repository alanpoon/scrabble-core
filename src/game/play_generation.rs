use crate::dawg::{Dawg, DawgEdge, DawgNodeIndex};
use crate::game::scoring::score_play;
use crate::game::util::{Direction, Position};
use crate::game::{CheckedAisleSquare, CheckedScrabbleBoard, BOARD_SIZE};
use hardback_codec::cards::{self,WaitForInputType,Board};

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct ScoredScrabblePlay {
    pub play: ScrabblePlay,
    pub score: i32,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct ScrabblePlay {
    pub start: Position,
    pub direction: Direction,
    pub word: String,
}

pub struct PlayGenerator<'a> {
    pub dawg: &'a Dawg,
    pub checked_board: CheckedScrabbleBoard,
    pub rack: Vec<usize>,
}

impl<'a> PlayGenerator<'a> {
    pub fn plays(&self) -> Vec<ScoredScrabblePlay> {
        let mut plays: Vec<ScoredScrabblePlay> = Vec::new();
        for aisle in self.generation_aisles().iter() {
            
            for (anchor_index, tile) in aisle.squares.iter().enumerate() {
             //   if tile.is_anchor {
                println!("generation_aisles().tile() {:?}",tile);
                    let solving_anchor = GenerationAnchor {
                        dawg: &self.dawg,
                        aisle,
                        anchor_index,
                    };
                plays.extend(solving_anchor.scored_plays(&self.rack));
             //   }
            }
        }
        println!("player {:?}",plays.len());
        plays
    }

    fn generation_aisles(&self) -> Vec<GenerationAisle> {
        let board = &self.checked_board;
        let mut solving_rows: Vec<GenerationAisle> = Vec::with_capacity(2 * 1);
        for index in 0..1 {
            for &direction in Direction::iterator() {
                let solving_row = GenerationAisle {
                    direction,
                    index,
                    squares: board.aisle(direction, index),
                };
                solving_rows.push(solving_row);
            }
        }
        solving_rows
    }
}

#[derive(Debug)]
pub struct GenerationAisle {
    pub direction: Direction,
    pub index: usize,
    pub squares: [CheckedAisleSquare; BOARD_SIZE],
}

impl GenerationAisle {
    pub fn scored_play(&self, start_word_index: usize, word: String) -> ScoredScrabblePlay {
        let start = self.position(start_word_index);
        let score = score_play(&self, start_word_index, &word);
        let play = ScrabblePlay {
            start,
            direction: self.direction,
            word,
        };
        ScoredScrabblePlay { play, score }
    }

    pub fn position(&self, cross: usize) -> Position {
        Position::from_aisle_cross(self.direction, self.index, cross)
    }
}
pub struct BoardStruct{
}
impl Board for BoardStruct{
    fn two_cent_per_adv(&mut self,
        player_id: usize,
        card_index: usize,
        wait_for_input: &mut [WaitForInputType; 4]){

        }
    fn minus_other_ink(&mut self,
        player_id: usize,
        card_index: usize,
        wait_for_input: &mut [WaitForInputType; 4]){
            
        }
    fn lockup_offer(&mut self,
        player_id: usize,
        card_index: usize,
        wait_for_input: &mut [WaitForInputType; 4]){}
    fn uncover_adjacent(&mut self,
                    player_id: usize,
                    card_index: usize,
                    wait_for_input: &mut [WaitForInputType; 4]){}
    fn double_adjacent(&mut self,
                player_id: usize,
                card_index: usize,
                wait_for_input: &mut [WaitForInputType; 4]){}
    fn trash_other(&mut self,
            player_id: usize,
            card_index: usize,
            wait_for_input: &mut [WaitForInputType; 4]){}
    fn one_vp_per_wild(&mut self,
                player_id: usize,
                card_index: usize,
                wait_for_input: &mut [WaitForInputType; 4]){}
    fn putback_or_discard_three(&mut self,
                            player_id: usize,
                            card_index: usize,
                            wait_for_input: &mut [WaitForInputType; 4]){}
}
struct GenerationState {
    plays: Vec<ScoredScrabblePlay>,
    rack: Vec<usize>,
    partial_word: String,
    cardmeta:[cards::ListCard<BoardStruct>; 180]
}

struct GenerationAnchor<'a> {
    dawg: &'a Dawg,
    aisle: &'a GenerationAisle,
    anchor_index: usize,
}

impl<'a> GenerationAnchor<'a> {
    pub fn scored_plays(&self, rack: &Vec<usize>) -> Vec<ScoredScrabblePlay> {
        let initial_state = self.initial_state(rack);
        if let Ok((mut state, node)) = initial_state {
            let initial_limit = self.initial_limit();
            self.add_plays_for_left(&mut state, node, initial_limit);
            state.plays
        } else {
            Vec::new()
        }
    }

    fn initial_state(&self, rack: &Vec<usize>) -> Result<(GenerationState, DawgNodeIndex), ()> {
        let left_part_start_index = self.left_part_start_index();
        let (partial_word, maybe_node) = self.initial_left_part(left_part_start_index);
        
        if maybe_node.is_some() {
            let play_generation_state = GenerationState {
                plays: Vec::new(),
                rack: rack.clone(),
                partial_word,
                cardmeta: cards::populate::<BoardStruct>()
            };
            Ok((play_generation_state, maybe_node))
        } else {
            Err(()) // No words can be built starting with the initial left part
        }
    }

    fn left_part_start_index(&self) -> usize {
        for (possible_first, square) in self.aisle.squares[..self.anchor_index]
            .iter()
            .enumerate()
            .rev()
        {
            if square.tile.is_none() {
                return possible_first + 1;
            }
        }
        0
    }

    fn initial_left_part(&self, left_part_start_index: usize) -> (String, DawgNodeIndex) {
        let mut partial_word = String::with_capacity(BOARD_SIZE);
        let mut node = self.dawg.root();
        for square in self.aisle.squares[left_part_start_index..self.anchor_index].iter() {
            let ch = square.tile.unwrap();
            partial_word.push(ch);
            if node.is_some() {
                node = self
                    .dawg
                    .leaving_edge(node, ch)
                    .map(|edge| edge.target)
                    .unwrap_or(DawgNodeIndex(0));
            }
        }
        (partial_word, node)
    }

    fn initial_limit(&self) -> usize {
        for limit in 0..self.anchor_index {
            let square = &self.aisle.squares[self.anchor_index - (limit + 1)];
            if square.tile.is_some() || square.is_anchor {
                return limit;
            }
        }
        self.anchor_index
    }

    fn add_plays_for_left(&self, state: &mut GenerationState, node: DawgNodeIndex, limit: usize) {
        self.extend_right(state, node, self.anchor_index);
        if limit > 0 {
            self.dawg.apply_to_child_edges(node, |edge| {
                let target = edge.target;
                println!("target {:?}",edge.letter);
                if target.is_some() {
                    for (i,card_index) in state.rack.clone().iter().enumerate(){
                        let card_letter_vec:Vec<char> = state.cardmeta[*card_index].letter.chars().collect();
                        if card_letter_vec[0]==edge.letter{
                            state.rack.remove(i);
                            state.partial_word.push(edge.letter);
                            self.add_plays_for_left(state,target,limit -1);
                            state.partial_word.pop();
                            state.rack.push(*card_index);
                            break;
                        }
                    }
                }
            });
        }
    }

    fn extend_right(
        &self,
        state: &mut GenerationState,
        node: DawgNodeIndex,
        next_tile_index: usize,
    ) {
        if next_tile_index >= BOARD_SIZE {
            return;
        }
        let next_square = &self.aisle.squares[next_tile_index];
        if let Some(ch) = next_square.tile {
            if let Some(edge) = self.dawg.leaving_edge(node, ch) {
                self.extend_using_edge(state, next_tile_index, edge);
            }
        } else {
            self.dawg.apply_to_child_edges(node, |edge| {
                for (i,card_index) in state.rack.clone().iter().enumerate(){
                    let card_letter_vec:Vec<char> = state.cardmeta[*card_index].letter.chars().collect();
                    if card_letter_vec[0]==edge.letter{
                        state.rack.remove(i);
                        if next_square.is_compatible(edge.letter){
                            self.extend_using_edge(state,next_tile_index,edge);
                        }
                        state.rack.push(*card_index);
                        break;
                    }
                }
            })
        }
    }

    fn extend_using_edge(
        &self,
        state: &mut GenerationState,
        placement_index: usize,
        edge: &DawgEdge,
    ) {
        state.partial_word.push(edge.letter);
        self.check_add_play(state, edge, placement_index + 1);
        let target = edge.target;
        if target.is_some() {
            self.extend_right(state, target, placement_index + 1);
        }
        state.partial_word.pop();
    }

    fn check_add_play(
        &self,
        state: &mut GenerationState,
        edge: &DawgEdge,
        next_square_index: usize,
    ) {
        if next_square_index < BOARD_SIZE && self.aisle.squares[next_square_index].tile.is_some() {
            return; // there is a tile in the next square, so this partial_word isn't a valid play
        }
        if next_square_index < self.anchor_index + 1 {
            return; // Haven't placed anything in the anchor index
        }

        if edge.word_terminator {
            let start = next_square_index - state.partial_word.len();
            let play = self
                .aisle
                .scored_play(start, state.partial_word.to_string());
            state.plays.push(play)
        }
    }
}
