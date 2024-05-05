use std::io;
use rand::seq::SliceRandom;

fn print_board(board: &Vec<char>) {
    println!("");
    println!("{}|{}|{}", board[0], board[1], board[2]);
    println!("{}|{}|{}", board[3], board[4], board[5]);
    println!("{}|{}|{}", board[6], board[7], board[8]);
    println!("");
}

fn input_move(board: &mut Vec<char>) {

    println!("Where do you want to move? Input a number between 1 and 9.");

    let mut move_string: String = String::new();
    
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
            avail_moves.push(
                u32::try_from(i).unwrap()
            );
        }
    }

    let random_index: &u32 = avail_moves.choose(&mut rand::thread_rng()).unwrap();
    board[*random_index as usize] = 'O';

}

fn check_for_winner(board: &Vec<char>) -> Option<char> {

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
    
    for comb in all_combs.iter() {
        match comb {
            ['X', 'X', 'X'] => return Some('X'),
            ['O', 'O', 'O'] => return Some('O'),
            _ => {},
        }
    }

    if board.contains(&'-') {
        return None;
    }
    
    Some('T')

}

fn main() {

    let mut board: Vec<char> = vec!['-', '-', '-', '-', '-', '-', '-', '-', '-'];
    let mut winner: Option<char>;

    println!("\nWelcome to tic-tac-toe!\nYou are player X.");

    loop {

        print_board(&board);

        input_move(&mut board);

        winner = check_for_winner(&board);
        if winner.is_some() {
            break;
        }

        random_move(&mut board);

        winner = check_for_winner(&board);
        if winner.is_some() {
            break;
        }

    }

    print_board(&board);

    match winner.unwrap() {
        'X' => println!("You win!"),
        'O' => println!("You lose!"),
        _ => println!("Tie game!"),
    }

}