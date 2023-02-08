mod rope;

use self::rope::{Direction, Move, Point, Rope};

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| (Direction::parse(&line[0..1]), line[2..].parse().unwrap()))
        .collect()
}

#[aoc(day9, part1)]
fn short_rope(input: &Vec<Move>) -> usize {
    let mut rope = Rope::new(vec![Point::new(0, 0); 2]);
    input.iter().for_each(|mv| rope.make_move(mv));
    rope.get_tail_history_count()
}

#[aoc(day9, part2)]
fn long_rope(input: &Vec<Move>) -> usize {
    let mut rope = Rope::new(vec![Point::new(0, 0); 10]);
    input.iter().for_each(|mv| rope.make_move(mv));
    rope.get_tail_history_count()
}
