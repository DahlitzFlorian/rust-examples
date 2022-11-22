use std::io;
use std::io::Write;

fn check_win(board: &mut [[char; 6]; 4], player: char) -> bool {
    // row
    for row in 0..board.len() {
        for column in 0..board[0].len() - 2 {
            if board[row][column] == player
                && board[row][column + 1] == player
                && board[row][column + 2] == player
            {
                return true;
            }
        }
    }

    // column
    for row in 0..board.len() - 2 {
        for column in 0..board[0].len() {
            if board[row][column] == player
                && board[row + 1][column] == player
                && board[row + 2][column] == player
            {
                return true;
            }
        }
    }

    // forward diagonal
    for row in 0..board.len() - 2 {
        for column in 0..board[0].len() - 2 {
            if board[row][column] == player
                && board[row + 1][column + 1] == player
                && board[row + 2][column + 2] == player
            {
                return true;
            }
        }
    }

    // backward diagonal
    for row in (2..board.len()).rev() {
        for column in 0..board[0].len() - 2 {
            if board[row][column] == player
                && board[row - 1][column + 1] == player
                && board[row - 2][column + 2] == player
            {
                return true;
            }
        }
    }

    return false;
}

fn print_board(board: &mut [[char; 6]; 4]) {
    for (row, columns) in board.iter().enumerate().rev() {
        print!("{} ", row + 1);
        for column in columns.iter() {
            print!("{} ", column);
        }
        println!();
    }
    println!("  1 2 3 4 5 6");
}

fn change_player(player: char) -> char {
    if player == 'O' {
        'X'
    } else {
        'O'
    }
}

fn request_column(player: char, num_columns: u8) -> u8 {
    loop {
        print!("Player {} should enter a column: ", player);
        io::stdout().flush().unwrap();
        let mut column = String::new();

        io::stdin()
            .read_line(&mut column)
            .expect("Failed to read line.");

        let column: u8 = match column.trim().parse() {
            Ok(col) => {
                // TODO: Check for valid move
                if (1..=num_columns).contains(&col) {
                    col
                } else {
                    continue;
                }
            }
            Err(_) => continue,
        };

        return column;
    }
}

fn place_move(board: &mut [[char; 6]; 4], column: usize, player: &char) {
    for row in 0..board.len() {
        if board[row][column - 1] == ' ' {
            board[row][column - 1] = *player;
            break;
        }
    }
}

fn main() {
    println!("This is a Connect-3 in Rust!");
    let mut board: [[char; 6]; 4] = [[' '; 6]; 4];
    let mut active_player: char = 'O';
    print_board(&mut board);

    while check_win(&mut board, active_player) != true {
        active_player = change_player(active_player);
        let next_column = request_column(active_player, board[0].len() as u8);
        place_move(&mut board, next_column as usize, &active_player);
        print_board(&mut board);
    }
    println!("Player {} won!", active_player);
}
