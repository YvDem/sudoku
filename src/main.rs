use sudoku::Board;
use sudoku::Player;

fn main() {
    let mut board = Board {
        current_player: Player::X,
        board_content: [0; 9],
    };

    loop {
        let current_player = board.current_player_symbol();
        println!("C'est au tour de {} de jouer!", current_player);
        board.show_board_content();

        let player_position = board.ask_position();
        board.update_board_position(player_position);

        let end = board.evaluate_end();
        if end {
            let current_player = board.current_player_symbol();
            board.show_board_content();
            println!("{} a gagné!!!", current_player);
            break;
        }
        board.change_current_player();
    }
}
