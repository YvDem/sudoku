use sudoku::prompt;
use sudoku::Game;
use sudoku::Player;

fn main() {
    loop {
        match prompt("-> `p` pour jouer contre un autre joueur\n-> `ai` pour jouer contre l'ai\n-> q pour quitter: ").trim()
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
        println!("C'est au tour de {} de jouer!", current_player);

        println!("{}", board.bcontent());

        let player_position = board.ask_position();
        board.update_board(player_position);

        if board.closed() {
            println!("{}", board.bcontent());

            match board.winner() {
                Player::Empty => println!("Match nul !"),
                Player::O => println!("O à gagné !"),
                Player::X => println!("X à gagné !"),
            }
            break;
        }
    }
}

fn play_ia() {
    let mut board = Game::default();
    loop {
        let current_player = board.current_player.symbol();
        println!("C'est au tour de {} de jouer!", current_player);

        println!("{}", board.bcontent());

        let player_position = board.ask_position();
        board.update_board(player_position);

        if board.closed() {
            println!("{}", board.bcontent());

            match board.winner() {
                Player::Empty => println!("Match nul !"),
                Player::O => println!("O à gagné !"),
                Player::X => println!("X à gagné !"),
            }
            break;
        }
    }
}
