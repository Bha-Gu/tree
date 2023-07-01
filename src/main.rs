mod binary;
use binary::dsf_type::DFSType;
use binary::search_tree::BinarySearchTree;
use binary::tree::BinaryTree;

fn main() {
    let tree = BinaryTree::from_vec_order(&[7, 23, 5, 4, 3, 18, 21], DFSType::Pre);
    let tree2 = BinaryTree::from_vec_order(&[1, 8, 2, 7, 3, 6, 4, 5], DFSType::In);
    let pre = tree.dfs_order_search(DFSType::Pre);
    let in_o = tree.dfs_order_search(DFSType::In);
    let post = tree.dfs_order_search(DFSType::Post);
    let bfs = tree2.breadth_first_search();
    let bst = (BinarySearchTree::from_vec(&bfs)).breadth_first_search();
    println!("{pre:?}");
    println!("{in_o:?}");
    println!("{post:?}");
    println!("{bfs:?}");
    println!("{bst:?}");
    println!("{:?}", tree == tree2);
}
