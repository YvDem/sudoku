use sudoku::Game;
use trees::{Init, Node, Tree};

fn main() {
    let mut tree = trees::Tree::init(Game::default());
    let base_node = tree.base();
    create_all_possibilites(base_node)
}

fn create_all_possibilites(node: &mut Node<Game>) {
    for pos in node.value().empty_positions().iter() {
        let mut sboard = node.value().clone();
        sboard.update_board_position(pos.clone());

        let snode = trees::Node::init(sboard);
        node.add_snode(snode);
    }
}

#[allow(dead_code)]
fn show_all_possibilities(_tree: &Tree<Game>) {
    //for e in tree.content().iter() {}
}
