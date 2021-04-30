use std::io::stdin;
mod board;
mod ai;

static DEFAULT_STRENGTH: usize = 5;

fn get_input() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {

    let mut the_board: board::Board = board::new_board();
    let mut col_played: usize;
    let is_ai: u8;
    let mut rng = rand::thread_rng();
    let mut strength: usize = DEFAULT_STRENGTH;
    
    println!("AI? (None, 1, 2, or 12)");
    match get_input().trim().parse::<usize>() {
        Ok(n) => {
                   match n {
                    1  => is_ai = 1,
                    2  => is_ai = 2,
                    12 => is_ai = 3,
                    _  => is_ai = 0
                   } 
                 },
        Err(..) => is_ai = 0
    };
    if is_ai > 0 {
        println!("AI strength?");
        match get_input().trim().parse::<usize>() {
            Ok(n) => strength = n,
            Err(..) => {
                    println!("Could not parse, revert to default strength of {}.", DEFAULT_STRENGTH);
                }
        };
    }

    board::show_board(&the_board);

    for _n_turn in 1..22 {
         
        for player in (std::ops::Range::<u8>{start:1, end:3}) {

            println!("Player {}:", player);
            if is_ai & (1 << (player-1)) > 0 {
                col_played = ai::play(&the_board, player, &mut rng, strength);
            } else {
                loop {
                    match get_input().trim().parse::<usize>() {
                        Ok(c) => {
                                    col_played = c;
                                    if (c < 1) || (c > 7) {
                                        println!("Not a valid column");
                                    } else if board::column_full(&the_board, c) {
                                        println!("Column full!");
                                    } else {
                                        break;
                                    }
                                 },
                        Err(..) => println!("Wrong value")
                    };
                }
            }
            board::add(&mut the_board, player.into(), col_played);

            board::show_board(&the_board);
            if board::just_won(&the_board, player, col_played) {
                println!("Player {} wins! Congratulations!", player);
                return;
            }
        }
        
    }
    println!("No winner...");
}
