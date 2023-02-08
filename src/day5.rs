mod stacks;

use self::stacks::{Move, Stacks};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Stacks, Vec<Move>) {
    let mut parts = input.split("\n\n");
    let stacks = Stacks::parse(parts.next().unwrap());

    let moves = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| Move::parse(line))
        .collect();

    (stacks, moves)
}

#[aoc(day5, part1)]
pub fn move_boxes((initial_stacks, moves): &(Stacks, Vec<Move>)) -> String {
    let mut stacks = initial_stacks.clone();
    moves.iter().for_each(|mv| stacks.make_move(mv));
    stacks.top()
}

#[aoc(day5, part2)]
pub fn move_boxes_multi((initial_stacks, moves): &(Stacks, Vec<Move>)) -> String {
    let mut stacks = initial_stacks.clone();
    moves.iter().for_each(|mv| stacks.make_move_at_once(mv));
    stacks.top()
}