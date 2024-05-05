use rand::seq::SliceRandom;
use std::io;
use std::convert::TryFrom;

fn print_board(board: &mut Vec<char>) {
    println!("");
    println!("{}|{}|{}", board[0], board[1], board[2]);
    println!("{}|{}|{}", board[3], board[4], board[5]);
    println!("{}|{}|{}", board[6], board[7], board[8]);
    println!("");
}

fn input_move(board: &mut Vec<char>) {

    println!("Where do you want to move? Input a number between 1 and 9.");

    let mut move_string = String::new();

    io::stdin()
        .read_line(&mut move_string)
        .expect("Failed to read from stdin.");

    let move_index: u32 = match move_string.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid move! You must input an integer.");
            return;
        },
    };

    if move_index < 1 || move_index > 9 || board[move_index as usize - 1] != '-' {
        println!("Invalid move! You must select an available move.");
        return;
    }

    board[move_index as usize - 1] = 'X';

}

fn random_move(board: &mut Vec<char>) {

    let mut avail_moves: Vec<u32> = Vec::new();

    for i in 0..9 {
        if board[i] == '-' {
            let index: u32 = u32::try_from(i).unwrap();
            avail_moves.push(index);
        }
    }

    let random_index = avail_moves.choose(&mut rand::thread_rng()).unwrap();
    board[*random_index as usize] = 'O';

}

fn check_for_winner(board: &mut Vec<char>) -> char {

    let all_combs: [[char; 3]; 8] = [
        [board[0], board[1], board[2]],
        [board[3], board[4], board[5]],
        [board[6], board[7], board[8]],
        [board[0], board[3], board[6]],
        [board[1], board[4], board[7]],
        [board[2], board[5], board[8]],
        [board[0], board[4], board[8]],
        [board[2], board[4], board[6]]
    ];
    
    // for comb in all_combs.into_iter() {
    for comb in all_combs.iter() {
        // match winner {
        //     ['X', 'X', 'X'] => return 'X',
        //     ['O', 'O', 'O'] => return 'O',
        // }
        if comb[0] == 'X' && comb[1] == 'X' && comb[2] == 'X' {
            return 'X';
        } else if comb[0] == 'O' && comb[1] == 'O' && comb[2] == 'O' {
            return 'O';
        }
    }

    if !board.contains(&'-') {
        return 'T';
    }
    return 'N';

}

fn main() {

    let mut board = vec!['-', '-', '-', '-', '-', '-', '-', '-', '-'];
    let mut winner: char;

    println!("You are player X.");

    loop {

        print_board(&mut board);

        input_move(&mut board);
        winner = check_for_winner(&mut board);
        if winner != 'N' {
            break;
        }

        random_move(&mut board);
        winner = check_for_winner(&mut board);
        if winner != 'N' {
            break;
        }

    }

    print_board(&mut board);

    if winner == 'X' {
        println!("You win!");
    } else if winner == 'O' {
        println!("You lose!");
    } else {
        println!("Tie game!");
    }

}