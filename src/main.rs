#![allow(unused)]
use std::io;
fn main() {
    let position = ask_position();
    println!("La position choisie est : {}", position);

    let my_board = Board {current_player: 0, is_finished: false, board_content: [0; 9]};
    my_board.show_content()
}

struct Board {
    current_player: i8,
    is_finished: bool,
    board_content: [i16; 9],
}

impl Board {
    fn show_content(&self) {
        let mut i = 0;
        println!("{}", self.board_content.iter()
            .map(|n| { i += 1; 
                if i % 3 != 0 { format!(" {} |", n)} 
                else { format!(" {} \n", n)}
            })
            .fold(String::new(), |acc, arg| acc + arg.as_str()));
    }
}


fn ask_position() -> u16 {
    let board_position: u16 = loop {
        let mut input = String::new();

        println!("Donnez la position à laquelle vous voulez jouer!");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let int_position: u16 = match input.trim().parse() {
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

        break int_position;
    };

    board_position
}
