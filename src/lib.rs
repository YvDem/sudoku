use std::{fmt, io};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    X,
    O,
    Empty,
}

impl Player {
    pub fn symbol(&self) -> &str {
        match &self {
            Player::X => "X",
            Player::O => "O",
            Player::Empty => " ",
        }
    }
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

#[derive(Clone)]
pub struct Board {
    pub content: [Player; 9],
}

#[derive(Clone)]
pub struct Game {
    pub current_player: Player,
    pub board_content: Board,
    pub winner: Player,
}

impl Default for Board {
    fn default() -> Board {
        Board {
            content: [Player::Empty; 9],
        }
    }
}

impl Default for Game {
    fn default() -> Game {
        Game {
            current_player: Player::X,
            board_content: Board::default(),
            winner: Player::Empty,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows = ["a", "b", "c"];
        write!(
            f,
            "  1 | 2 | 3 \n{}",
            self.content
                .iter()
                .enumerate()
                .map(|(i, n)| {
                    match i % 3 {
                        0 => format!("{} {} |", rows[i / 3], n.symbol()),
                        2 => format!(" {} \n", n.symbol()),
                        1 => format!(" {} |", n.symbol()),
                        _ => panic!("show_board_content: seems like x % 3 > 2"),
                    }
                })
                .fold(String::new(), |acc, arg| acc + arg.as_str())
        )
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(current player: {} \n Content: {})",
            self.current_player, self.board_content
        )
    }
}

impl Game {
    // Todo: impl tous les elements liées à Board dans le impl de board
    pub fn board_content(&self) -> &Board {
        &self.board_content
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
        !self.board_content.content.contains(&Player::Empty)
    }

    pub fn player_positions(&self) -> Vec<(i32, i32)> {
        self.board_content
            .content
            .iter()
            .enumerate()
            .map(|(i, p)| {
                if p == &self.current_player {
                    ((i % 3) as i32, ((i / 3) % 3) as i32)
                } else {
                    (4, 4)
                }
            })
            .filter(|e| e != &(4 as i32, 4 as i32))
            .collect()
    }

    pub fn empty_positions(&self) -> Vec<usize> {
        self.board_content
            .content
            .iter()
            .enumerate()
            .map(|(i, p)| if p == &Player::Empty { i } else { 10 })
            .filter(|e| e < &10)
            .collect()
    }

    fn is_position_empty(&self, pos: usize) -> bool {
        let board_position = self.board_content.content[pos];
        board_position == Player::Empty
    }

    pub fn update_board_position(&mut self, pos: usize) {
        self.board_content.content[pos] = self.current_player;
    }

    pub fn change_current_player(&mut self) {
        match self.current_player {
            Player::X => self.current_player = Player::O,
            Player::O => self.current_player = Player::X,
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
