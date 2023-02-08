use std::{hash::Hash, ops::Sub};

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn parse(point: &str) -> Self {
        let mut parts = point.split(", ");
        Self {
            x: parts.next().unwrap()[2..].parse().unwrap(),
            y: parts.next().unwrap()[2..].parse().unwrap(),
        }
    }
}

/// Manhattan distance
impl Sub for &Point {
    type Output = u32;

    fn sub(self, rhs: Self) -> Self::Output {
        ((self.x - rhs.x).abs() + (self.y - rhs.y).abs()) as u32
    }
}

pub struct Sensor {
    position: Point,
    closest_beacon: Point,
    beacon_distance: u32,
}

impl Sensor {
    pub fn new(position: Point, closest_beacon: Point, beacon_distance: u32) -> Self {
        Sensor {
            position,
            closest_beacon,
            beacon_distance,
        }
    }

    pub fn find_row_coverage(
        &self,
        row: i32,
        min: Option<i32>,
        max: Option<i32>,
        include_beacons: bool,
    ) -> Option<(i32, i32)> {
        let mut row_point = self.position.clone();
        row_point.y = row;

        let row_distance = &row_point - &self.position;
        if row_distance > self.beacon_distance {
            return None;
        }

        if !include_beacons && row_distance == self.beacon_distance {
            return None;
        }

        let x_distance = (self.beacon_distance - row_distance) as i32;
        let mut range = (self.position.x - x_distance, self.position.x + x_distance);

        if !include_beacons && self.closest_beacon.y == row {
            if self.closest_beacon.x == range.0 {
                range.0 += 1;
            } else {
                range.1 -= 1;
            }
        }

        if let Some(min) = min {
            range.0 = min.max(range.0);
        }

        if let Some(max) = max {
            range.1 = max.min(range.1);
        }

        Some(range)
    }
}
