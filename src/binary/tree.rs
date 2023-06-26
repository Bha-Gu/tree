#[derive(Debug, Clone)]
struct BTNode<T>
where
    T: Clone,
{
    value: T,
    left: Option<*mut BTNode<T>>,
    right: Option<*mut BTNode<T>>,
}

#[derive(Debug, Clone)]
pub struct BinaryTree<T>
where
    T: Clone,
{
    root: Option<*mut BTNode<T>>,
}

#[derive(PartialEq, Clone, Copy)]
enum WalkType {
    Pre,
    In,
    Post,
}

impl<T: Clone> Drop for BinaryTree<T> {
    fn drop(&mut self) {
        if let Some(tree) = self.root {
            unsafe {
                let root_node = (*tree).clone();
                let _left = BinaryTree {
                    root: root_node.left,
                };
                let _right = BinaryTree {
                    root: root_node.right,
                };
            }
        }
    }
}

impl<T: Clone> BinaryTree<T> {
    pub const fn new() -> Self {
        Self { root: None }
    }

    pub fn from_vec_pre(items: Vec<T>) -> Self {
        let length = items.len();
        if length == 0 {
            Self::new()
        } else {
            let plength = length;
            let skip = {
                let mut result = 1;
                while result * 2 <= plength {
                    result *= 2;
                }
                result - 1
            };
            let root_item = items[0].to_owned();
            let mut left_arr = vec![];
            for i in 0..skip {
                left_arr.push(items[i + 1].to_owned());
            }
            let left_arr = left_arr;
            let mut right_arr = vec![];
            for i in skip..length - 1 {
                right_arr.push(items[i + 1].to_owned());
            }
            let right_arr = right_arr;
            let left = Self::from_vec_pre(left_arr);
            let right = Self::from_vec_pre(right_arr);
            let root_node = BTNode {
                value: root_item,
                left: left.root,
                right: right.root,
            };
            let box_node = Box::new(root_node);
            let raw_root = Box::into_raw(box_node);
            Self {
                root: Some(raw_root),
            }
        }
    }

    pub fn pre_order_search(&self) -> Vec<T> {
        let mut path = Vec::new();
        if let Some(curr) = self.root {
            unsafe {
                Self::walk((*curr).clone(), &mut path, WalkType::Pre);
            };
        }
        path
    }

    pub fn in_order_search(&self) -> Vec<T> {
        let mut path = Vec::new();
        if let Some(curr) = self.root {
            unsafe {
                Self::walk((*curr).clone(), &mut path, WalkType::In);
            };
        }
        path
    }

    pub fn post_order_search(&self) -> Vec<T> {
        let mut path = Vec::new();
        if let Some(curr) = self.root {
            unsafe {
                Self::walk((*curr).clone(), &mut path, WalkType::Post);
            };
        }
        path
    }

    fn walk(curr: BTNode<T>, path: &mut Vec<T>, walk: WalkType) {
        //pre
        if walk == WalkType::Pre {
            path.push((curr).value.clone());
        }

        //recurse
        unsafe {
            if let Some(left) = curr.left {
                Self::walk((*left).clone(), path, walk);
            }
            if walk == WalkType::In {
                path.push((curr).value.clone());
            }
            if let Some(right) = curr.right {
                Self::walk((*right).clone(), path, walk);
            }
        }
        //post
        if walk == WalkType::Post {
            path.push((curr).value);
        }
    }
}
