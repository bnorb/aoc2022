mod monkey;

use self::monkey::Monkey;
use std::{cell::RefCell, rc::Rc};

fn parse_monkeys(input: &str, reduce_worry: Rc<dyn Fn(usize) -> usize>) -> Vec<RefCell<Monkey>> {
    input
        .split("\n\n")
        .map(|monkey_str| RefCell::new(Monkey::parse(monkey_str, Rc::clone(&reduce_worry))))
        .collect()
}

fn simulate(monkeys: &Vec<RefCell<Monkey>>, rounds: u16) {
    for _ in 1..=rounds {
        monkeys.iter().for_each(|monkey| {
            let mut items = monkey.borrow_mut().throw_items();
            while let Some((item, m)) = items.pop_front() {
                monkeys.get(m).unwrap().borrow_mut().catch_item(item);
            }
        })
    }
}

fn find_two_best(monkeys: &Vec<RefCell<Monkey>>) -> (usize, usize) {
    monkeys.iter().fold((0, 0), |mut maxes, monkey| {
        let inspected = monkey.borrow().get_inspected();
        if inspected > maxes.0 {
            maxes.1 = maxes.0;
            maxes.0 = inspected;
        } else if inspected > maxes.1 {
            maxes.1 = inspected;
        }

        maxes
    })
}

#[aoc(day11, part1)]
fn chill_sim(input: &str) -> usize {
    let monkeys = parse_monkeys(input, Rc::new(|item: usize| item / 3));
    simulate(&monkeys, 20);
    let two_best = find_two_best(&monkeys);

    two_best.0 * two_best.1
}

#[aoc(day11, part2)]
fn anxious_sim(input: &str) -> usize {
    let monkeys = parse_monkeys(input, Rc::new(|item: usize| item));
    let prod = monkeys.iter().fold(1, |p, m| p * m.borrow().get_divisor());
    let reducer = Rc::new(move |item: usize| item % prod);

    monkeys.iter().for_each(|m| {
        let r = Rc::clone(&reducer);
        m.borrow_mut().set_worry_reducer(r);
    });

    simulate(&monkeys, 10000);
    let two_best = find_two_best(&monkeys);

    two_best.0 * two_best.1
}
