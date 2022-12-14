use std::cmp;
use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
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
    let node = match node {
        Some(node) => node,
        None => return 0,
    };
    return match value.cmp(&node.value) {
        Ordering::Equal => node.value,
        Ordering::Less => search_tree(&node.left, value),
        Ordering::Greater => search_tree(&node.right, value),
    };
}

// return the sum of all values from the binary tree
pub fn sum_tree(node: Option<Box<Node>>) -> i32 {
    match node {
        Some(node) => node.value + sum_tree(node.left) + sum_tree(node.right),
        None => 0,
    }
}

// return the max value from the binary tree
pub fn max_tree(node: Option<Box<Node>>, max: &mut i32) -> i32 {
    match node {
        Some(node) => {
            let mut max = cmp::max(node.value, *max);
            cmp::max(
                max_tree(node.right, &mut max),
                max_tree(node.left, &mut max),
            )
        }
        None => *max,
    }
}

// maximum depth of binary tree https://leetcode.com/problems/maximum-depth-of-binary-tree
// DFS: Time O(n) | Space O(n)
pub fn max_depth_tree(node: Option<Box<Node>>) -> i32 {
    match node {
        Some(node) => 1 + cmp::max(max_depth_tree(node.left), max_depth_tree(node.right)),
        None => 0,
    }
}

// maximum depth of binary tree https://leetcode.com/problems/maximum-depth-of-binary-tree
// BFS: Time O(n) | Space O(n)
pub fn max_depth_tree_bfs(node: Option<Box<Node>>) -> i32 {
    let node = match node {
        Some(n) => n,
        None => return 0,
    };
    let mut depth = 0;
    let mut queue = VecDeque::from([&node]);
    while !queue.is_empty() {
        for _ in 0..queue.len() {
            if let Some(current) = queue.pop_front() {
                if let Some(left) = &current.as_ref().left {
                    queue.push_back(left);
                }
                if let Some(right) = &current.as_ref().right {
                    queue.push_back(right);
                }
            }
        }
        depth += 1; // inc per node level
    }
    return depth;
}

pub fn max_depth_tree_iterative_dfs(node: Option<Box<Node>>) -> i32 {
    let node = match node {
        Some(n) => n,
        None => return 0,
    };
    let mut max_depth = 0;
    let mut stack = VecDeque::from([(&node, max_depth)]);
    while !stack.is_empty() {
        if let Some((current, mut depth)) = stack.pop_back() {
            depth += 1;
            max_depth = cmp::max(max_depth, depth);
            if let Some(left) = &current.as_ref().left {
                stack.push_back((&left, depth));
            };
            if let Some(right) = &current.as_ref().right {
                stack.push_back((&right, depth));
            };
        };
    }
    return max_depth;
}

// TODO
// 1. implement insert/delete in binary tree
// 2. convert binary tree to array
// 3. concert array to binary tree
// 4. flatten binary tree
// 5. balance binary tree
// 6. maximum binary tree https://leetcode.com/problems/maximum-binary-tree/
// 7. Binary Tree Paths https://leetcode.com/problems/binary-tree-paths/
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
        assert_eq!(
            tree.as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .value,
            7
        );
        assert_eq!(
            tree.as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .value,
            6
        );

        assert_eq!(tree.as_ref().unwrap().right.as_ref().unwrap().value, 2);
        assert_eq!(
            tree.as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .value,
            5
        );
        assert_eq!(
            tree.as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .value,
            4
        );
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

    #[test]
    fn max_depth_binary_tree() {
        let tree = build_binary_tree();
        let max_depth = max_depth_tree(tree);
        assert_eq!(max_depth, 3);
    }

    #[test]
    fn max_depth_binary_tree_bfs() {
        let tree = build_binary_tree();
        let max_depth = max_depth_tree_bfs(tree);
        assert_eq!(max_depth, 3);
    }

    #[test]
    fn max_depth_binary_tree_iterative_dfs() {
        let tree = build_binary_tree();
        let max_depth = max_depth_tree_iterative_dfs(tree);
        assert_eq!(max_depth, 3);

        let unbalanced_tree = build_unbalanced_binary_tree();
        let max_depth = max_depth_tree_iterative_dfs(unbalanced_tree);
        assert_eq!(max_depth, 3);
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

    //        3
    //       / \
    //      9   20
    //          / \
    //         15  7
    fn build_unbalanced_binary_tree() -> Option<Box<Node>> {
        Some(Box::new(Node {
            value: 3,
            left: Some(Box::new(Node {
                value: 9,
                left: None,
                right: None,
            })),
            right: Some(Box::new(Node {
                value: 20,
                left: Some(Box::new(Node {
                    value: 15,
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
