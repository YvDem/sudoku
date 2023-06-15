use std::io;

pub enum Player {
    X,
    O,
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
}

#[allow(dead_code)]
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

    pub fn evaluate_end(&self) -> bool {
        // On ne regarde que les placements du joueur actuel (celui qui vient de jouer)
        let player_positions: Vec<(i32, i32)> = self.player_positions();

        if player_positions.len() < 3 {
            return false;
        }

        for element in player_positions.iter() {
            // on crée une combinaison gagnante verticale avec notre element
            let vert_elem = [(0, element.1), (1, element.1), (2, element.1)];
            if player_positions.contains_arr(&vert_elem) {
                return true;
            }

            let hori_elem = [(element.0, 0), (element.0, 1), (element.0, 2)];
            if player_positions.contains_arr(&hori_elem) {
                return true;
            }

            let righ_diag = [(0, 0), (1, 1), (2, 2)];
            if player_positions.contains_arr(&righ_diag) {
                return true;
            }

            let left_diag = [(2, 0), (1, 1), (0, 2)];
            if player_positions.contains_arr(&left_diag) {
                return true;
            }
        }

        false
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
        }
    }

    pub fn current_player_value(&self) -> usize {
        match self.current_player {
            Player::X => 1,
            Player::O => 2,
        }
    }

    pub fn current_player_symbol(&self) -> &str {
        match self.current_player {
            Player::X => "X",
            Player::O => "O",
        }
    }

    pub fn ask_position(&self) -> usize {
        let board_position: usize = loop {
            let mut input = String::new();

            println!("Donnez la position à laquelle vous voulez jouer! (ex: a1, b2 ..) :");

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let int_position = match as_tuple(input.trim()) {
                Ok(tuple) => tuple[1].into(),
                Err(_) => {
                    println!("Entrez une position valide!");
                    continue;
                }
            };

            break int_position;
        };

        board_position
    }
}

fn as_tuple(input: &str) -> Result<&[u8], &str> {
    let bytes = input.as_bytes();

    let result = match bytes {
        &[b'a' | b'b' | b'c', i @ 1..=3] => std::result::Result::Ok(bytes),
        _ => std::result::Result::Err("Not correct position"),
    };
    result
}
