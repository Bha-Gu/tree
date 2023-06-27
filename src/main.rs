mod binary;
use binary::tree::BinaryTree;

fn main() {
    let tree = BinaryTree::from_vec_pre(&[7, 23, 5, 4, 3, 18, 21]);
    let pre = tree.pre_order_search();
    let in_o = tree.in_order_search();
    let post = tree.post_order_search();
    let bfs = tree.breadth_first_search();
    println!("{pre:?}");
    println!("{in_o:?}");
    println!("{post:?}");
    println!("{bfs:?}");
    println!("{tree:?}");
}
