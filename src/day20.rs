mod circle;

use self::circle::CircularList;

#[aoc_generator(day20, part1)]
fn input_generator(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc_generator(day20, part2)]
fn input_generator_2(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.parse::<i64>().unwrap() * 811589153)
        .collect()
}

#[aoc(day20, part1)]
fn get_sum(nums: &Vec<i64>) -> i64 {
    let circle = CircularList::from(nums);
    circle.move_all();

    let coords: Vec<i64> = [1000, 2000, 3000]
        .into_iter()
        .map(|coord| circle.find_coord(coord))
        .collect();
    coords.into_iter().fold(0, |sum, val| sum + val)
}

#[aoc(day20, part2)]
fn get_big_sum(nums: &Vec<i64>) -> i64 {
    let circle = CircularList::from(nums);

    for _ in 0..10 {
        circle.move_all();
    }

    let coords: Vec<i64> = [1000, 2000, 3000]
        .into_iter()
        .map(|coord| circle.find_coord(coord))
        .collect();
    coords.into_iter().fold(0, |sum, val| sum + val)
}
