use sudoku::Board;
use sudoku::Player;
fn main() {
    let mut board = Board::default();

    loop {
        let current_player = board.current_player_symbol();
        println!("C'est au tour de {} de jouer!", current_player);

        board.show_board_content();

        let player_position = board.ask_position();
        board.update_board_position(player_position);

        let end = board.evaluate_end();
        if end {
            board.show_board_content();

            match board.get_winner() {
                Player::Empty => println!("Match nul !"),
                Player::O => println!("O à gagné !"),
                Player::X => println!("O à gagné !"),
            }
            break;
        }
        board.change_current_player();
    }
}
