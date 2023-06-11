#![allow(unused)]
use std::io;
fn main() {
    let mut board = Board {current_player: Player::X, is_finished: false, board_content: [0; 9], winner: Player::NoPlayer};
    while !board.is_game_finished() {

        let current_player = board.current_player_symbol();
        println!("C'est au tour de {} de jouer!", current_player);
        board.show_board_content();

        let player_position = board.ask_position();
        board.update_board_position(player_position);
        board.evaluate_end();




        board.change_current_player();

    }
}

enum Player {
    X,
    O,
    NoPlayer
}

struct Board {
    // Note: here 0 is empty, 1 is X and 2 is O
    current_player: Player,
    is_finished: bool,
    board_content: [usize; 9],
    winner: Player,
}

impl Board {
    fn show_board_content(&self) {
        let mut i = 0;
        let mut s = " ";

        println!("{}", self.board_content
            .iter()
            .map(|n| { i += 1; s = self.player_symbol(n);
                if i % 3 != 0 { format!(" {} |", s)} 
                else { format!(" {} \n", s)}
            })
            .fold(String::new(), |acc, arg| acc + arg.as_str()));
    }

    fn evaluate_end(&self) -> bool {
        // On ne regarde que les placements du joueur actuel (celui qui vient de jouer)
        false
    }

    fn player_symbol(&self, value: &usize) ->  &str {
        match value {
            1 => "X",
            2 => "O",
            _ => " "
        }
    }

    fn is_game_finished(&self) -> bool {
        self.is_finished
    }

    fn is_position_empty(&self, pos: usize) -> bool {
        let board_position = self.board_content[pos];
        matches!(board_position, 0)
    }

    fn update_board_position(&mut self, pos: usize) {
            self.board_content[pos] = self.current_player_value();
    }

    fn change_current_player(&mut self) {
        match self.current_player {
            Player::X => { self.current_player = Player::O },
            Player::O => { self.current_player = Player::X },
            Player::NoPlayer => panic!("Il n'y a pas de joueur!")
        }
        
    }

    fn current_player_value(&self) -> usize {
        match self.current_player {
            Player::X => 1,
            Player::O => 2,
            Player::NoPlayer => 0
        }
    }

    fn current_player_symbol(&self) -> &str {
        match self.current_player {
            Player::X => "X",
            Player::O => "O",
            Player::NoPlayer => panic!("Aucun joueur selectionné!")
        }
    }

    fn ask_position(&self) -> usize {
        let board_position: usize = loop {
            let mut input = String::new();
    
            println!("Donnez la position à laquelle vous voulez jouer!");
    
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
    
            let int_position: usize = match input.trim().parse() {
                Ok(num @ 0..=8) => num,
                Ok(_) => {
                    println!("Entrez une valeure valide (entre 0 et 8)");
                    continue;
                }
                Err(_) => {
                    println!("Entrez une valeure valide (entre 0 et 8)");
                    continue;
                }
            };
    
            if self.is_position_empty(int_position) { break int_position} 
            else { 
                println!("Cette position est déjà prise!");
                continue; 
            };
        };
    
        board_position
    }
    
}