use std::io;
#[allow(unused_imports)]
use sudoku::{Board, Game, Player};
use trees::{Init, Node};

fn main() {
    let mut b_tree = Node::init(Game {
        current_player: Player::X,
        board_content: Board {
            content: [
                Player::X,
                Player::X,
                Player::Empty,
                Player::O,
                Player::O,
                Player::Empty,
                Player::Empty,
                Player::Empty,
                Player::Empty,
            ],
        },
        winner: Player::Empty,
        closed: false,
    });

    create_all_possibilites_r(&mut b_tree);
    println!("size: {}", b_tree.size(-1));
    print!("minimax: {}\n", minimax_r(&mut b_tree, 9, Player::X));

    for snode in b_tree.mut_snodes() {
        println!("minimax of snode :{}", minimax_r(snode, 9, Player::X))
    }

    show_possibilities(&b_tree);
}

fn create_all_possibilites_r(node: &mut Node<Game>) {
    if node.value().closed() {
        return;
    }

    for pos in node.value().bcontent().empty_positions() {
        let mut sboard = node.value().clone();
        sboard.update_board(pos);

        let mut snode = trees::Node::init(sboard);
        create_all_possibilites_r(&mut snode);
        node.add_snode(snode);
    }
}

fn minimax_r(node: &mut Node<Game>, depth: i32, max_player: Player) -> i32 {
    if node.value().closed() || depth == 0 {
        let value = match node.value().winner() {
            Player::Empty => 0,
            e if e == max_player => 1,
            _ => -1,
        };
        return value;
    }

    let mut value = -9000;
    if node.value().c_player() == max_player {
        for snode in node.mut_snodes() {
            let b_value = minimax_r(snode, depth - 1, max_player);
            value = if b_value > value { b_value } else { value }
        }
    } else {
        value = 9000;
        for snode in node.mut_snodes() {
            let b_value = minimax_r(snode, depth - 1, max_player);
            value = if b_value > value { value } else { b_value }
        }
    }

    value
}

trait GetNodeScore {
    fn snodes_boards(&self) -> Vec<&Board>;
}

impl GetNodeScore for Node<Game> {
    fn snodes_boards(&self) -> Vec<&Board> {
        self.snodes_values()
            .iter()
            .map(|v: &&Game| v.bcontent())
            .fold(Vec::new(), |mut acc, elem| {
                acc.push(elem);
                acc
            })
    }
}

#[allow(dead_code)]
fn show_possibilities(node: &Node<Game>) {
    println!("Current Node:\n{}", node.value().bcontent());
    println!("Sub-nodes: {}", node.size(0),);

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
