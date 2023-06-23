#[allow(unused_imports)]
use sudoku::{Board, Game, Player};
use trees::{Init, Node, Tree};

fn main() {
    let mut tree = Tree::init((0, Game::default()));
    let base_node = tree.base();
    create_all_possibilites_r(base_node, Player::X);
    println!("size: {}", tree.size());
    print!("minimax: {}", minimax(tree.base(), Player::X))
}

fn create_all_possibilites_r(node: &mut Node<(i32, Game)>, player: Player) {
    if node.value().1.closed() {
        match node.value().1.winner() {
            Player::Empty => node.value.0 = 0,
            e if e == player => node.value.0 = 1,
            _ => node.value.0 = 0,
        }
        return;
    }

    for pos in node.value().1.bcontent().empty_positions() {
        let mut sboard = node.value().1.clone();
        sboard.update_board(pos);

        let mut snode = trees::Node::init((node.value().0, sboard));
        create_all_possibilites_r(&mut snode, player);
        node.add_snode(snode);
    }
}

#[allow(dead_code)]
fn minimax(node: &Node<(i32, Game)>, player: Player) -> i32 {
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

#[allow(dead_code)]
fn show_tree(tree: Tree<(i32, Game)>) {
    let mut i = 0;
    for e in tree.content() {
        if i == 12 {
            return;
        }
        println!("{}", e.1.bcontent());
        i = i + 1;
    }
}
