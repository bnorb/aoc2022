mod range;

use self::range::Range;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(',');
            (
                Range::parse(parts.next().unwrap()),
                Range::parse(parts.next().unwrap()),
            )
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn contains(input: &Vec<(Range, Range)>) -> usize {
    input
        .iter()
        .filter(|(first_range, second_range)| {
            first_range.contains(second_range) || second_range.contains(first_range)
        })
        .count()
}

#[aoc(day4, part2)]
pub fn overlaps(input: &Vec<(Range, Range)>) -> usize {
    input
        .iter()
        .filter(|(first_range, second_range)| first_range.overlaps(second_range))
        .count()
}
