use std::collections::LinkedList;

pub mod avl;

#[cfg(test)]
mod tests {
    use crate::tree::{BinaryTree, TreeNode};

    #[test]
    fn it_works() {
        let mut tree = BinaryTree::new();
        tree.root = Some(Box::new(TreeNode {
            value: 1,
            left: Some(Box::new(TreeNode {
                value: 2,
                left: Some(Box::new(TreeNode {
                    value: 4,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    value: 5,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(TreeNode {
                value: 3,
                left: None,
                right: None,
            })),
        }));

        let d = tree.depth();
        assert_eq!(d, 3);
    }
}

struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn new(v: T) -> TreeNode<T> {
        TreeNode {
            value: v,
            left: None,
            right: None,
        }
    }
}

struct BinaryTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T> BinaryTree<T> {
    pub fn new() -> BinaryTree<T> {
        BinaryTree { root: None }
    }

    pub fn depth(&self) -> usize {
        let mut level: usize = 0;
        let mut front = 0;
        let mut rear = 0;
        let mut last = 1;

        match self.root {
            None => 0,
            Some(ref x) => {
                let mut queue = LinkedList::new();
                queue.push_front(x);
                rear += 1;

                while !queue.is_empty() {
                    let node = queue.pop_back().unwrap();
                    front += 1;

                    match node.left {
                        Some(ref x) => {
                            rear += 1;
                            queue.push_front(x);
                        }
                        None => (),
                    }

                    match node.right {
                        Some(ref x) => {
                            rear += 1;
                            queue.push_front(x);
                        }
                        None => (),
                    }

                    if front == last {
                        last = rear;
                        level += 1;
                    }
                }

                level
            }
        }
    }
}
