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
    pub fn new() -> Self{
        Self {
            head: None,
            tail: None,
        }
    }
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
    pub fn pop_back(&mut self) -> Option<u32> {
        // If self.tail is None, ? immediately returns None from the function
        let old_tail = self.tail.take()?;
        // Check if the removed node had a predecessor
        let prev_node = old_tail.borrow_mut().prev.take().and_then(|weak| weak.upgrade());
        match prev_node {
        Some(new_tail) => {
                new_tail.borrow_mut().next = None;
                self.tail = Some(new_tail);
            }
            None => {
                self.head = None;
            }
        }
        let val = old_tail.borrow().val;
        Some(val)
    }
    pub fn pop_front(&mut self) -> Option<u32> {
        let old_head = self.head.take()?;
        let next_node = old_head.borrow_mut().next.take();
        match next_node {
            Some(next_node) => {
                next_node.borrow_mut().prev = None;
                self.head = Some(next_node);
            }
            None => {
                self.tail = None;
            }
        }
        let val = old_head.borrow().val;
        Some(val)
    }
}

impl Default for LinkedList {
    fn default() -> Self {
        Self::new()
    }
}