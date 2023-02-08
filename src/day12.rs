use std::collections::{HashSet, LinkedList};

fn find_cell(map: &Vec<Vec<char>>, target: char) -> (usize, usize) {
    map.iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .enumerate()
                .find_map(|(c, cell)| if *cell == target { Some((r, c)) } else { None })
        })
        .unwrap()
}

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn convert_tile(tile: char) -> char {
    match tile {
        'S' => 'a',
        'E' => 'z',
        other => other,
    }
}

fn can_move(from: char, to: char, down: bool) -> bool {
    let from = convert_tile(from);
    let to = convert_tile(to);

    if down {
        from as i8 - to as i8 <= 1
    } else {
        to as i8 - from as i8 <= 1
    }
}

/// Don't run it on grids larger than usize - 1 rows or cols lmao.
/// The usize overflows backwards, so the >= 0 comparison is unneeded.
fn get_next(
    visited: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
    row: usize,
    col: usize,
) -> Vec<(usize, usize)> {
    vec![
        (row - 1, col),
        (row + 1, col),
        (row, col - 1),
        (row, col + 1),
    ]
    .into_iter()
    .filter(|(r, c)| *r < height && *c < width)
    .filter(|point| !visited.contains(point))
    .collect()
}

fn count_shortest(map: &Vec<Vec<char>>, start: char, ends: HashSet<char>, down: bool) -> u16 {
    let start = find_cell(map, start);
    let mut queue = LinkedList::from([(start.0, start.1, 0)]);
    let mut visited = HashSet::from([start]);

    let width = map[0].len();
    let height = map.len();

    while let Some((row, col, steps)) = queue.pop_front() {
        if ends.contains(&map[row][col]) {
            return steps;
        }

        get_next(&visited, width, height, row, col)
            .into_iter()
            .filter(|(r, c)| can_move(map[row][col], map[*r][*c], down))
            .for_each(|(r, c)| {
                visited.insert((r, c));
                queue.push_back((r, c, steps + 1));
            });
    }

    unreachable!()
}

#[aoc(day12, part1)]
fn start_to_end(map: &Vec<Vec<char>>) -> u16 {
    count_shortest(map, 'S', HashSet::from(['E']), false)
}

#[aoc(day12, part2)]
fn end_to_a(map: &Vec<Vec<char>>) -> u16 {
    count_shortest(map, 'E', HashSet::from(['S', 'a']), true)
}
