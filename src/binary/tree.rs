#[derive(Debug, Clone)]
struct BTNode<T>
where
    T: Clone,
{
    value: T,
    left: Option<*mut BTNode<T>>,
    right: Option<*mut BTNode<T>>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct BinaryTree<T>
where
    T: Clone,
{
    root: Option<*mut BTNode<T>>,
}

impl<T: Clone> Drop for BinaryTree<T> {
    fn drop(&mut self) {
        if let Some(tree) = self.root {
            unsafe {
                let root_node = (*tree).clone();
                let _left = Self {
                    root: root_node.left,
                };
                let _right = Self {
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

    pub fn from_vec_pre(items: &[T]) -> Self {
        let length = items.len();
        if length == 0 {
            Self::new()
        } else {
            let root_item = items[0].to_owned();
            let half_length = if length % 2 == 1 {
                (length - 1) / 2
            } else {
                length / 2
            };
            let left_arr = &items[1..=half_length];
            let right_arr = &items[half_length + 1..];
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
                Self::pre_order(&(*curr).clone(), &mut path);
            };
        }
        path
    }

    pub fn in_order_search(&self) -> Vec<T> {
        let mut path = Vec::new();
        if let Some(curr) = self.root {
            unsafe {
                Self::in_order(&(*curr).clone(), &mut path);
            };
        }
        path
    }

    pub fn post_order_search(&self) -> Vec<T> {
        let mut path = Vec::new();
        if let Some(curr) = self.root {
            unsafe {
                Self::post_order(&(*curr).clone(), &mut path);
            };
        }
        path
    }

    fn pre_order(curr: &BTNode<T>, path: &mut Vec<T>) {
        path.push(curr.value.clone());

        if let Some(left) = curr.left {
            unsafe {
                Self::pre_order(&*left, path);
            }
        }

        if let Some(right) = curr.right {
            unsafe {
                Self::pre_order(&*right, path);
            }
        }
    }

    fn in_order(curr: &BTNode<T>, path: &mut Vec<T>) {
        if let Some(left) = curr.left {
            unsafe {
                Self::in_order(&*left, path);
            }
        }

        path.push(curr.value.clone());

        if let Some(right) = curr.right {
            unsafe {
                Self::in_order(&*right, path);
            }
        }
    }

    fn post_order(curr: &BTNode<T>, path: &mut Vec<T>) {
        if let Some(left) = curr.left {
            unsafe {
                Self::post_order(&*left, path);
            }
        }

        if let Some(right) = curr.right {
            unsafe {
                Self::post_order(&*right, path);
            }
        }

        path.push(curr.value.clone());
    }
}
