use sudoku::{create_all_possibilites_r, minimax, prompt};
use sudoku::{Game, Player};
use trees::{Init, Node};

fn main() {
    loop {
        match prompt("-> `p` to play against another player\n-> `ai` to play against an ai\n-> `q` to quit: ").trim()
        {
            "p" => play_1v1(),
            "ai" => play_ia(),
            "q" => break,
            _ => continue,
        }
    }
}

fn play_1v1() {
    let mut board = Game::default();
    loop {
        let current_player = board.current_player.symbol();
        println!("It's time for {} to play!", current_player);

        println!("{}", board.bcontent());

        let player_position = board.ask_position();
        board.update_board(player_position);

        if board.closed() {
            println!("{}", board.bcontent());

            match board.winner() {
                Player::Empty => println!("Draw !"),
                Player::O => println!("O won !"),
                Player::X => println!("X won !"),
            }
            break;
        }
    }
}

fn play_ia() {
    let mut board = Game::default();

    let player = loop {
        let answer = match prompt("Do you want to start ? (y/n)").trim() {
            "y" => Player::X,
            "n" => Player::O,
            _ => {
                println!("Enter a valid option!");
                continue;
            }
        };

        break answer;
    };

    let level: u32 = loop {
        let level = match prompt("-> Choose difficulty: (1..4)")
            .trim()
            .parse()
        {
            Ok(num @ 1..=5) => num * 3,
            _ => {
                println!("Enter a valid number!");
                continue;
            }
        };

        break level;
    };

    let ai = player.swap();

    loop {
        let current_player = board.current_player.symbol();
        println!("It's time for {} to play!", current_player);
        println!("{}", board.bcontent());

        if board.c_player() == ai {
            let mut b_tree = Node::init((board.clone(), 0, 0)); // game, value of node, move played
            create_all_possibilites_r(&mut b_tree, level as i32);
            let b_move_val = minimax(&mut b_tree, level as i32, ai, true);
            for snode in b_tree.snodes() {
                if snode.value().1 == b_move_val {
                    board = snode.value.0.clone()  // yes, here we are replacing the current board with a board stored in the tree with the ia chosen move.
                }
            }
        } else {
            let player_position = board.ask_position();
            board.update_board(player_position);
        }

        if board.closed() {
            println!("{}", board.bcontent());

            match board.winner() {
                Player::Empty => println!("Draw !"),
                Player::O => println!("O won !"),
                Player::X => println!("X won !"),
            }
            break;
        }
    }
}
