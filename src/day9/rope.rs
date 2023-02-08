use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}

impl Direction {
    pub fn parse(input: &str) -> Self {
        match input {
            "L" => Self::Left,
            "R" => Self::Right,
            "D" => Self::Down,
            "U" => Self::Up,
            _ => panic!("invalid direction"),
        }
    }
}

pub type Move = (Direction, u8);

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn next_in_dir(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Left => self + &Point::new(-1, 0),
            Direction::Right => self + &Point::new(1, 0),
            Direction::Down => self + &Point::new(0, -1),
            Direction::Up => self + &Point::new(0, 1),
        }
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub struct Rope {
    parts: Vec<Point>,
    tail_history: HashSet<Point>,
}

impl Rope {
    pub fn new(parts: Vec<Point>) -> Self {
        assert!(parts.len() >= 2);
        let tail_pos = *parts.last().unwrap();
        Self {
            parts,
            tail_history: HashSet::from([tail_pos]),
        }
    }

    pub fn get_tail_history_count(&self) -> usize {
        self.tail_history.len()
    }

    fn move_part(&mut self, part: usize, direction: &Direction) -> Option<usize> {
        let current = self.parts.get(part).unwrap();
        let next_pos = match self.parts.get(part - 1) {
            Some(last) => {
                let mut diff = last - current;
                if diff.x.abs() <= 1 && diff.y.abs() <= 1 {
                    return None;
                }

                diff.x = diff.x.checked_div(diff.x.abs()).unwrap_or(0);
                diff.y = diff.y.checked_div(diff.y.abs()).unwrap_or(0);

                Some(current + &diff)
            }
            None => Some(current.next_in_dir(direction)),
        };

        if let Some(next) = next_pos {
            let curr = self.parts.get_mut(part).unwrap();
            *curr = next;

            return if part == self.parts.len() - 1 {
                self.tail_history.insert(*self.parts.last().unwrap());
                None
            } else {
                Some(part + 1)
            };
        }

        None
    }

    pub fn make_move(&mut self, mv: &Move) {
        let mut i = 0;
        while i < mv.1 {
            let mut current = 0;
            while let Some(next) = self.move_part(current, &mv.0) {
                current = next
            }

            i += 1;
        }
    }
}
