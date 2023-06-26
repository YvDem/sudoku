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

    pub fn swap(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
            Player::Empty => Player::Empty,
        }
    }
}

trait Contains<T> {
    fn contains_elems(&self, arr: &[T]) -> bool;
}

impl<T: PartialEq> Contains<T> for Vec<T> {
    // ne doit etre utilisé que sur des types ne containant pas d'éléments en double
    fn contains_elems(&self, arr: &[T]) -> bool {
        let arr_size = arr.len();
        let count: Vec<&T> = arr.iter().filter(|e| self.contains(e)).collect();

        count.len() == arr_size
    }
}

trait Wcomb {
    fn wcomb(self, size: i32) -> Vec<Vec<i32>>;
}

impl Wcomb for i32 {
    fn wcomb(self, len: i32) -> Vec<Vec<i32>> {
        let size = (len as f32).sqrt() as i32;
        let offset = self % size;
        let layer = self / size;

        let mut result = vec![
            (self - offset..(layer + 1) * size).collect::<Vec<_>>(),
            (self - layer * size..len).step_by(size as usize).collect(),
        ];

        if self % (size + 1) == 0 {
            result.push(
                (self - (layer * (size + 1))..len)
                    .step_by((size + 1) as usize)
                    .collect(),
            );
        }

        if self % (size - 1) == 0 {
            result.push(
                (self - (layer * (size - 1))..len)
                    .step_by((size - 1) as usize)
                    .filter(|e| e < &8)
                    .collect(),
            );
        }
        result
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
    pub closed: bool,
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
            closed: false,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows = ["a", "b", "c"];
        write!(
            f,
            "  1 | 2 | 3 \n",
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
                .fold(String::new(), |acc, arg| acc + arg.as_str()),
        )
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows = ["a", "b", "c"];
        write!(
            f,
            "  1 | 2 | 3 \n",
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
                .fold(String::new(), |acc, arg| acc + arg.as_str()),
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

impl Board {
    fn is_full(&self) -> bool {
        !self.content.contains(&Player::Empty)
    }

    fn positions(&self, player: Player) -> Vec<i32> {
        self.content
            .iter()
            .enumerate()
            .map(|(i, p)| if p == &player { i as i32 } else { 10 })
            .filter(|e| e != &(10 as i32))
            .collect()
    }

    pub fn eval_winner(&self, player: Player) -> Option<Player> {
        let pl_pos: Vec<i32> = self.positions(player);

        if pl_pos.len() < 3 {
            return None;
        }

        for pos in pl_pos.iter() {
            for comb in pos.wcomb(9) {
                if pl_pos.contains_elems(&comb) {
                    return Some(player);
                }
            }
        }

        if self.is_full() {
            return Some(Player::Empty);
        }

        None
    }

    pub fn empty_positions(&self) -> Vec<usize> {
        self.content
            .iter()
            .enumerate()
            .map(|(i, p)| if p == &Player::Empty { i } else { 10 })
            .filter(|e| e < &10)
            .collect()
    }
}

impl Game {
    // Todo: impl tous les elements liées à Board dans le impl de board
    pub fn bcontent(&self) -> &Board {
        &self.board_content
    }

    pub fn c_player(&self) -> Player {
        self.current_player
    }

    pub fn winner(&self) -> Player {
        self.winner
    }

    pub fn closed(&self) -> bool {
        self.closed
    }

    pub fn eval_end(&mut self) {
        match self.board_content.eval_winner(self.current_player) {
            Some(player) => {
                self.winner = player;
                self.closed = true;
            }
            None => (),
        }
    }

    fn is_position_empty(&self, pos: usize) -> bool {
        let board_position = self.board_content.content[pos];
        board_position == Player::Empty
    }

    pub fn update_board(&mut self, pos: usize) {
        self.board_content.content[pos] = self.current_player;
        self.eval_end();
        self.change_current_player();
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
