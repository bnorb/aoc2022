mod tree;

use std::{cell::RefCell, rc::Rc};
use self::tree::Node;

#[aoc_generator(day7)]
fn generate_input(input: &str) -> Rc<RefCell<Node>> {
    tree::parse(input)
}

#[aoc(day7, part1)]
pub fn sum_small_100k_dirs(root: &Rc<RefCell<Node>>) -> u32 {
    let mut nodes = vec![Rc::clone(root)];
    nodes.extend(root.borrow().flatten_children());

    nodes
        .iter()
        .map(|node| node.borrow_mut().size())
        .filter(|size| *size <= 100000)
        .sum()
}

#[aoc(day7, part2)]
pub fn find_deletable(root: &Rc<RefCell<Node>>) -> u32 {
    let mut nodes = vec![Rc::clone(root)];
    nodes.extend(root.borrow().flatten_children());

    let needed = 30000000 - (70000000 - root.borrow_mut().size());

    nodes
        .iter()
        .map(|node| node.borrow_mut().size())
        .filter(|size| *size >= needed)
        .min()
        .unwrap()
}
