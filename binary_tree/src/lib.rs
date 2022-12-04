use std::cmp::Ordering;

#[derive(PartialEq)]
pub struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub fn invert_tree(node: &mut Option<Box<Node>>) {
    match node {
        Some(node) => {
            std::mem::swap(&mut node.left, &mut node.right);
            invert_tree(&mut node.left);
            invert_tree(&mut node.right)
        }
        None => return,
    }
}

pub fn search_tree(node: &Option<Box<Node>>, value: i32) -> i32 {
    match node {
        Some(node) => {
            match value.cmp(&node.value) {
                Ordering::Equal => node.value,
                Ordering::Less => search_tree(&node.left, value),
                Ordering::Greater => search_tree(&node.right, value),
           }
        },
        None => 0,
    }
}

pub fn sum_tree(node: Option<Box<Node>>) -> i32 {
    match node {
        Some(node) => node.value + sum_tree(node.left) + sum_tree(node.right),
        None => 0,
    } 
}

pub fn max_tree(node: Option<Box<Node>>, max: &mut i32) -> i32 {
    match node {
        Some(node) => {
            if &node.value > max {
                *max = node.value
            }
            let mut max_left: i32 = max_tree(node.left, max);
            max_tree(node.right, &mut max_left)
        },
        None => *max,
    }
}

// TODO
// 1. implement insert/delete in binary tree
// 2. convert binary tree to array
// 3. concert array to binary tree
// 4. flatten binary tree
// 5. balance binary tree
// 6. maximum binary tree https://leetcode.com/problems/maximum-binary-tree/
// 7. Binary Tree Paths https://leetcode.com/problems/binary-tree-paths/
// 8. Maximum Depth of Binary Tree https://leetcode.com/problems/maximum-depth-of-binary-tree
// 9. Construct String from Binary Tree https://leetcode.com/problems/construct-string-from-binary-tree
// 10. Flatten Binary Tree to Linked List https://leetcode.com/problems/flatten-binary-tree-to-linked-list/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invert_binary_tree() {
        let mut tree = build_binary_tree();
        invert_tree(&mut tree);
        //        1
        //       / \
        //      3   2
        //     / \ / \
        //    7  6 5  4
        assert_eq!(tree.as_ref().unwrap().left.as_ref().unwrap().value, 3);
        assert_eq!(tree.as_ref().unwrap().left.as_ref().unwrap().left.as_ref().unwrap().value, 7);
        assert_eq!(tree.as_ref().unwrap().left.as_ref().unwrap().right.as_ref().unwrap().value, 6);

        assert_eq!(tree.as_ref().unwrap().right.as_ref().unwrap().value, 2);
        assert_eq!(tree.as_ref().unwrap().right.as_ref().unwrap().left.as_ref().unwrap().value, 5);
        assert_eq!(tree.as_ref().unwrap().right.as_ref().unwrap().right.as_ref().unwrap().value, 4);
    }

    #[test]
    fn search_binary_tree() {
        let tree = build_binary_tree();
        let value_to_search = 1;
        let value = search_tree(&tree, value_to_search);
        assert_eq!(value, value_to_search);

        let value_to_search = 3;
        let value = search_tree(&tree, value_to_search);
        assert_eq!(value, value_to_search);
    }

    #[test]
    fn sum_binary_tree() {
        let tree = build_binary_tree();
        let sum = sum_tree(tree);
        assert_eq!(sum, 28);
    }

    #[test]
    fn max_binary_tree() {
        let tree = build_binary_tree();
        let mut max = 0;
        let max = max_tree(tree, &mut max);
        assert_eq!(max, 7);
    }

    //        1
    //       / \
    //      2   3
    //     / \ / \
    //    4  5 6  7
    fn build_binary_tree() -> Option<Box<Node>> {
        Some(Box::new(Node {
            value: 1,
            left: Some(Box::new(Node {
                value: 2,
                left: Some(Box::new(Node {
                    value: 4,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: 5,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(Node {
                value: 3,
                left: Some(Box::new(Node {
                    value: 6,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: 7,
                    left: None,
                    right: None,
                })),
            })),
        }))
    }
}
