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