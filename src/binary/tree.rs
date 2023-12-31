use super::dsf_type::DFSType;
use linked::queue::Queue;

#[derive(Debug, Clone)]
struct BTNode<T>
where
    T: Clone + PartialEq,
{
    value: T,
    left: Option<*mut BTNode<T>>,
    right: Option<*mut BTNode<T>>,
}

impl<T: Clone + PartialEq> PartialEq for BTNode<T> {
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
pub struct BinaryTree<T>
where
    T: Clone + PartialEq,
{
    root: Option<*mut BTNode<T>>,
}

impl<T: Clone + PartialEq> PartialEq for BinaryTree<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self.root, other.root) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(s), Some(o)) => unsafe { *s == *o },
        }
    }
}

impl<T: Clone + PartialEq> Drop for BinaryTree<T> {
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

impl<T: Clone + PartialEq> BinaryTree<T> {
    pub const fn new() -> Self {
        Self { root: None }
    }

    pub fn from_vec_order(items: &[T], order: DFSType) -> Self {
        let length = items.len();
        if length == 0 {
            Self::new()
        } else {
            let half_length = if length % 2 == 1 {
                (length - 1) / 2
            } else {
                length / 2
            };
            let (root_item, left_arr, right_arr) = match order {
                DFSType::Pre => {
                    let root_item = items[0].to_owned();
                    let left_arr = &items[1..=half_length];
                    let right_arr = &items[half_length + 1..];
                    (root_item, left_arr, right_arr)
                }
                DFSType::In => {
                    let root_item = items[half_length].to_owned();
                    let left_arr = &items[0..half_length];
                    let right_arr = &items[half_length + 1..];
                    (root_item, left_arr, right_arr)
                }
                DFSType::Post => {
                    let root_item = items[length - 1].to_owned();
                    let left_arr = &items[0..half_length];
                    let right_arr = &items[half_length..length - 1];
                    (root_item, left_arr, right_arr)
                }
            };

            let left = Self::from_vec_order(left_arr, order);
            let right = Self::from_vec_order(right_arr, order);
            let root_node = BTNode {
                value: root_item,
                left: left.root,
                right: right.root,
            };
            let box_node = Box::new(root_node);
            let raw_root = Box::into_raw(box_node);
            std::mem::forget(left);
            std::mem::forget(right);
            Self {
                root: Some(raw_root),
            }
        }
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
                        }
                        if let Some(right) = curr_node.right {
                            out.enqueue((*right).clone());
                        }
                        if let Some(out) = out.dequeue() {
                            bfs.push(out.value);
                        }
                    }
                }
            }
        }

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
