mod sensor;

use self::sensor::{Point, Sensor};
use std::cmp::Ordering;

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let position = Point::parse(&parts.next().unwrap()[10..]);
            let closest_beacon = Point::parse(&parts.next().unwrap()[21..]);
            let beacon_distance = &position - &closest_beacon;

            Sensor::new(position, closest_beacon, beacon_distance)
        })
        .collect()
}

fn find_combined_coverage(
    sensors: &Vec<Sensor>,
    row: i32,
    min: Option<i32>,
    max: Option<i32>,
    include_beacons: bool,
) -> Vec<(i32, i32)> {
    let mut ranges: Vec<(i32, i32)> = sensors
        .iter()
        .filter_map(|sensor| sensor.find_row_coverage(row, min, max, include_beacons))
        .collect();

    ranges.sort_by(|a, b| match b.0.cmp(&a.0) {
        Ordering::Equal => b.1.cmp(&a.1),
        other => other,
    });

    let mut combined = Vec::new();
    let mut current = ranges.pop().unwrap();

    while let Some(range) = ranges.pop() {
        if range.1 <= current.1 {
            continue;
        }

        if range.0 <= current.1 {
            current.1 = range.1;
        } else {
            combined.push(current);
            current = range;
        }
    }
    combined.push(current);

    combined
}

#[aoc(day15, part1)]
fn check_row(sensors: &Vec<Sensor>) -> u32 {
    find_combined_coverage(sensors, 2000000, None, None, false)
        .into_iter()
        .fold(0, |sum, (min, max)| sum + (max - min) as u32 + 1)
}

#[aoc(day15, part2)]
fn find_beacon(sensors: &Vec<Sensor>) -> u64 {
    for row in 0..=4000000 {
        let coverage = find_combined_coverage(sensors, row, Some(0), Some(4000000), true);

        if coverage.len() > 1 {
            return (coverage[0].1 + 1) as u64 * 4000000 + row as u64;
        } else if coverage[0].0 == 1 {
            return row as u64;
        } else if coverage[0].1 == 3999999 {
            return 4000000 * 4000000 + row as u64;
        }
    }

    unreachable!()
}
