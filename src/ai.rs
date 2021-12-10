/*
    A basic artificial intelligence for the connect-4 game. This game has been
    formally solved, and it is possible to write a solver that will always win
    if it plays first. This artificial intelligence, however, is much 
    more simple and can be beaten. 
*/

use std::convert::TryFrom;
use crate::board;
use rand::prelude::*;

fn can_i_win_in(n_turns: usize, player: u8, the_board: &board::Board, last_col_played: usize) -> bool {

    if n_turns == 0 {
        return board::just_won(the_board, player, last_col_played);
    }

    let mut i_can_win;
    let mut the_board_2: board::Board;
    let mut the_board_3: board::Board;
    
    for col_opp_next in 1..8 {
        the_board_2 = the_board.copy();
        board::add(&mut the_board_2, (3-player).into(), col_opp_next);
        if board::just_won(&the_board_2, 3-player, col_opp_next) {
            return false;
        }
        i_can_win = false;
        for col_next in 1..8 {
            the_board_3 = the_board_2.copy();
            board::add(&mut the_board_3, player.into(), col_next);
            if can_i_win_in(n_turns-1, player, &the_board_3, col_next) {
                i_can_win = true;
                break;
            }
        }
        if !i_can_win {
            return false;
        }
    }

    true
}


fn can_opp_win_in(n_turns: usize, player: u8, the_board: &board::Board) -> bool {

    if n_turns == 0 {
        return false;
    }

    let mut she_can_win;
    let mut the_board_2: board::Board;
    let mut the_board_3: board::Board;
    
    for col_opp_next in 1..8 {
        the_board_2 = the_board.copy();
        board::add(&mut the_board_2, (3-player).into(), col_opp_next);
        if board::just_won(&the_board_2, 3-player, col_opp_next) {
            return true;
        }
        she_can_win = true;
        for col_next in 1..8 {
            the_board_3 = the_board_2.copy();
            board::add(&mut the_board_3, player.into(), col_next);
            if !can_opp_win_in(n_turns-1, player, &the_board_3) {
                she_can_win = false;
                break;
            }
        }
        if she_can_win {
            return true;
        }
    }

    false
}


