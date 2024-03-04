use crate::game_state_c4::*;
use indicatif::ProgressBar; 

use rand::*;

// use std::alloc::{alloc, Layout};

struct PNode { 
    cum_eval: f64, 
    num_visits: f64, 
    game_state: GSC4,

    parent_ind: Option<usize>, 
    own_ind: usize,
    first_child_ind: Option<usize>, 
}

impl PNode {

    fn clone(&self) -> PNode {
        return PNode { 
            cum_eval: self.cum_eval,
            num_visits: self.num_visits,
            game_state: self.game_state.clone(),
            parent_ind: self.parent_ind,
            own_ind: self.own_ind,
            first_child_ind: self.first_child_ind,
        }
    }

    fn value_score(&self) -> f64 {
        if self.num_visits == 0.0 { 
            return 0.0; 
        } else {
            return self.cum_eval / self.num_visits;
        }
    }
}

pub struct GTree { 
    game_arr: Vec<Option<PNode>>,  // Vec::with_capacity()
    depth: usize,
}

impl GTree { 
    pub fn new(max_depth: usize, root_position: &GSC4) -> Self { 

        // let game_arr_layout = Layout::array::<PNode>(6*max_depth).expect("Error in Game Array Memory Allocation");
        // let mut game_arr_ptr = unsafe { alloc(game_arr_layout)} as *mut PNode; 
        let root_position_new: GSC4 = root_position.clone();
        let game_arr = Vec::with_capacity(6*max_depth); 
        let mut gtree = GTree { 
            game_arr: game_arr, 
            depth: max_depth,
        };
        gtree.set(Some(PNode{
            cum_eval: 0.0, 
            num_visits: 0.0, 
            game_state: root_position_new, 
            parent_ind: None, 
            own_ind: 0 as usize, 
            first_child_ind: None,
        })); 
        return gtree;
    }

    fn get(&self, index: usize) -> Option<&PNode> { 
        //Some(*self.game_arr.offset(index as isize))
        return self.game_arr[index].as_ref();
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut PNode> {
        return self.game_arr[index].as_mut();
    }

    fn set(&mut self, value: Option<PNode>) {
        // *self.game_arr.offset(self.book_mark as isize) = value;
        self.game_arr.push(value);
    }

    fn backpropagate(&mut self, node_ind: usize, eval: f64) {
        let mut temp_index = node_ind; 
        loop {
            let temp_node = self.get_mut(temp_index).unwrap();
            temp_node.cum_eval += eval; 
            temp_node.num_visits += 1.0; 
            if temp_node.parent_ind.is_none() { 
                break 
            } else {
                temp_index = temp_node.parent_ind.unwrap(); 
            }
        }
    }

    fn select_leaf(&self) -> usize { 
        let mut candidate_node_ind: usize = 0; // sets candidate node as the root node 
        loop { 
            let candidate_node_ptr = self.get(candidate_node_ind).unwrap();
            if candidate_node_ptr.first_child_ind.is_none() {
                return candidate_node_ptr.own_ind
            } else {
                let mut favorite_child_ind: usize = 0; // arbitrary value, should never be read
                let mut max_score = -f64::INFINITY; 
                for i in 1..7 { 
                    let next_child = self.get((*candidate_node_ptr).first_child_ind.unwrap() + i); 
                    if next_child.is_none() { continue } 
                    let next_score = 1.5 * ((*candidate_node_ptr).num_visits).sqrt() / (1.0 + next_child.unwrap().num_visits) - (next_child.unwrap().value_score());
                    if max_score < next_score {
                        favorite_child_ind = next_child.unwrap().own_ind; 
                        max_score = next_score;
                    }
                } if max_score > -f64::INFINITY {
                    candidate_node_ind = favorite_child_ind; 
                } else { 
                    return candidate_node_ptr.own_ind; // accounts for possibility of their being no legal moves 
                }
            }
        }
    }

    fn evaluate_leaf(&self, leaf_ind: usize) -> f64 {
        return self.rollout(&self.get(leaf_ind).unwrap().game_state);
    }

    fn conceive_children(&mut self, leaf_ind: usize, book_mark: usize) { 
        let leaf: &mut PNode = self.get_mut(leaf_ind).unwrap(); 
        assert!(leaf.first_child_ind.is_none());
        leaf.first_child_ind = Some(book_mark);
    }

    fn expand_leaf(&mut self, leaf_ind: usize, book_mark: usize) {

        self.conceive_children(leaf_ind, book_mark);
        
        let leaf = self.get(leaf_ind).unwrap().clone();
        let legals = leaf.game_state.legal_moves();

        for i in 0..7 { 
            if legals[i] { 
                self.set(Some(PNode{
                    cum_eval: 0.0, 
                    num_visits: 0.0, 
                    game_state: leaf.game_state.move_from_int(i), 
                    parent_ind: Some(leaf.own_ind), 
                    own_ind: book_mark + i, 
                    first_child_ind: None, 
                }));
            } else { 
                self.set(None);
            }
        }
         
    }

    pub fn rollout(&self, game_state: &GSC4) -> f64 { 

        let mut rng = thread_rng(); 

        let mut temp_state: GSC4 = game_state.clone(); 
        loop {
            let legals = temp_state.legal_moves_vec();
            if legals.len() > 0 {
                // println!("legals.len() = {}", legals.len());
                let mv = legals[rng.gen_range(0..legals.len())];
                if temp_state.winning_move(mv) { 
                    return if game_state.is_player_one() != temp_state.is_player_one() {-1.0} else {1.0};  
                } temp_state = temp_state.move_from_int(mv);
            } else { 
                assert!(temp_state.board_full());
                break;
            }
        } return 0.0;
    }

    pub fn make_move(&mut self) -> usize { 

        let depth = self.depth as u64; 
        let bar = ProgressBar::new(depth); 
        for i in 0..=self.depth { 

            bar.inc(1);

            // println!("checkpoint 1");
            let mut next_node_ind = self.select_leaf(); 
            // println!("checkpoint 2");
            let eval = self.evaluate_leaf(next_node_ind); 
            // println!("checkpoint 3");
            self.backpropagate(next_node_ind, eval);
            // println!("checkpoint 4");
            self.expand_leaf(next_node_ind, self.game_arr.len());
            // println!("checkpoint 5");

        }
        let mut favorite_child: Option<&PNode> = None; 
        let mut max_score: f64 = -f64::INFINITY; 
        for i in 1..=7 { 
            let temp_node: Option<&PNode> = self.get(i); 
            if temp_node.is_none() { 
                continue;
            } else if temp_node.unwrap().value_score() > max_score { 
                favorite_child = temp_node; 
                max_score = temp_node.unwrap().value_score();
            }
        }
        if favorite_child.is_none() { 
            panic!("Position has no legal moves");
        } else {
            return favorite_child.unwrap().own_ind - 1; 
        }
    }
}

fn main() { 
    print!("Main Function")
}