mod mcts_classic;
use core::num;

use mcts_classic::*;

mod game_state_c4;
use game_state_c4::*;

use std::io;


fn main() {
    let num_positions: usize = 100000;
    let starting_position = GSC4::make_blank();
    let mut game_tree: GTree; 
    // implement tree recycling 

    let mut temp_position: GSC4 = starting_position.clone(); 

    temp_position.print_board();

    loop { 

        let mv: usize; 
        if temp_position.is_player_one() { 
            game_tree = GTree::new(num_positions, &temp_position);
            mv = game_tree.make_move();

        } else { 
            // take user input 
            println!("User input > ");
            let mut input = String::new(); 
            io::stdin().read_line(&mut input).expect("Failed to read input"); 
            mv = input.trim().parse::<usize>().unwrap();
        }

        if temp_position.move_from_int(mv).board_full() { 
            println!("DRAW"); 
            temp_position = temp_position.move_from_int(mv);
            temp_position.print_board();
            break; 
        } else if temp_position.winning_move(mv) { 
            let winner_token = if temp_position.is_player_one() { "x" } else { "o" };
            println!("<><><><> {} wins! <><><><>", winner_token);
            temp_position = temp_position.move_from_int(mv);
            temp_position.print_board();
            break; 
        } 
        temp_position = temp_position.move_from_int(mv);
        temp_position.print_board();
    } 
}


