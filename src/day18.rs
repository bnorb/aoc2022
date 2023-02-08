mod cube;

use self::cube::{Bounds, Cube, CubeGenerator, Side};
use std::collections::{HashSet, LinkedList};

#[aoc_generator(day18)]
fn input_generator(input: &str) -> Vec<Cube> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<u8> = line.split(',').map(|part| part.parse().unwrap()).collect();
            Cube::new(parts[0], parts[1], parts[2])
        })
        .collect()
}

fn get_uncovered_sides(cubes: &Vec<Cube>) -> HashSet<Side> {
    let mut uncovered = HashSet::new();

    cubes.iter().for_each(|cube| {
        let sides = cube.sides();
        for side in sides {
            if uncovered.contains(&side) {
                uncovered.remove(&side);
            } else {
                uncovered.insert(side);
            }
        }
    });

    uncovered
}

#[aoc(day18, part1)]
fn count_all(cubes: &Vec<Cube>) -> usize {
    get_uncovered_sides(cubes).len()
}

fn fill_cubes(
    start: Cube,
    visited: &mut HashSet<Cube>,
    all_uncovered: &HashSet<Side>,
    inner_uncovered: &mut HashSet<Side>,
    bounds: &Bounds,
) {
    if visited.contains(&start) {
        return;
    }

    visited.insert(start);
    let mut queue = LinkedList::from([start]);
    let mut sides = HashSet::new();
    let mut inner = true;

    while let Some(current) = queue.pop_front() {
        sides.extend(
            current
                .sides()
                .into_iter()
                .filter(|side| all_uncovered.contains(side)),
        );

        let neighbors: Vec<Cube> = current
            .neighbors(all_uncovered)
            .into_iter()
            .filter(|cube| !visited.contains(cube))
            .collect();

        let all_neighbor_count = neighbors.len();

        let neighbors: Vec<Cube> = neighbors
            .into_iter()
            .filter(|cube| cube.in_bounds(bounds))
            .collect();

        if neighbors.len() < all_neighbor_count {
            inner = false;
        }

        for cube in neighbors {
            visited.insert(cube);
            queue.push_back(cube);
        }
    }

    if inner {
        inner_uncovered.extend(sides.into_iter());
    }
}

#[aoc(day18, part2)]
fn count_outside(cubes: &Vec<Cube>) -> usize {
    let all_uncovered = get_uncovered_sides(cubes);
    let bounds = cubes.iter().fold(
        ((255, 0), (255, 0), (255, 0)),
        |((x_min, x_max), (y_min, y_max), (z_min, z_max)), cube| {
            let (x, y, z) = cube.coords();
            (
                (x_min.min(x), x_max.max(x)),
                (y_min.min(y), y_max.max(y)),
                (z_min.min(z), z_max.max(z)),
            )
        },
    );

    let mut visited_cubes = HashSet::new();
    for cube in cubes {
        visited_cubes.insert(*cube);
    }

    let mut inner_sides = HashSet::new();
    let mut generator = CubeGenerator::new(bounds);
    while let Some(cube) = generator.next() {
        fill_cubes(
            cube,
            &mut visited_cubes,
            &all_uncovered,
            &mut inner_sides,
            &bounds,
        );
    }

    all_uncovered.len() - inner_sides.len()
}
