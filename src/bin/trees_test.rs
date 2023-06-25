use std::io;
#[allow(unused_imports)]
use sudoku::{Board, Game, Player};
use trees::{Init, Node};

fn main() {
    let mut b_tree = Node::init((0, Game::default(), 0));
    create_all_possibilites_r(&mut b_tree, Player::X);
    println!("size: {}", b_tree.size(-1));
    print!("minimax: {}\n", minimax(&b_tree, Player::X));
    show_possibilities(&b_tree);
}

fn create_all_possibilites_r(node: &mut Node<(i32, Game, i32)>, player: Player) {
    if node.value().1.closed() {
        match node.value().1.winner() {
            Player::Empty => node.value.0 = 0,
            e if e == player => node.value.0 = 10 - node.value().2,
            _ => node.value.0 = 10 - node.value().2,
        }
        return;
    }

    for pos in node.value().1.bcontent().empty_positions() {
        let mut sboard = node.value().1.clone();
        sboard.update_board(pos);

        let mut snode = trees::Node::init((node.value().0, sboard, node.value().2 + 1));
        create_all_possibilites_r(&mut snode, player);
        node.add_snode(snode);
    }
}

#[allow(dead_code)]
fn minimax(node: &Node<(i32, Game, i32)>, player: Player) -> i32 {
    if node.value().1.closed() {
        return node.value().0;
    }

    let mut value = -9000;
    if node.value().1.c_player() != player {
        for snode in node.snodes() {
            let b_value = minimax(snode, player.swap());
            value = if b_value > value { b_value } else { value }
        }
    } else {
        value = 9000;
        for snode in node.snodes() {
            let b_value = minimax(snode, player.swap());
            value = if b_value > value { value } else { b_value }
        }
    }

    value
}

trait GetNodeScore {
    fn snodes_scores(&self) -> Vec<i32>;
}

impl GetNodeScore for Node<(i32, Game, i32)> {
    fn snodes_scores(&self) -> Vec<i32> {
        self.snodes_values()
            .iter()
            .map(|v: &&(i32, Game, i32)| v.0)
            .collect()
    }
}

fn show_possibilities(node: &Node<(i32, Game, i32)>) {
    println!("Current Node:\n{}", node.value().1.bcontent());
    println!(
        "Sub-nodes: {}, values {:?}",
        node.size(0),
        node.snodes_scores()
    );

    let cmd: u32 = loop {
        let mut input = String::new();

        println!("1..n -> choisir une nth node\n 0 -> exit");

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
        0 => return,
        cmd if cmd <= node.size(0) as u32 => show_possibilities(node.snode_at((cmd - 1) as usize)),
        _ => return,
    }
}
