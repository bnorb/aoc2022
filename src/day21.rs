mod monkeymap;

use self::monkeymap::{Equation, MonkeyMap};

#[aoc(day21, part1)]
fn find_root(input: &str) -> i64 {
    let monkeys = MonkeyMap::parse(input);
    monkeys.get_val("root").unwrap()
}

#[aoc(day21, part2)]
fn find_humn(input: &str) -> i64 {
    let mut monkeys = MonkeyMap::parse(input);
    monkeys.correct();
    let (unknown, val) = monkeys.calc_half();

    let eq = monkeys
        .build_humn_equation(unknown.as_str())
        .substitute_var(unknown.as_str(), &Equation::Num(val))
        .unwrap();

    eq.calc()
}
