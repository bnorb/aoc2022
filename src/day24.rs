mod map;

use self::map::Map;

#[aoc(day24, part1)]
fn traverse(input: &str) -> usize {
    let mut map = Map::parse(input);

    map.traverse((0, 1), (map.height() - 1, map.width() - 2));

    map.steps_taken()
}

#[aoc(day24, part2)]
fn imma_kill_that_elf(input: &str) -> usize {
    let mut map = Map::parse(input);
    let start = (0, 1);
    let end = (map.height() - 1, map.width() - 2);

    map.traverse(start, end);
    map.traverse(end, start);
    map.traverse(start, end);

    map.steps_taken()
}
