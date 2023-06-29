use std::io;
use sudoku::{Board, Game, Player};
use trees::{Init, Node};

fn main() {
    let mut b_tree = Node::init((Game::default(), 0));
    create_all_possibilites_r(&mut b_tree);
    minimax(&mut b_tree, 9, Player::X, true);
    show_possibilities(&b_tree);
}

pub fn create_all_possibilites_r(node: &mut Node<(Game, i32)>) {
    if node.value().0.closed() {
        return;
    }

    for pos in node.value().0.bcontent().empty_positions() {
        let mut sboard = node.value().0.clone();
        sboard.update_board(pos);

        let mut snode = Node::init((sboard, 0));
        create_all_possibilites_r(&mut snode);
        node.add_snode(snode);
    }
}

pub fn minimax(node: &mut Node<(Game, i32)>, depth: i32, player: Player, maxplayer: bool) -> i32 {
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

#[allow(dead_code)]
fn show_possibilities(node: &Node<(Game, i32)>) {
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
