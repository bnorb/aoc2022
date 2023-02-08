use std::{collections::LinkedList, rc::Rc};

pub struct Monkey {
    items: LinkedList<usize>,
    inspected: usize,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> usize>,
    reduce_worry: Rc<dyn Fn(usize) -> usize>,
    divisor: usize,
}

impl Monkey {
    pub fn parse(monkey_string: &str, reduce_worry: Rc<dyn Fn(usize) -> usize>) -> Self {
        let mut iter = monkey_string.lines();
        iter.next();

        let items = iter.next().unwrap().trim()[16..]
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();

        let mut operation = iter.next().unwrap().trim()[21..].split(' ');
        let operand = String::from(operation.next().unwrap());
        let rhs = String::from(operation.next().unwrap());

        let operation = Box::new(move |old: usize| {
            let rhs = match rhs.as_str() {
                "old" => old,
                num => num.parse().unwrap(),
            };

            match operand.as_str() {
                "+" => old + rhs,
                "*" => old * rhs,
                _ => unreachable!(),
            }
        });

        let divisor: usize = iter.next().unwrap().trim()[19..].parse().unwrap();
        let t = iter.next().unwrap().trim()[25..].parse().unwrap();
        let f = iter.next().unwrap().trim()[26..].parse().unwrap();

        let test = Box::new(move |item: usize| if item % divisor == 0 { t } else { f });

        Self {
            items,
            inspected: 0,
            operation,
            test,
            reduce_worry,
            divisor,
        }
    }

    pub fn throw_items(&mut self) -> LinkedList<(usize, usize)> {
        let mut res = LinkedList::new();
        self.inspected += self.items.len();

        while let Some(mut item) = self.items.pop_front() {
            item = (self.operation)(item);
            item = (self.reduce_worry)(item);
            res.push_back((item, (self.test)(item)));
        }

        res
    }

    pub fn catch_item(&mut self, item: usize) {
        self.items.push_back(item);
    }

    pub fn get_inspected(&self) -> usize {
        self.inspected
    }

    pub fn get_divisor(&self) -> usize {
        self.divisor
    }

    pub fn set_worry_reducer(&mut self, reducer: Rc<dyn Fn(usize) -> usize>) {
        self.reduce_worry = reducer
    }
}