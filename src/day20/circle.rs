use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct Node {
    value: i64,
    next: Weak<RefCell<Node>>,
    prev: Weak<RefCell<Node>>,
}

impl Node {
    fn new(value: i64) -> Self {
        Self {
            value,
            next: Weak::new(),
            prev: Weak::new(),
        }
    }
}

pub struct CircularList {
    zero: Rc<RefCell<Node>>,
    original_order: Vec<Rc<RefCell<Node>>>,
    size: usize,
}

impl CircularList {
    pub fn from(values: &Vec<i64>) -> Self {
        assert!(values.len() > 0);

        let original_order: Vec<Rc<RefCell<Node>>> = values
            .iter()
            .map(|val| Rc::new(RefCell::new(Node::new(*val))))
            .collect();

        let mut iter = original_order.iter();

        let first = Rc::clone(iter.next().unwrap());
        let mut zero = Rc::clone(&first);

        let mut prev = Rc::clone(&first);
        while let Some(node) = iter.next() {
            let current = Rc::clone(node);
            current.borrow_mut().prev = Rc::downgrade(&prev);
            prev.borrow_mut().next = Rc::downgrade(&current);

            if current.borrow().value == 0 {
                zero = Rc::clone(&current);
            }

            prev = current;
        }

        prev.borrow_mut().next = Rc::downgrade(&first);
        first.borrow_mut().prev = Rc::downgrade(&prev);

        Self {
            zero,
            size: original_order.len(),
            original_order,
        }
    }

    fn move_node_back(&self, node: Rc<RefCell<Node>>, steps: usize) {
        let mut target = Rc::clone(&node);
        for _ in 0..steps {
            let prev = target.borrow().prev.upgrade().unwrap();
            target = prev;
        }

        let node_next = node.borrow().next.upgrade().unwrap();
        let node_prev = node.borrow().prev.upgrade().unwrap();
        let target_prev = target.borrow().prev.upgrade().unwrap();

        node.borrow_mut().next = Rc::downgrade(&target);
        node.borrow_mut().prev = Rc::downgrade(&target_prev);
        target_prev.borrow_mut().next = Rc::downgrade(&node);
        target.borrow_mut().prev = Rc::downgrade(&node);
        node_prev.borrow_mut().next = Rc::downgrade(&node_next);
        node_next.borrow_mut().prev = Rc::downgrade(&node_prev);
    }

    fn move_node_forward(&self, node: Rc<RefCell<Node>>, steps: usize) {
        let mut target = Rc::clone(&node);
        for _ in 0..steps {
            let next = target.borrow().next.upgrade().unwrap();
            target = next;
        }

        let node_next = node.borrow().next.upgrade().unwrap();
        let node_prev = node.borrow().prev.upgrade().unwrap();
        let target_next = target.borrow().next.upgrade().unwrap();

        node.borrow_mut().next = Rc::downgrade(&target_next);
        node.borrow_mut().prev = Rc::downgrade(&target);
        target_next.borrow_mut().prev = Rc::downgrade(&node);
        target.borrow_mut().next = Rc::downgrade(&node);
        node_prev.borrow_mut().next = Rc::downgrade(&node_next);
        node_next.borrow_mut().prev = Rc::downgrade(&node_prev);
    }

    fn move_node(&self, node: Rc<RefCell<Node>>) {
        let val = node.borrow().value;
        let steps = val.abs() as usize % (self.size - 1);
        if steps == 0 {
            return;
        }

        if val < 0 {
            self.move_node_back(node, steps);
        } else {
            self.move_node_forward(node, steps);
        }
    }

    pub fn move_all(&self) {
        for node in self.original_order.iter() {
            self.move_node(Rc::clone(&node));
        }
    }

    pub fn find_coord(&self, coord: usize) -> i64 {
        let steps = coord % (self.size - 1);
        let mut current = Rc::clone(&self.zero);
        for _ in 0..steps {
            let next = current.borrow().next.upgrade().unwrap();
            current = next;
        }

        let val = current.borrow().value;
        val
    }
}
