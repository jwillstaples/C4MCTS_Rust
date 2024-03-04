use std::io; 

pub struct GSC4 { 
    board: [[[bool; 2]; 6]; 7], 
    player_one: bool,
}

impl GSC4 { 

    pub fn clone(&self) -> GSC4 { 
        return GSC4 { 
            board: self.board.clone(), 
            player_one: self.player_one,
        };
    }
    
    pub fn is_player_one(&self) -> bool { return self.player_one; }

    fn col_full(&self, col: usize) -> bool { 
        return self.board[col][5][0] || self.board[col][5][1]; 
    }

    pub fn board_full(&self) -> bool { 
        for i in 0..7 {
            if ! self.col_full(i) { return false}
        } return true; 
    } 

    pub fn legal_moves(&self) -> [bool; 7] { 
        let mut legals = [false; 7]; 
        for i in 0..7 { 
            legals[i] = ! self.col_full(i);
        } return legals;
    }

    pub fn legal_moves_vec(&self) -> Vec<usize> {
        let mut legals = Vec::with_capacity(7); 
        // assert!(legals.len()==0);
        for i in 0..7 { 
            if !self.col_full(i) { legals.push(i)}
        } return legals; 
    } 

    pub fn winning_move(&self, mv: usize) -> bool { 
        let mover_index = if self.player_one { 0 } else { 1 }; 
        let mut row_ind: usize = 0; 
        for row in 0..6 {
            if !self.board[mv][row][0] && !self.board[mv][row][1] { 
                row_ind = row; 
                break; 
            }
        }

        let mut consec_counter: u8;

        // check vertical wins
        if row_ind > 2 { 
            if self.board[mv][row_ind-1][mover_index]
                && self.board[mv][row_ind-2][mover_index]
                && self.board[mv][row_ind-3][mover_index] {
                // println!("vertical win detected");
                return true;
            }
        }
        
        // check horizontal and diagonal wins
        for dy_dx in -1i32..=1 { 
            consec_counter = 0; 
            for dx in [-1i32, 1].iter() { 
                let mut col = mv as i32 + dx; 
                let mut row = row_ind as i32 + dx*dy_dx; 
                loop { 
                    if col < 0 
                        || col > 6 
                        || row < 0
                        || row > 5
                        || !self.board[col as usize][row as usize][mover_index]
                    {
                        break; 
                    }
                    col += dx; 
                    row += dx * dy_dx; 
                    consec_counter += 1;
                }
            } 
            if consec_counter > 2 { return true; }
        }
        return false;
    }

    pub fn print_board(&self) { 
        for row in (0..6).rev() { 
            print!("|");
            for col in 0..7 {
                if self.board[col][row][0] {
                    print!("x");
                } else if self.board[col][row][1] {
                    print!("o");
                } else {
                    print!(" ");
                } print!("|")
            } println!("")
        } println!(" 0 1 2 3 4 5 6 ");
    }

    pub fn move_from_int(&self, mv: usize) -> GSC4 {

        assert!(mv < 7); 

        assert!(!self.col_full(mv));
        let mut new_board = self.board.clone(); 
        let mover_index = if self.player_one { 0 } else { 1 }; 

        for row in 0..6 { 
            if !self.board[mv][row][0] && !self.board[mv][row][1] { 
                new_board[mv][row][mover_index] = true; 
                break; 
            }
        }
        let new_gs = GSC4 { 
            board: new_board,
            player_one: ! self.player_one,
        }; return new_gs;
    }

    pub fn make_blank() -> GSC4 {
        let blank_gs = GSC4 {
            board: [[[false; 2]; 6]; 7],
            player_one: true,
        }; 
        return blank_gs;
    }
}

fn main() { 
    let mut game_state = GSC4::make_blank(); 
    game_state.print_board(); 

    loop {

        // take user input 
        println!("User input > ");
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).expect("Failed to read input"); 
        let mv = input.trim().parse::<usize>().unwrap();

        println!("checkpoint");

        if game_state.winning_move(mv) { 
            game_state.move_from_int(mv).print_board();
            let winner_token = if game_state.player_one { "x" } else { "o" };
            println!("<><><><> {} wins! <><><><>", winner_token);
            break;
        }

        game_state = game_state.move_from_int(mv);
        game_state.print_board();

        if game_state.board_full() { break; }

    }
}