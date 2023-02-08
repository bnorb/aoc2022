use std::{cell::RefCell, cmp, collections::HashSet};

#[aoc_generator(day14)]
fn input_generator(input: &str) -> (RefCell<HashSet<(u16, u16)>>, u16) {
    let mut filled = HashSet::new();
    let mut bottom = 0;

    input
        .lines()
        .map(|line| {
            line.split(" -> ").map(|point| {
                let point: Vec<u16> = point.split(',').map(|num| num.parse().unwrap()).collect();
                (point[0], point[1])
            })
        })
        .for_each(|mut formation| {
            let mut last = formation.next().unwrap();

            if last.1 > bottom {
                bottom = last.1;
            }

            while let Some(current) = formation.next() {
                if last.0 == current.0 {
                    for y in cmp::min(last.1, current.1)..=cmp::max(last.1, current.1) {
                        filled.insert((current.0, y));
                    }
                } else {
                    for x in cmp::min(last.0, current.0)..=cmp::max(last.0, current.0) {
                        filled.insert((x, current.1));
                    }
                }

                if current.1 > bottom {
                    bottom = current.1;
                }

                last = current;
            }
        });

    (RefCell::new(filled), bottom)
}

fn does_settle(
    (mut x, mut y): (u16, u16),
    filled: &RefCell<HashSet<(u16, u16)>>,
    bottom: u16,
) -> bool {
    y += 1;
    if !filled.borrow().contains(&(x, y)) {
        if y >= bottom {
            filled.borrow_mut().insert((x, y - 1));
            return false;
        }

        return does_settle((x, y), filled, bottom);
    }

    x -= 1;
    if !filled.borrow().contains(&(x, y)) {
        return does_settle((x, y), filled, bottom);
    }

    x += 2;
    if !filled.borrow().contains(&(x, y)) {
        return does_settle((x, y), filled, bottom);
    }

    filled.borrow_mut().insert((x - 1, y - 1));

    true
}

#[aoc(day14, part1)]
fn until_falls((filled, bottom): &(RefCell<HashSet<(u16, u16)>>, u16)) -> u16 {
    let mut i = 0;
    while does_settle((500, 0), filled, *bottom) {
        i += 1;
    }

    i
}

#[aoc(day14, part2)]
fn until_clogs((filled, bottom): &(RefCell<HashSet<(u16, u16)>>, u16)) -> u16 {
    let mut i = 0;
    while !filled.borrow().contains(&(500, 0)) {
        does_settle((500, 0), filled, *bottom + 2);
        i += 1;
    }

    i
}
