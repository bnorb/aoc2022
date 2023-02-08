use std::collections::{HashSet, LinkedList};

fn find_distinct_n(input: &str, n: usize) -> usize {
    let mut iter = input.chars().enumerate();
    let mut buffer = LinkedList::new();
    let mut set = HashSet::new();

    while let Some((i, c)) = iter.next() {
        buffer.push_back(c);
        if buffer.len() == n {
            set.clear();
            if buffer.iter().fold(true, |res, c| set.insert(*c) && res) {
                return i + 1;
            }
            buffer.pop_front();
        }
    }

    panic!("oof");
}

#[aoc(day6, part1)]
pub fn find_start(input: &str) -> usize {
    find_distinct_n(input, 4)
}

#[aoc(day6, part2)]
pub fn find_message(input: &str) -> usize {
    find_distinct_n(input, 14)
}
