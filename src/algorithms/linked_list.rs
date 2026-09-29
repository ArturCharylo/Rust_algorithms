use std::cell::RefCell;
use std::rc::{Rc, Weak};

// Strong reference to a node
type NodeRef<T> = Rc<RefCell<Node<T>>>;
// Weak reference to a node to prevent reference cycles
type WeakNodeRef<T> = Weak<RefCell<Node<T>>>;

pub struct Node<T> {
    pub val: T,
    pub next: Option<NodeRef<T>>,
    pub prev: Option<WeakNodeRef<T>>,
}

pub struct LinkedList<T> {
    head: Option<NodeRef<T>>,
    tail: Option<NodeRef<T>>,
}

pub struct IntoIter<T>(LinkedList<T>);

impl<T> LinkedList<T>{
    pub fn new() -> Self{
        Self {
            head: None,
            tail: None,
        }
    }
    pub fn push_front(&mut self, val:T) -> () {
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
    pub fn push_back(&mut self, val:T) -> () {
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
    pub fn pop_back(&mut self) -> Option<T> {
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
        // Extract the inner Node out of Rc and RefCell
        if let Ok(ref_cell) = Rc::try_unwrap(old_tail) {
            let node = ref_cell.into_inner();
            Some(node.val)
        } else {
            None
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
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
        // Extract the inner Node out of Rc and RefCell
        if let Ok(ref_cell) = Rc::try_unwrap(old_head) {
            let node = ref_cell.into_inner();
            Some(node.val)
        } else {
            None
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        // Wrap self into the IntoIter tuple struct
        IntoIter(self)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_back().is_some() {}
    }
}