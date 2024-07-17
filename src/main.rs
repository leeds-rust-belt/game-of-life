use rand::Rng;
use std::io;
use std::io::prelude::*;

fn main() {
    let size: usize = 40;
    let mut game_board = vec![vec![0; size as usize]; size as usize];

    // randomly seed  board
    seed_game_board(&mut game_board, size);

    // pretty print the game board with initial seed
    print_game_board(&game_board, size);

    // Generations
    for i in 0..20000 {
        print!("\x1B[2J\x1B[1;1H"); // hack to clear the screen with setting cursor to top left - Taken from https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
        redraw_game_board(&mut game_board, size);
        print_game_board(&game_board, size);
        println!("{}",i);
        //pause();
    }
}

// function to redraw game 
fn redraw_game_board(game_board: &mut Vec<Vec<i32>>, size: usize) {
    // Loop through every cell
    for i in 0..size { 
        for j in 0..size {
            // set up a count of live cell sin nearest naieghbours
            let mut count = 0;
            // Loop through nearest neighbours
            // REMEMBER that the range is exclusive on the end number so -1..2 is -1,0,1
            for dx in -1..2 {
                for dy in -1..2 {
                    // calculate the row and col to check
                    let row = (i as i32 + dx) as usize; // cast as usize so we can uas as an index
                    let col = (j as i32 + dy) as usize;
                    // check for array boundaries
                    if row < size && col < size {
                        //Add the avlue at the cell to the live count
                        count += game_board[row][col];
                    }
                }
            }
            // remove count value from cell to check
            count -= game_board[i][j];
            // Life rules - TODO Cehck these are the correct rules
            if game_board[i][j] == 1 && count < 2 {
                game_board[i][j] = 0;
            } else if game_board[i][j] == 1 && count > 3 {
                game_board[i][j] = 0;
            } else if game_board[i][j] == 0 && count == 3 {
                game_board[i][j] = 1;
            }
        }
    }
}

// function to seed game board
fn seed_game_board(game_board: &mut Vec<Vec<i32>>, size: usize) {
    for i in 0..size {
        for j in 0..size {
            let random = rand::random::<i32>() % 10;
            // should get about 10% cells live
            if random < 1 {
                game_board[i][j] = 1;
            } else {
                game_board[i][j] = 0;
            }
        }
    }
}

// function to print game board
fn print_game_board(game_board: &Vec<Vec<i32>>, size: usize) {
    for i in 0..size {
        for j in 0..size {
            if game_board[i][j] == 0 {
                print!(" ");
            } else {
                print!("*");
            }
        }
        println!();
    }
}

// Pause function stolen from https://users.rust-lang.org/t/rusts-equivalent-of-cs-system-pause/4494/2
fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
} 