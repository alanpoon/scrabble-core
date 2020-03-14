use crate::game::play_generation::GenerationAisle;
use crate::game::util::{Position, BLANK_TILE_CHAR, EMPTY_SQUARE_CHAR};
use hardback_boardstruct::codec_lib::codec::{Player};
use hardback_boardstruct::codec_lib::cards::{self,WaitForInputType};
use hardback_boardstruct::resolve_cards;
use hardback_boardstruct::board::BoardStruct;

pub fn score_play(aisle: &GenerationAisle, start_word_index: usize, 
    arranged: Vec<(usize,bool,Option<String>,bool)>,cardmeta:&[cards::ListCard<BoardStruct>; 180]) -> (usize,i8) {
    let mut position = aisle.position(start_word_index);
    let mut wait_for_input: [WaitForInputType; 4] = [vec![], vec![], vec![], vec![]];
    let mut board = BoardStruct::new(vec![create_player("p1".to_string(),arranged),create_player("p2".to_string(),vec![])],&vec![]);
    resolve_cards::resolve_cards(&mut board,0,cardmeta,&mut wait_for_input);
    let mut p2 = board.players[1].clone();
    let mut v1 = vec![1,2,3];
    let mut v2 = vec![1,2,3];
    let mut _wait_vec_vec = &mut wait_for_input[0];
    for x in 0.._wait_vec_vec.len(){
        if let Some(_wait_vec) = _wait_vec_vec.remove(0) {
            if let Some(&(ref next_gstate, ref log, ref _closure)) =
                _wait_vec.3.get(0) {
                println!("log {:?}",log);
                (*_closure)(&mut p2,
                            &mut v1,
                            &mut v2);
            }
        }
    }
    let p1 = board.players[0].clone();
    
    action_space(p1,p2)
}
fn create_player(name:String,arranged:Vec<(usize,bool,Option<String>,bool)>)->Player{
    Player{
        name: name,
        vp: 0,
        coin: 0,
        ink: 5,
        remover: 5,
        literacy_award: 0,
        arranged: arranged,
        vec_of_cards_to_decide: vec![],
        hand: vec![],
        draft: vec![],
        draftlen: 5,
        lockup: vec![],
        timeless_classic: vec![],
        skip_cards: vec![],
        discard:vec![]
    }
}
fn action_space(player:Player,player2:Player)->(usize,i8){
    let d_vp = player.vp as i8;
    let d_coin = player.coin as i8;
    let d_ink:i8 = player.ink as i8 -5;
    
    let d_remover:i8 = player.remover as i8 - 5;
    let d_literacy_award:i8 = player.literacy_award as i8;
    let d_lockup:i8 = player.lockup.len() as i8;
    let d_discard:i8 = player.discard.len() as i8;
    let minus_ink:i8 = 5-player2.ink as i8;
    let minus_remover:i8 = 5-player2.remover as i8;
    let mut sortable:[[i8;2];9] = [[0,d_vp],[1,d_coin],[2,d_ink],
    [3,d_remover],[4,d_literacy_award],[5,d_lockup],[6,d_discard],[7,minus_ink],[8,minus_remover]];
    sortable.sort_by(|a, b| b[1].partial_cmp(&a[1]).unwrap());
    (sortable[0][0] as usize,sortable[0][1])
}