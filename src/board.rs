// Each column of the board has 6 spots which can take 3 values each. 
// We represent it by a u16, with each spot coded in two bits.
// In all this file, 
//  * player must be 1 or 2,
//  * col must be between 1 and 7 included,
//  * line must be between 1 and 6 included.

use std::convert::TryInto;

pub struct Board {
    pub columns: Vec<u16>,
}

pub fn new_board() -> Board {
    Board {
        columns: vec![0; 7],
    }
}

impl Board
{
    pub fn copy(&self) -> Board {
        let mut n_board = new_board();
        for i in 0..7 {
            n_board.columns[i] = self.columns[i];
        }
        n_board
    }
}

pub fn read_value(board: &Board, col: usize, line: u8) -> u8 {
    if (col < 1) || (col > 7) {
        panic!();
    }
    ((&board.columns[col-1] & (3 << 2*(line-1))) >> 2*(line-1)).try_into().unwrap()
}

// fn set_value(board: &mut Board, col: usize, line: u8, val: u16) {
//     board.columns[col-1] = (&board.columns[col-1] & (3 << 2*(line-1))) | (val << 2*(line-1));
// }

pub fn show_board(board: &Board) {
    for i in 1..7 {
        let mut line = String::from("│");
        for j in 1..8 {
            match read_value (&board, j, 7-i) {
                1 => line.push('●'),
                2 => line.push('○'),
                _ => line.push(' ')
            };
            line.push('│');
        }
        println!("{}", line);
    }
    println!("╰─┴─┴─┴─┴─┴─┴─╯");
}

// pub fn show_board_ascii(board: &Board) {
//     for i in 1..7 {
//         let mut line = String::from("│");
//         for j in 1..8 {
//             match read_value (&board, j, 7-i) {
//                 1 => line.push('.'),
//                 2 => line.push('o'),
//                 _ => line.push(' ')
//             };
//             line.push('|');
//         }
//         println!("{}", line);
//     }
//     println!("");
// }

// add a token to a column
pub fn add(board: &mut Board, player: u16, col: usize) -> u8 {
    let column: &mut u16 = &mut board.columns[col-1];
    if *column > 1023 {
        return 7;
    }
    if *column > 255 {
        *column += player*1024;
        return 6;
    }
    if *column > 63 {
        *column += player*256;
        return 5;
    }
    if *column > 15 {
        *column += player*64;
        return 4;
    }
    if *column > 3 {
        *column += player*16;
        return 3;
    }
    if *column > 0 {
        *column += player*4;
        return 2;
    }
    *column += player;
    return 1;
}

pub fn column_full(board: &Board, col: usize) -> bool {
    let column: &u16 = &board.columns[col-1];
    *column > 1023
}

pub fn just_won(board: &Board, player: u8, col: usize) -> bool {

    // determine the line where the latest piece is
    let line: u8;
    let column: &u16 = &board.columns[col-1];
    if *column > 1023 {
        line = 6;
    } else if *column > 255 {
        line = 5;
    } else if *column > 63 {
        line = 4;
    } else if *column > 15 {
        line = 3;
    } else if *column > 3 {
        line = 2;
    } else {
        line = 1;
    }

    // if the token does not belong to this player, return false
    if read_value(&board, col, line) != player {
        return false;
    }
    
    // check the column
    if line > 3 {
        if (read_value(&board, col, line-1) == player)
           && (read_value(&board, col, line-2) == player)
           && (read_value(&board, col, line-3) == player) {

            return true;   
        }
    }

    // check the line
    let mut n_aligned = 1;
    if (col > 1) && (read_value(&board, col-1, line) == player) {
        n_aligned += 1;
        if (col > 2) && (read_value(&board, col-2, line) == player) {
            n_aligned += 1;
            if (col > 3) && (read_value(&board, col-3, line) == player) {
                return true;
            }
        }
    }
    if (col < 7) && (read_value(&board, col+1, line) == player) {
        n_aligned += 1;
        if (col < 6) && (read_value(&board, col+2, line) == player) {
            n_aligned += 1;
            if (col < 5) && (read_value(&board, col+3, line) == player) {
                return true;
            }
        }
    }
    if n_aligned > 3 {
        return true
    }

    // check the first diagonal
    let mut n_aligned = 1;
    if (col > 1) && (line > 1) && (read_value(&board, col-1, line-1) == player) {
        n_aligned += 1;
        if (col > 2) && (line > 2) && (read_value(&board, col-2, line-2) == player) {
            n_aligned += 1;
            if (col > 3) && (line > 3) && (read_value(&board, col-3, line-3) == player) {
                return true;
            }
        }
    }
    if (col < 7) && (line < 7) && (read_value(&board, col+1, line+1) == player) {
        n_aligned += 1;
        if (col < 6) && (line < 6) && (read_value(&board, col+2, line+2) == player) {
            n_aligned += 1;
            if (col < 5) && (line < 5) && (read_value(&board, col+3, line+3) == player) {
                return true;
            }
        }
    }
    if n_aligned > 3 {
        return true
    }
    
    // check the second diagonal
    let mut n_aligned = 1;
    if (col > 1) && (line < 7) && (read_value(&board, col-1, line+1) == player) {
        n_aligned += 1;
        if (col > 2) && (line < 6) && (read_value(&board, col-2, line+2) == player) {
            n_aligned += 1;
            if (col > 3) && (line < 5) && (read_value(&board, col-3, line+3) == player) {
                return true;
            }
        }
    }
    if (col < 7) && (line > 1) && (read_value(&board, col+1, line-1) == player) {
        n_aligned += 1;
        if (col < 6) && (line > 2) && (read_value(&board, col+2, line-2) == player) {
            n_aligned += 1;
            if (col < 5) && (line > 3) && (read_value(&board, col+3, line-3) == player) {
                return true;
            }
        }
    }
    if n_aligned > 3 {
        return true
    }
    
    false
}
