use sudoku::Game;
use sudoku::Player;

fn main() {
    let mut board = Game::default();

    loop {
        let current_player = board.current_player.symbol();
        println!("C'est au tour de {} de jouer!", current_player);

        println!("{}", board.bcontent());

        let best_move = "Ne pas jouer";
        println!("Notre IA vous conseille de jouer: {}", best_move);

        let player_position = board.ask_position();
        board.update_board_position(player_position);
        board.eval_end();

        if board.closed() {
            println!("{}", board.bcontent());

            match board.winner() {
                Player::Empty => println!("Match nul !"),
                Player::O => println!("O à gagné !"),
                Player::X => println!("X à gagné !"),
            }
            break;
        }
        board.change_current_player();
    }
}
