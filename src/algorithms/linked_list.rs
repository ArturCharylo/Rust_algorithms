use std::cell::RefCell;
use std::rc::{Rc, Weak};

// Strong reference to a node
type NodeRef = Rc<RefCell<Node>>;
// Weak reference to a node to prevent reference cycles
type WeakNodeRef = Weak<RefCell<Node>>;

pub struct Node {
    pub val: u32,
    pub next: Option<NodeRef>,
    pub prev: Option<WeakNodeRef>,
}

pub struct LinkedList {
    head: Option<NodeRef>,
    tail: Option<NodeRef>,
}

impl LinkedList{
    pub fn push_front(&mut self, val:u32) -> () {
        if self.head.is_none() {
            // Create a new isolated node wrapped for shared ownership and mutability
            let new_node = Rc::new(RefCell::new(Node {
                val,
                next: None,
                prev: None,
            }));
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(new_node);
        }
        else{
            let new_node = Rc::new(RefCell::new(Node {
                val,
                next: self.head.clone(),
                prev: None,
            }));
            // Borrow the current head mutably and point its prev back to new_node
            self.head
                .as_ref()
                .unwrap()
                .borrow_mut()
                .prev = Some(Rc::downgrade(&new_node));

            // Set the new node as the head of the list
            self.head = Some(new_node);
        }
    }
    pub fn push_back(&mut self, val:u32) -> () {
        if self.tail.is_none() {
            // Create a new isolated node wrapped for shared ownership and mutability
            let new_node = Rc::new(RefCell::new(Node {
                val,
                next: None,
                prev: None,
            }));
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(new_node);
        }
        else{
            let new_node = Rc::new(RefCell::new(Node {
                val,
                next: None,
                prev: Some(Rc::downgrade(&self.tail.as_ref().unwrap())),
            }));
            // Borrow the old tail mutably and point its next forward to new_node
            self.tail
                .as_ref()
                .unwrap()
                .borrow_mut()
                .next = Some(Rc::clone(&new_node));

            // Set the new node as the head of the list
            self.tail = Some(new_node);
        }
    }
}