fn estimate_score(col: usize, player: u8, the_board: &board::Board, strength: usize) -> usize {
    
    // parameters;
    let const_empty: usize = 1;
    let const_one: usize = 2;
    let const_two: usize = 4;
    let const_one_adv: usize = 4;
    let const_two_adv: usize = 8;

    // if the move is not possible, return 0
    if board::column_full(the_board, col) {
        return 0;
    } 

    // if the move makes me win, return the highest score
    
    // copy the board
    let mut test_board = the_board.copy();

    // try the move on the new board
    let line = board::add(&mut test_board, player.into(), col);

    if board::just_won(&test_board, player, col) {
        return usize::MAX;
    }
    
    for n_turns in 1..=strength {
        // If the move gives my opponent an opportunity to win, return n_turns
        if can_opp_win_in(n_turns, player, &test_board) {
            return n_turns;
        }
        // If I can win in n steps, return the maximum minus n_turns
        if can_i_win_in(n_turns, player, &test_board, col) {
            return usize::MAX - n_turns;
        }
    }
    
    // otherwise, the score is at least strength+1
    let mut score: usize = strength+1;
    
    let mut n_good_tokens: u8;
    let mut current_token: u8;
    let mut i_: u8;
    let mut j_: u8;
    
    // score for each possible horizontal line
    // i: index of the last token relative to the current position
    for i in 0..4 {
        n_good_tokens = 1;
        if (col > 3-i) && (col < 8-i) {
            for j in 0..4 {
                current_token = board::read_value(the_board, col+i-j, line);
                if current_token == player {
                    n_good_tokens += 1;
                } else if current_token == 3-player {
                    n_good_tokens = 0;
                    break;
                }
            }
            match n_good_tokens {
                1 => score += const_empty,
                2 => score += const_one,
                3 => score += const_two,
                _ => ()
            };
        }
    }
    
    // score for each horizontal line blocked
    // i: index of the last token relative to the current position
    for i in 0..4 {
        n_good_tokens = 1;
        if (col > 3-i) && (col < 8-i) {
            for j in 0..4 {
                current_token = board::read_value(the_board, col+i-j, line);
                if current_token == 3-player {
                    n_good_tokens += 1;
                } else if current_token == player {
                    n_good_tokens = 0;
                    break;
                }
            }
            match n_good_tokens {
                2 => score += const_one_adv,
                3 => score += const_two_adv,
                _ => ()
            };
        }
    }
    
    // score for each possible vertical line
    // i: index of the upper token relative to the current position
    for i in 1..4 {
        n_good_tokens = 1;
        if (line > 3-i) && (line <= 7-i) {
            for j in 0..4 {
                current_token = board::read_value(the_board, col, line+i-j);
                if current_token == player {
                    n_good_tokens += 1;
                } else if current_token == 3-player {
                    n_good_tokens = 0;
                    break;
                }
            }
            match n_good_tokens {
                1 => score += const_empty,
                2 => score += const_one,
                3 => score += const_two,
                _ => ()
            };
        }
    }
    
    // score for each possible vertical line blocked
    // i: index of the upper token relative to the current position
    for i in 1..4 {
        n_good_tokens = 1;
        if (line > 3-i) && (line <= 7-i) {
            for j in 0..4 {
                current_token = board::read_value(the_board, col, line+i-j);
                if current_token == 3-player {
                    n_good_tokens += 1;
                } else if current_token == player {
                    n_good_tokens = 0;
                    break;
                }
            }
            match n_good_tokens {
                2 => score += const_one_adv,
                3 => score += const_two_adv,
                _ => ()
            };
        }
    }
    
    // score for each possible diagonal 1
    // i: index of the upper right token relative to the current position
    for i in 1..4 {
        i_ = u8::try_from(i).unwrap();
        n_good_tokens = 1;
        if (line > 3-i_) && (line <= 7-i_) && (col > 3-i) && (col < 8-i) {
            for j in 0..4 {
                j_ = u8::try_from(j).unwrap();
                current_token = board::read_value(the_board, col+i-j, line+i_-j_);
                if current_token == player {
                    n_good_tokens += 1;
                } else if current_token == 3-player {
                    n_good_tokens = 0;
                    break;
                }
            }
            match n_good_tokens {
                1 => score += const_empty,
                2 => score += const_one,
                3 => score += const_two,
                _ => ()
            };
        }
    }
    
    // score for each possible diagonal blocked 1
    // i: index of the upper right token relative to the current position
    for i in 1..4 {
        i_ = u8::try_from(i).unwrap();
        n_good_tokens = 1;
        if (line > 3-i_) && (line <= 7-i_) && (col > 3-i) && (col < 8-i) {
            for j in 0..4 {
                j_ = u8::try_from(j).unwrap();
                current_token = board::read_value(the_board, col+i-j, line+i_-j_);
                if current_token == 3-player {
                    n_good_tokens += 1;
                } else if current_token == player {
                    n_good_tokens = 0;
                    break;
                }
            }
            match n_good_tokens {
                2 => score += const_one_adv,
                3 => score += const_two_adv,
                _ => ()
            };
        }
    }
    
    // score for each possible diagonal 2
    // i: index of the lower right token relative to the current position
    for i in 0..3 {
        n_good_tokens = 1;
        i_ = u8::try_from(i).unwrap();
        if (line > i_) && (line < 5+i_) && (col > 3-i) && (col < 8-i) {
            for j in 0..4 {
                j_ = u8::try_from(j).unwrap();
                current_token = board::read_value(the_board, col+i-j, line-i_+j_);
                if current_token == player {
                    n_good_tokens += 1;
                } else if current_token == 3-player {
                    n_good_tokens = 0;
                    break;
                }
            }
            match n_good_tokens {
                1 => score += const_empty,
                2 => score += const_one,
                3 => score += const_two,
                _ => ()
            };
        }
    }
    
    // score for each possible diagonal blocked 2
    // i: index of the lower right token relative to the current position
    for i in 0..3 {
        n_good_tokens = 1;
        i_ = u8::try_from(i).unwrap();
        if (line > i_) && (line < 5+i_) && (col > 3-i) && (col < 8-i) {
            for j in 0..4 {
                j_ = u8::try_from(j).unwrap();
                current_token = board::read_value(the_board, col+i-j, line-i_+j_);
                if current_token == 3-player {
                    n_good_tokens += 1;
                } else if current_token == player {
                    n_good_tokens = 0;
                    break;
                }
            }
            match n_good_tokens {
                2 => score += const_one_adv,
                3 => score += const_two_adv,
                _ => ()
            };
        }
    }

    if score < usize::MAX - strength {
        score
    } else {
        usize::MAX - strength - 1
    }
}

pub fn play(board: &board::Board, player: u8, rng: &mut ThreadRng, strength: usize) -> usize {
    let mut best_cols = Vec::<usize>::new();
    let mut best_score: usize = 0;
    let mut score: usize;

    // look for the moves with a best score
    for col in 1..8 {
        score = estimate_score(col, player, board, strength);
        if score == best_score {
            best_cols.push(col);
        }
        if score > best_score {
            best_score = score;
            best_cols = vec![col];
        }
    }
    
    // choose a move randomly among these
    best_cols[rng.gen_range(0..best_cols.len())]

}
