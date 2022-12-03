#[derive(Debug, PartialEq)]
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
        let mut tree = Some(Box::new(Node {
            value: 1,
            left: Some(Box::new(Node {
                value: 2,
                left: None,
                right: None,
            })),
            right: Some(Box::new(Node {
                value: 5,
                left: None,
                right: None,
            })),
        }));
        invert_tree(&mut tree);
        assert_eq!(tree.as_ref().unwrap().left.as_ref().unwrap().value, 5);
        assert_eq!(tree.as_ref().unwrap().right.as_ref().unwrap().value, 2);
    }
}
