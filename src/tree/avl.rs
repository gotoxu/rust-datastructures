use crate::stack::Stack;
use std::collections::LinkedList;

#[derive(Clone)]
struct TreeNode<T>
where
    T: Ord,
{
    balance: i8,
    entry: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord> TreeNode<T> {
    fn new(value: T) -> TreeNode<T> {
        TreeNode {
            balance: 0,
            entry: value,
            left: None,
            right: None,
        }
    }
}

impl<T: Ord> PartialEq for TreeNode<T> {
    fn eq(&self, other: &TreeNode<T>) -> bool {
        std::ptr::eq(self, other)
    }
}

pub struct AvlTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
    number: u64,
}

impl<T: Ord> AvlTree<T> {
    pub fn new() -> AvlTree<T> {
        AvlTree {
            root: None,
            number: 0,
        }
    }

    pub fn insert(&mut self, entry: T) {
        match self.root {
            None => {
                let n = TreeNode::new(entry);
                self.root = Some(Box::new(n));
                self.number += 1;
            }
            Some(ref x) => {
                let mut dummy = x;
                loop {
                    if dummy.entry > entry {
                        match dummy.left {
                            None => break,
                            Some(ref x) => dummy = x,
                        }
                    } else if dummy.entry < entry {
                        match dummy.right {
                            None => break,
                            Some(ref x) => dummy = x,
                        }
                    } else {
                        return;
                    }
                }

                self.number += 1;
            }
        }
    }
}
