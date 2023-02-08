use std::{cell::RefCell, collections::LinkedList};

#[derive(Debug, PartialEq, Eq, Ord)]
pub enum Packet {
    List(Vec<Box<Packet>>),
    Num(u8),
}

impl Packet {
    pub fn parse(input: &str) -> Self {
        let mut iter = input.chars();
        let stack: RefCell<LinkedList<Vec<Box<Packet>>>> = RefCell::new(LinkedList::new());
        let current_num = RefCell::new(String::new());

        let add_num = || {
            if current_num.borrow().len() == 0 {
                return;
            }

            stack
                .borrow_mut()
                .back_mut()
                .unwrap()
                .push(Box::new(Packet::Num(current_num.borrow().parse().unwrap())));
            current_num.borrow_mut().clear();
        };

        while let Some(c) = iter.next() {
            match c {
                '[' => stack.borrow_mut().push_back(Vec::new()),
                ']' => {
                    add_num();
                    if stack.borrow().len() == 1 {
                        return Self::List(stack.borrow_mut().pop_back().unwrap());
                    } else {
                        let closed = stack.borrow_mut().pop_back().unwrap();
                        stack
                            .borrow_mut()
                            .back_mut()
                            .unwrap()
                            .push(Box::new(Self::List(closed)));
                    }
                }
                ',' => add_num(),
                num => current_num.borrow_mut().push(num),
            }
        }

        unreachable!()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Num(a), Self::Num(b)) => a.partial_cmp(b),
            (Self::List(a), Self::List(b)) => a.partial_cmp(b),
            (Self::List(a), Self::Num(b)) => a.partial_cmp(&vec![Box::new(Self::Num(*b))]),
            (Self::Num(a), Self::List(b)) => vec![Box::new(Self::Num(*a))].partial_cmp(b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_packet() {
        let packet = "[1,[25,6,[7]]]";
        let expected = Packet::List(vec![
            Box::new(Packet::Num(1)),
            Box::new(Packet::List(vec![
                Box::new(Packet::Num(25)),
                Box::new(Packet::Num(6)),
                Box::new(Packet::List(vec![Box::new(Packet::Num(7))])),
            ])),
        ]);

        assert_eq!(Packet::parse(packet), expected);
    }

    #[test]
    fn can_parse_only_list_packet() {
        let packet = "[[[]]]";
        let expected = Packet::List(vec![Box::new(Packet::List(vec![Box::new(Packet::List(
            Vec::new(),
        ))]))]);

        assert_eq!(Packet::parse(packet), expected);
    }

    #[test]
    fn can_compare_packets() {
        let p1 = Packet::parse("[1,1,3,1,1]");
        let p2 = Packet::parse("[1,1,5,1,1]");
        assert!(p1 < p2);

        let p1 = Packet::parse("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        let p2 = Packet::parse("[1,[2,[3,[4,[5,6,0]]]],8,9]");
        assert!(p1 > p2);
    }
}
