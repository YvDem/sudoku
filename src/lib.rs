use std::{fmt, io};
use trees::{Init, Node}; // trees is a self-made librarie with limited functionalities to create / add to tree-shaped data structures

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

pub trait Wcomb {
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
                .fold(String::new(), |acc, arg| acc + arg.as_str()),
        )
    }
}

impl fmt::Debug for Board {
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
                if pl_pos.contains_elems(&comb) && comb.len() == 3 {
                    return Some(player);
                }
            }
        }

        if self.is_full() {
            return Some(Player::Empty);
        }

        None
    }

    pub fn show_wcomb(&self, player: Player) {
        let pl_pos: Vec<i32> = self.positions(player);

        if pl_pos.len() < 3 {
            return;
        }

        for pos in pl_pos.iter() {
            for comb in pos.wcomb(9) {
                println!("wcomb: {:?}", comb);
            }
        }
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

pub fn prompt(message: &str) -> String {
    let mut answer = String::new();

    println!("{}", message);

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    answer
}

pub fn create_all_possibilites_r(node: &mut Node<(Game, i32, i32)>, depth: i32) {
    if node.value().0.closed() || depth == 0 {
        return;
    }

    for pos in node.value().0.bcontent().empty_positions() {
        let mut sboard = node.value().0.clone();
        sboard.update_board(pos);

        let mut snode = Node::init((sboard, 0, pos as i32));
        create_all_possibilites_r(&mut snode, depth - 1);
        node.add_snode(snode);
    }
}

pub fn minimax(
    node: &mut Node<(Game, i32, i32)>,
    depth: i32,
    player: Player,
    maxplayer: bool,
) -> i32 {
    if node.value().0.closed() || depth == 0 {
        match node.value().0.winner() {
            Player::Empty => return 0,
            pl if pl == player => node.value.1 = 1,
            _ => node.value.1 = -1,
        };
        return node.value().1;
    }

    if maxplayer {
        let mut val = -9000;
        for snode in node.mut_snodes() {
            let sval = minimax(snode, depth - 1, player, false);
            val = if sval > val { sval } else { val };
        }
        node.value.1 = val;
        return node.value().1;
    } else {
        let mut val = 9000;
        for snode in node.mut_snodes() {
            let sval = minimax(snode, depth - 1, player, true);
            val = if sval > val { val } else { sval };
        }
        node.value.1 = val;
        return node.value().1;
    }
}

trait GetNodeScore {
    fn snodes_boards(&self) -> Vec<&Board>;
    fn snodes_scores(&self) -> Vec<i32>;
}

impl GetNodeScore for Node<(Game, i32)> {
    fn snodes_boards(&self) -> Vec<&Board> {
        self.snodes_values()
            .iter()
            .map(|v| v.0.bcontent())
            .fold(Vec::new(), |mut acc, elem| {
                acc.push(elem);
                acc
            })
    }

    fn snodes_scores(&self) -> Vec<i32> {
        self.snodes_values()
            .iter()
            .map(|v| v.1)
            .fold(Vec::new(), |mut acc, elem| {
                acc.push(elem);
                acc
            })
    }
}

pub fn show_possibilities(node: &Node<(Game, i32)>) {
    println!(
        "Current Node:\n{}\n Value: {}",
        node.value().0.bcontent(),
        node.value().1,
    );

    println!(
        "Sub-nodes: {}, Values: {:?}",
        node.size(0),
        node.snodes_scores()
    );

    let cmd: u32 = loop {
        let mut input = String::new();

        println!("1..n -> choisir une nth node\n 0 -> affiche toutes les snodes");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break input;
    };

    match cmd {
        0 => {
            println!("{:?}", node.snodes_boards());
            return show_possibilities(node);
        }
        cmd if cmd <= node.size(0) as u32 => show_possibilities(node.snode_at((cmd - 1) as usize)),
        _ => return,
    }
}
