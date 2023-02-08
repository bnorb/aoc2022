use std::{collections::HashSet, hash::Hash};

#[derive(PartialEq, Hash, Eq, Debug)]
pub enum SidePlane {
    XY,
    XZ,
    YZ,
}

pub type Side = (u8, u8, u8, SidePlane);
pub type Bounds = ((u8, u8), (u8, u8), (u8, u8));

#[derive(PartialEq, Hash, Eq, Copy, Clone, Debug)]
pub struct Cube {
    x: u8,
    y: u8,
    z: u8,
}

impl Cube {
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        Self { x, y, z }
    }

    pub fn coords(&self) -> (u8, u8, u8) {
        (self.x, self.y, self.z)
    }

    pub fn sides(&self) -> [Side; 6] {
        [
            (self.x, self.y, self.z, SidePlane::XY),
            (self.x, self.y, self.z, SidePlane::XZ),
            (self.x, self.y, self.z, SidePlane::YZ),
            (self.x, self.y, self.z + 1, SidePlane::XY),
            (self.x, self.y + 1, self.z, SidePlane::XZ),
            (self.x + 1, self.y, self.z, SidePlane::YZ),
        ]
    }

    pub fn in_bounds(&self, bounds: &Bounds) -> bool {
        self.x >= (bounds.0).0
            && self.x <= (bounds.0).1
            && self.y >= (bounds.1).0
            && self.y <= (bounds.1).1
            && self.z >= (bounds.2).0
            && self.z <= (bounds.2).1
    }

    pub fn neighbors(&self, uncovered_sides: &HashSet<Side>) -> Vec<Self> {
        let (x, y, z) = (self.x, self.y, self.z);
        let sides = self.sides();
        // order must be same as sides()
        vec![
            Cube::new(x, y, z - 1), // XY
            Cube::new(x, y - 1, z), // XZ
            Cube::new(x - 1, y, z), // YZ
            Cube::new(x, y, z + 1), // XY
            Cube::new(x, y + 1, z), // XZ
            Cube::new(x + 1, y, z), // YZ
        ]
        .into_iter()
        .enumerate()
        .filter(|(i, _)| !uncovered_sides.contains(&sides[*i]))
        .map(|(_, cube)| cube)
        .collect()
    }
}

pub struct CubeGenerator {
    current: Option<Cube>,
    bounds: Bounds,
}

impl CubeGenerator {
    pub fn new(bounds: Bounds) -> Self {
        Self {
            current: None,
            bounds,
        }
    }
}

impl Iterator for CubeGenerator {
    type Item = Cube;

    fn next(&mut self) -> Option<Self::Item> {
        let cube = match self.current {
            None => Some(Cube::new(
                (self.bounds.0).0,
                (self.bounds.1).0,
                (self.bounds.2).0,
            )),
            Some(current) if current.z < (self.bounds.2).1 => {
                Some(Cube::new(current.x, current.y, current.z + 1))
            }
            Some(current) if current.y < (self.bounds.1).1 => {
                Some(Cube::new(current.x, current.y + 1, (self.bounds.2).0))
            }
            Some(current) if current.x < (self.bounds.0).1 => Some(Cube::new(
                current.x + 1,
                (self.bounds.1).0,
                (self.bounds.2).0,
            )),
            _ => None,
        };

        self.current = cube;
        cube
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_generate_cubes() {
        let mut generator = CubeGenerator::new(((1, 2), (1, 2), (1, 2)));
        assert_eq!(generator.next().unwrap(), Cube::new(1, 1, 1));
        assert_eq!(generator.next().unwrap(), Cube::new(1, 1, 2));
        assert_eq!(generator.next().unwrap(), Cube::new(1, 2, 1));
        assert_eq!(generator.next().unwrap(), Cube::new(1, 2, 2));
        assert_eq!(generator.next().unwrap(), Cube::new(2, 1, 1));
        assert_eq!(generator.next().unwrap(), Cube::new(2, 1, 2));
        assert_eq!(generator.next().unwrap(), Cube::new(2, 2, 1));
        assert_eq!(generator.next().unwrap(), Cube::new(2, 2, 2));
        assert_eq!(generator.next(), None);
    }
}
