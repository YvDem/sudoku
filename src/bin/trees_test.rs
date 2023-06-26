use std::io;
#[allow(unused_imports)]
use sudoku::{Board, Game, Player};
use trees::{Init, Node};

fn main() {
    let mut b_tree = Node::init(Game::default());
    create_all_possibilites_r(&mut b_tree);
    println!("minimax: {}", minimax(&b_tree, 9, Player::X, true));
}

fn create_all_possibilites_r(node: &mut Node<Game>) {
    if node.value().closed() {
        return;
    }

    for pos in node.value().bcontent().empty_positions() {
        let mut sboard = node.value().clone();
        sboard.update_board(pos);

        let mut snode = Node::init(sboard);
        create_all_possibilites_r(&mut snode);
        node.add_snode(snode);
    }
}

fn minimax(node: &Node<Game>, depth: i32, player: Player, maxplayer: bool) -> i32 {
    if node.value().closed() || depth == 0 {
        match node.value().winner() {
            Player::Empty => return 0,
            pl if pl == player => return 10 - depth,
            _ => return depth - 10,
        }
    }

    if maxplayer {
        let mut val = -9000;
        for snode in node.snodes() {
            let sval = minimax(snode, depth - 1, player, false);
            val = if sval > val { sval } else { val };
        }
        return val;
    } else {
        let mut val = 9000;
        for snode in node.snodes() {
            let sval = minimax(snode, depth - 1, player, true);
            val = if sval > val { val } else { sval };
        }
        return val;
    }
}

trait GetNodeScore {
    fn snodes_boards(&self) -> Vec<&Board>;
}

impl GetNodeScore for Node<Game> {
    fn snodes_boards(&self) -> Vec<&Board> {
        self.snodes_values()
            .iter()
            .map(|v| v.bcontent())
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
