use std::collections::VecDeque;
#[allow(unused_imports)]
use sudoku::{Board, Game, Player};
use trees::{Init, Node, Tree};

fn main() {
    let mut tree = Tree::init((Player::X, 0, Game::default()));
    let base_node = tree.base();
    create_all_possibilites(base_node);
    println!("size: {}", tree.size());
}

fn create_all_possibilites(node: &mut Node<(Player, i32, Game)>) {
    let mut file = VecDeque::from([node]);

    loop {
        let elem = match file.pop_front() {
            None => break,
            Some(e) => e,
        };

        if elem.value().2.closed() {
            match elem.value().2.winner() {
                Player::Empty => elem.value.1 = 0,
                e if e == elem.value().0 => elem.value.1 = 1,
                _ => elem.value.1 = 0,
            }
            continue;
        }

        for pos in elem.value().2.bcontent().empty_positions().iter() {
            let mut sboard = elem.value().2.clone();

            sboard.update_board_position(*pos);
            sboard.eval_end();
            sboard.change_current_player(); // si le board est close, le current player n'affecte pas les resultats car le winner est set

            let snode = trees::Node::init((elem.value().0, elem.value().1, sboard));
            elem.add_snode(snode);
        }

        let mut n_elem = VecDeque::from_iter(elem.mut_snodes());
        file.append(&mut n_elem);
    }
}

fn minmax(tree: &mut Tree<(Player, i32, Game)>) {
    // implem minimax :)
}
