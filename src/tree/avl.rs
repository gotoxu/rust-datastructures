use std::cmp::Ordering;

#[cfg(test)]
mod tests {
    use crate::tree::avl::AvlTree;

    #[test]
    fn insert_test() {
        let mut tree = AvlTree::new();
        tree.insert(4);
        tree.insert(2);
        tree.insert(6);
        tree.insert(1);
        tree.insert(3);
        tree.insert(5);
        tree.insert(7);
        tree.insert(16);
        tree.insert(15);
        tree.insert(14);
        tree.insert(13);
        tree.insert(12);
        tree.insert(11);
        tree.insert(9);

        assert_eq!(tree.root.unwrap().entry, 7)
    }
}

struct TreeNode<T>
where
    T: Ord + Copy,
{
    entry: T,
    height: usize,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Copy> TreeNode<T> {
    fn new(value: T) -> TreeNode<T> {
        TreeNode {
            entry: value,
            height: 0,
            left: None,
            right: None,
        }
    }
}

pub struct AvlTree<T>
where
    T: Ord + Copy,
{
    root: Option<Box<TreeNode<T>>>,
    number: u64,
}

impl<T: Ord + Copy> AvlTree<T> {
    pub fn new() -> AvlTree<T> {
        AvlTree {
            root: None,
            number: 0,
        }
    }

    pub fn insert(&mut self, entry: T) {
        match self.root.take() {
            None => self.root = Some(Box::new(TreeNode::new(entry))),
            Some(n) => self.root = Some(insert(n, entry)),
        }
    }
}

fn insert<T: Ord + Copy>(mut node: Box<TreeNode<T>>, entry: T) -> Box<TreeNode<T>> {
    match node.entry.cmp(&entry) {
        Ordering::Equal => {
            node.entry = entry;
            return node;
        }
        Ordering::Greater => node.left = insert_to(node.left.take(), entry),
        Ordering::Less => node.right = insert_to(node.right.take(), entry),
    }

    update_height(&mut *node);
    rotate(node)
}

fn insert_to<T: Ord + Copy>(on: Option<Box<TreeNode<T>>>, entry: T) -> Option<Box<TreeNode<T>>> {
    Some(match on {
        Some(n) => insert(n, entry),
        None => Box::new(TreeNode::new(entry)),
    })
}

fn rotate<T: Ord + Copy>(mut root: Box<TreeNode<T>>) -> Box<TreeNode<T>> {
    let l = height(&root.left);
    let r = height(&root.right);
    let diff = l as i32 - r as i32;

    if -1 <= diff && diff <= 1 {
        return root;
    }

    match diff {
        2 => {
            let left = root.left.take().expect("AVL broken");
            if height(&left.left) < height(&left.right) {
                let rotated = left_left_rotation(left);
                root.left = Some(rotated);
                update_height(&mut root);
            } else {
                root.left = Some(left);
            }

            right_right_rotation(root)
        }
        -2 => {
            let right = root.right.take().expect("AVL broken");
            if height(&right.left) > height(&right.right) {
                let rotated = right_right_rotation(right);
                root.right = Some(rotated);
                update_height(&mut root);
            } else {
                root.right = Some(right);
            }

            left_left_rotation(root)
        }
        _ => unreachable!(),
    }
}

fn left_left_rotation<T: Ord + Copy>(mut root: Box<TreeNode<T>>) -> Box<TreeNode<T>> {
    let mut new_root = root.right.take().expect("AVL broken");
    root.right = new_root.left.take();
    update_height(&mut root);
    new_root.left = Some(root);
    update_height(&mut new_root);

    new_root
}

fn right_right_rotation<T: Ord + Copy>(mut root: Box<TreeNode<T>>) -> Box<TreeNode<T>> {
    let mut new_root = root.left.take().expect("AVL broken");
    root.left = new_root.right.take();
    update_height(&mut root);
    new_root.right = Some(root);
    update_height(&mut new_root);

    new_root
}

fn update_height<T: Ord + Copy>(root: &mut TreeNode<T>) {
    root.height = std::cmp::max(height(&root.left), height(&root.right)) + 1;
}

fn height<T: Ord + Copy>(node: &Option<Box<TreeNode<T>>>) -> usize {
    return node.as_ref().map_or(0, |n| n.height);
}
