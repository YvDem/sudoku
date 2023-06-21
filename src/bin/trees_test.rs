use std::collections::VecDeque;
use sudoku::Game;
use trees::{Init, Node, Tree};

fn main() {
    let mut tree = trees::Tree::init(Game::default());
    let base_node = tree.base();
    create_all_possibilites(base_node);
    println!("size: {}", tree.size());
}

fn create_all_possibilites(node: &mut Node<Game>) {
    let mut file = VecDeque::from([node]);

    loop {
        let elem = match file.pop_front() {
            None => break,
            Some(e) => e,
        };

        if elem.value().closed() {
            continue;
        }

        for pos in elem.value().bcontent().empty_positions().iter() {
            let mut sboard = elem.value().clone();

            sboard.update_board_position(*pos);
            sboard.eval_end();
            sboard.change_current_player(); // si le board est close, le current player n'affecte pas les resultats car le winner est set

            let snode = trees::Node::init(sboard);
            elem.add_snode(snode);
        }

        let mut n_elem = VecDeque::from_iter(elem.mut_snodes());
        file.append(&mut n_elem);
    }
}

fn show_all_possibilities(tree: &Tree<Game>) {
    for e in tree.content().iter() {
        println!("{}, prochain joueur: {}", e.board_content, e.current_player)
    }
}
