use super::dsf_type::DFSType;
use linked::queue::Queue;

#[derive(Debug, Clone)]
struct BTNode<T>
where
    T: std::fmt::Debug + Clone + PartialEq,
{
    value: T,
    left: Option<*mut BTNode<T>>,
    right: Option<*mut BTNode<T>>,
}

impl<T: std::fmt::Debug + Clone + PartialEq> PartialEq for BTNode<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.value == other.value {
            match (self.left, self.right, other.left, other.right) {
                (None, None, None, None) => true,
                (Some(sl), None, Some(ol), None) => unsafe { *sl == *ol },
                (None, Some(sr), None, Some(or)) => unsafe { *sr == *or },
                (Some(sl), Some(sr), Some(ol), Some(or)) => unsafe { *sl == *ol && *sr == *or },
                _ => false,
            }
        } else {
            false
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct BinarySearchTree<T>
where
    T: std::fmt::Debug + Clone + PartialEq + PartialOrd,
{
    root: Option<*mut BTNode<T>>,
}

impl<T: std::fmt::Debug + Clone + PartialEq + PartialOrd> PartialEq for BinarySearchTree<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self.root, other.root) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(s), Some(o)) => unsafe { *s == *o },
        }
    }
}

impl<T: std::fmt::Debug + Clone + PartialEq + PartialOrd> Drop for BinarySearchTree<T> {
    fn drop(&mut self) {
        if let Some(root) = self.root {
            unsafe {
                let left = (*root).left;
                let _ = Self { root: left };
                let right = (*root).right;
                let _ = Self { root: right };
                drop(Box::from_raw(root));
            }
        }
    }
}

impl<T: std::fmt::Debug + Clone + PartialEq + PartialOrd> BinarySearchTree<T> {
    pub const fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, item: T) {
        if let Some(root_pointer) = self.root.as_mut() {
            unsafe {
                let mut root_node = *root_pointer;
                if item > (*root_node).value {
                    let mut tmp_tree = Self {
                        root: (*root_node).right,
                    };
                    Self::insert(&mut tmp_tree, item);
                    (*root_node).right = tmp_tree.root;
                    std::mem::forget(tmp_tree);
                } else {
                    let mut tmp_tree = Self {
                        root: (*root_node).left,
                    };
                    Self::insert(&mut tmp_tree, item);
                    (*root_node).left = tmp_tree.root;
                    std::mem::forget(tmp_tree);
                }
            }
        } else {
            let root = Some(Box::into_raw(Box::new(BTNode {
                value: item,
                left: None,
                right: None,
            })));
            self.root = root;
        }
    }

    pub fn from_vec(items: &[T]) -> Self {
        let mut out = Self::new();
        for i in items {
            out.insert(i.clone());
        }
        out
    }

    pub fn breadth_first_search(&self) -> Vec<T> {
        let mut bfs = Vec::new();

        if let Some(root) = self.root {
            let mut out = Queue::new();
            unsafe {
                let root_node = (*root).clone();
                out.enqueue(root_node);
                while out.length > 0 {
                    if let Some(curr) = out.peek() {
                        let curr_node = curr;
                        if let Some(left) = curr_node.left {
                            out.enqueue((*left).clone());
                            print!("Some({:?}) ", (*left).clone().value);
                        } else {
                            print!("None ");
                        }
                        if let Some(right) = curr_node.right {
                            out.enqueue((*right).clone());
                            print!("Some({:?}) ", (*right).clone().value);
                        } else {
                            print!("None ");
                        }
                        if let Some(out) = out.dequeue() {
                            bfs.push(out.value);
                        }
                    }
                }
            }
        }
        println!();
        bfs
    }

    pub fn dfs_order_search(&self, order: DFSType) -> Vec<T> {
        let mut path = Vec::new();
        if let Some(curr) = self.root {
            unsafe {
                Self::order(&(*curr).clone(), &mut path, order);
            };
        }
        path
    }

    fn order(curr: &BTNode<T>, path: &mut Vec<T>, order: DFSType) {
        let root_push = { |path: &mut Vec<T>| path.push(curr.value.clone()) };
        let left_push = {
            |path: &mut Vec<T>| {
                if let Some(left) = curr.left {
                    unsafe {
                        Self::order(&*left, path, order);
                    }
                }
            }
        };
        let right_push = {
            |path: &mut Vec<T>| {
                if let Some(right) = curr.right {
                    unsafe {
                        Self::order(&*right, path, order);
                    }
                }
            }
        };

        match order {
            DFSType::Pre => {
                root_push(path);
                left_push(path);
                right_push(path);
            }
            DFSType::In => {
                left_push(path);
                root_push(path);
                right_push(path);
            }
            DFSType::Post => {
                left_push(path);
                right_push(path);
                root_push(path);
            }
        }
    }
}
