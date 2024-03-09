pub mod rbtree { // our public red black tree module, so we can publish crate, and use in main
use std::cell::RefCell; use std::option;
// interior mutability
use std::rc::{Rc, Weak}; // rc for multiple references
// weak is for parent pointers because we can't have cyclic strong references
// we can upgrade the parent pointers temporarily if we need to change parent values

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

type Tree = Rc<RefCell<TreeNode<u32>>>;
type RedBlackTree = Option<Tree>;

#[derive(Clone, Debug, PartialEq)]
pub struct TreeNode<T> {
    color: NodeColor,
    key: T,
    parent: RedBlackTree,
    left: RedBlackTree,
    right: RedBlackTree,
}

impl TreeNode<u32> {
    pub fn get_parent_key(&self) -> Option<u32> {
        if let Some(rc) = &self.parent {
            Some(rc.borrow().key)
        } else {
            None
        }
    }
    

    // used for creating root in RBtree implementation
    // notice tree type. returns pointer to the root that we can borrow and mutate
    pub fn new(key: u32) -> Tree { // create a new node
        Rc::new(RefCell::new(TreeNode {
            color: NodeColor::Red, // New nodes are always red
            key,
            parent: None,
            left: None,
            right: None,
        }))
    }

    // used in insert function. we return full RedBlackTree type. so we dont need to wrap Tree in some everytime
    pub fn new_rb(key: u32) -> RedBlackTree {
        Some(Rc::new(RefCell::new(TreeNode {
            color: NodeColor::Red,
            key,
            parent: None,
            left: None,
            right: None,
        })))
    }
    

    pub fn insert(&mut self, new_key: u32) {
        if new_key < self.key { // smaller value check left child
            match &self.left {
                Some(left_child) => left_child.borrow_mut().insert(new_key),
                None => {
                    let rc = TreeNode::new(new_key);
                    rc.borrow_mut().parent = Some(Rc::new(RefCell::new(self.clone())));
                    self.left = Some(rc);
                    
                }, 
            }
        } else if new_key > self.key { // larger value check right child
            match &self.right {
                Some(right_child) => right_child.borrow_mut().insert(new_key),
                None => {
                    let rc = TreeNode::new(new_key);
                    rc.borrow_mut().parent = Some(Rc::new(RefCell::new(self.clone())));
                    self.right = Some(rc);
                    
                },
            }
        }
    }

    pub fn pretty_print(&self, prefix: String, is_left: bool) {
        if let Some(right_child) = &self.right {
            right_child.borrow().pretty_print(
                format!("{}{}", prefix, if is_left { "│   " } else { "    " }),
                false,
            );
        }

        println!(
            "{}{}── {}{}{}",
            prefix,
            if is_left { "┌" } else { "└" },
            self.key,
            match self.color {
                NodeColor::Red => " (Red)",
                NodeColor::Black => " (Black)",
            },
            match self.get_parent_key() {
                Some(parent_key) => parent_key.to_string(),
                None => "(No parent)".to_string(), // No parent key available
            }
        );

        if let Some(left_child) = &self.left {
            left_child.borrow().pretty_print(
                format!("{}{}", prefix, if is_left { "    " } else { "│   " }),
                true,
            );
        }
    }

    // Helper method to start the pretty printing process
    pub fn print_tree(&self) {
        self.pretty_print(String::new(), false);
    }
}

pub struct RBTree {
    pub root: RedBlackTree,
}

impl RBTree {
    pub fn new_empty() -> Self {
        RBTree { root: None }
    }

    pub fn new(key: u32) -> Self {
        let root_node = TreeNode::new(key);
        root_node.borrow_mut().color = NodeColor::Black; // root node should be black

        RBTree { 
            root: Some(root_node),
        }
    }

    pub fn insert(&mut self, key: u32) {
        match &self.root {
            Some(root) => root.borrow_mut().insert(key),
            None => self.root = Some(TreeNode::new(key)),
        }
    }

    pub fn tree_pretty_print(&mut self) {
        if let Some(ref root) = self.root {
            // If the tree is not empty, insert the new key using the existing root
            root.borrow().print_tree();
        }
    }

    
}
}

// // avl tree implementation here
// i guess we take out the stuff that we need for both and put it outside hte 
// // pub mod avltree { ... }

