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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invert_binary_tree() {
        //        1
        //       / \
        //      2   3
        //     / \ / \
        //    4  5 6  7
        let mut tree = Some(Box::new(Node {
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
        }));
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
}
