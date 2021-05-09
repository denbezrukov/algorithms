struct TreeNode<T> {
    value: T,
    children: Vec<TreeNode<T>>,
}

impl<T> TreeNode<T> {
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            children: Vec::new(),
        }
    }

    pub fn push_back(&mut self, node: TreeNode<T>) {
        self.children.push(node);
    }

    pub fn pop_child(&mut self) -> Option<T> {
        self.children.pop().map(|node| node.value)
    }
}

// impl<T> TreeNode<T>
// where
//     T: PartialEq,
// {
//     pub fn search(&self, value: &T) -> bool {
//         if &self.value == value {
//             true
//         } else {
//             self.children
//                 .iter()
//                 .find(|&node| node.search(value))
//                 .is_some()
//         }
//     }
// }

#[cfg(test)]
mod test {
    use crate::tree::TreeNode;

    #[test]
    fn add_child() {
        let mut root = TreeNode::new(1);
        let node = TreeNode::new(2);
        root.push_back(node);

        assert_eq!(root.pop_child(), Some(2));
    }

    // #[test]
    // fn search() {
    //     let mut root = TreeNode::new(1);
    //     let mut first_node = TreeNode::new(2);
    //     let second_node = TreeNode::new(3);
    //
    //     first_node.push_back(second_node);
    //     root.push_back(first_node);
    //
    //     assert!(root.search(&3));
    // }
}
