use std::collections::HashSet;

fn calc_priority(item: char) -> u32 {
    if item == item.to_ascii_lowercase() {
        item as u32 - 'a' as u32 + 1
    } else {
        item as u32 - 'A' as u32 + 27
    }
}

#[aoc_generator(day3, part1)]
pub fn part1_generator(input: &str) -> Vec<(HashSet<char>, HashSet<char>)> {
    input
        .lines()
        .map(|l| {
            let item_count = l.len();
            let first_pocket: HashSet<char> = l[..item_count / 2].chars().collect();
            let second_pocket: HashSet<char> = l[item_count / 2..].chars().collect();

            (first_pocket, second_pocket)
        })
        .collect()
}

#[aoc_generator(day3, part2)]
pub fn part2_generator(input: &str) -> Vec<HashSet<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[aoc(day3, part1)]
pub fn common_item(input: &Vec<(HashSet<char>, HashSet<char>)>) -> u32 {
    input
        .iter()
        .map(|(first_pocket, second_pocket)| {
            let item = first_pocket
                .iter()
                .find(|item| second_pocket.contains(item))
                .unwrap();

            calc_priority(*item)
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn badges(input: &Vec<HashSet<char>>) -> u32 {
    let mut iter = input.iter();
    let mut total = 0;

    while let Some(first_elf) = iter.next() {
        let other_elves = (iter.next().unwrap(), iter.next().unwrap());
        let item = first_elf
            .iter()
            .find(|item| other_elves.0.contains(item) && other_elves.1.contains(item))
            .unwrap();

        total += calc_priority(*item);
    }

    total
}
