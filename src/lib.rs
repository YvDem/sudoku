use std::io;

#[derive(Debug, Copy, Clone)]
pub enum Player {
    X,
    O,
    Empty,
}

trait ContainsArr<T> {
    fn contains_arr(&self, arr: &[T]) -> bool;
}

impl<T: PartialEq> ContainsArr<T> for Vec<T> {
    fn contains_arr(&self, arr: &[T]) -> bool {
        let arr_size = arr.len();

        let mut count = 0;
        for elem in arr.iter() {
            if self.contains(elem) {
                count += 1;
            };
        }

        arr_size == count
    }
}

pub struct Board {
    // Note: here 0 is empty, 1 is X and 2 is O
    pub current_player: Player,
    pub board_content: [usize; 9],
    pub winner: Player,
}

impl Default for Board {
    fn default() -> Board {
        Board {
            current_player: Player::X,
            board_content: [0; 9],
            winner: Player::Empty,
        }
    }
}

impl Board {
    pub fn show_board_content(&self) {
        let mut s = " ";
        let rows = ["a", "b", "c"];

        println!(
            "  1 | 2 | 3 \n{}",
            self.board_content
                .iter()
                .enumerate()
                .map(|(i, n)| {
                    s = self.player_symbol(n);
                    match i % 3 {
                        0 => format!("{} {} |", rows[i / 3], s),
                        2 => format!(" {} \n", s),
                        1 => format!(" {} |", s),
                        _ => panic!("show_board_content: seems like x % 3 > 2"),
                    }
                })
                .fold(String::new(), |acc, arg| acc + arg.as_str())
        );
    }

    pub fn get_winner(&self) -> &Player {
        &self.winner
    }

    pub fn evaluate_end(&mut self) -> bool {
        // On ne regarde que les placements du joueur actuel (celui qui vient de jouer)

        if self.is_board_full() {
            return true;
        }

        let player_positions: Vec<(i32, i32)> = self.player_positions();

        if player_positions.len() < 3 {
            return false;
        }

        for element in player_positions.iter() {
            // on crée une combinaison gagnante verticale avec notre element
            let vert_elem = [(0, element.1), (1, element.1), (2, element.1)];
            if player_positions.contains_arr(&vert_elem) {
                self.winner = self.current_player;
                return true;
            }

            let hori_elem = [(element.0, 0), (element.0, 1), (element.0, 2)];
            if player_positions.contains_arr(&hori_elem) {
                self.winner = self.current_player;
                return true;
            }

            let righ_diag = [(0, 0), (1, 1), (2, 2)];
            if player_positions.contains_arr(&righ_diag) {
                self.winner = self.current_player;
                return true;
            }

            let left_diag = [(2, 0), (1, 1), (0, 2)];
            if player_positions.contains_arr(&left_diag) {
                self.winner = self.current_player;
                return true;
            }
        }

        false
    }

    fn is_board_full(&self) -> bool {
        for element in self.board_content.iter() {
            if element == &0 {
                return false;
            }
        }

        true
    }

    pub fn player_positions(&self) -> Vec<(i32, i32)> {
        let mut positions: Vec<(i32, i32)> = vec![];
        let current_player = self.current_player_value();

        let mut i = 0;
        for element in self.board_content.iter() {
            if element != &current_player {
                i += 1;
                continue;
            } else {
                let x = i % 3;
                let y = (i / 3) % 3;
                positions.push((x, y));

                i += 1;
            };
        }

        positions
    }

    pub fn player_symbol(&self, value: &usize) -> &str {
        match value {
            1 => "X",
            2 => "O",
            _ => " ",
        }
    }

    fn is_position_empty(&self, pos: usize) -> bool {
        let board_position = self.board_content[pos];
        matches!(board_position, 0)
    }

    pub fn update_board_position(&mut self, pos: usize) {
        self.board_content[pos] = self.current_player_value();
    }

    pub fn change_current_player(&mut self) {
        match self.current_player {
            Player::X => self.current_player = Player::O,
            Player::O => self.current_player = Player::X,
            _ => panic!("How did we get here ?"),
        }
    }

    pub fn current_player_value(&self) -> usize {
        match self.current_player {
            Player::X => 1,
            Player::O => 2,
            Player::Empty => 0,
        }
    }

    pub fn current_player_symbol(&self) -> &str {
        match self.current_player {
            Player::X => "X",
            Player::O => "O",
            _ => panic!("How did we get here ?"),
        }
    }

    pub fn ask_position(&self) -> usize {
        let board_position: u32 = loop {
            let mut input = String::new();

            println!("Donnez la position à laquelle vous voulez jouer! (ex: a1, b2 ..) :");

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let int_position = match (
                input.trim().chars().nth(0).unwrap(),
                input.trim().chars().nth(1),
            ) {
                (.., None) => {
                    println!("Entrez une position valide!");
                    continue;
                }

                ('a', x @ Some('1' | '2' | '3')) => x.unwrap().to_digit(10).unwrap() + 0 - 1,
                ('b', x @ Some('1' | '2' | '3')) => x.unwrap().to_digit(10).unwrap() + 3 - 1,
                ('c', x @ Some('1' | '2' | '3')) => x.unwrap().to_digit(10).unwrap() + 6 - 1,

                _ => {
                    println!("Entrez une position valide!");
                    continue;
                }
            };

            if self.is_position_empty(int_position as usize) {
                break int_position;
            } else {
                println!("Entrez une position valide!");
                continue;
            }
        };

        board_position as usize
    }
}
