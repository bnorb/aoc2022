use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Rock {
    Horizontal,
    Cross,
    Corner,
    Vertical,
    Square,
}

impl Rock {
    fn coords(&self) -> Vec<(usize, usize)> {
        match self {
            Rock::Horizontal => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Rock::Cross => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            Rock::Corner => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Rock::Vertical => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Rock::Square => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }

    fn height(&self) -> usize {
        match self {
            Rock::Horizontal => 1,
            Rock::Cross => 3,
            Rock::Corner => 3,
            Rock::Vertical => 4,
            Rock::Square => 2,
        }
    }
}

impl Iterator for Rock {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Rock::Horizontal => Some(Rock::Cross),
            Rock::Cross => Some(Rock::Corner),
            Rock::Corner => Some(Rock::Vertical),
            Rock::Vertical => Some(Rock::Square),
            Rock::Square => Some(Rock::Horizontal),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Wind {
    Left,
    Right,
}

impl Wind {
    pub fn new(wind: char) -> Self {
        match wind {
            '>' => Self::Right,
            '<' => Self::Left,
            _ => unreachable!(),
        }
    }

    fn blow(&self, rock_coordinates: &mut Vec<(usize, usize)>) {
        if rock_coordinates.iter().any(|(x, _)| *x == 0) && *self == Self::Left {
            return;
        }

        if rock_coordinates.iter().any(|(x, _)| *x == 6) && *self == Self::Right {
            return;
        }

        rock_coordinates.iter_mut().for_each(|(x, _)| {
            *x = match *self {
                Wind::Left => *x - 1,
                Wind::Right => *x + 1,
            };
        })
    }
}

pub struct InfiniteStorm<'a> {
    winds: &'a Vec<Wind>,
    next_index: usize,
}

impl<'a> InfiniteStorm<'a> {
    pub fn new(winds: &'a Vec<Wind>) -> Self {
        InfiniteStorm {
            winds,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for InfiniteStorm<'a> {
    type Item = &'a Wind;

    fn next(&mut self) -> Option<Self::Item> {
        match self.winds.get(self.next_index) {
            Some(wind) => {
                self.next_index += 1;
                Some(wind)
            }
            None => {
                self.next_index = 1;
                self.winds.get(0)
            }
        }
    }
}

pub struct Chamber<'a> {
    chamber: Vec<[u8; 7]>,
    top: usize,
    storm: InfiniteStorm<'a>,
}

impl<'a> Display for Chamber<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.chamber.iter().rev() {
            write!(f, "|")?;
            for cell in row.iter() {
                match cell {
                    0 => write!(f, ".")?,
                    1 => write!(f, "#")?,
                    _ => unreachable!(),
                };
            }
            writeln!(f, "|")?;
        }

        write!(f, "+-------+")
    }
}

impl<'a> Chamber<'a> {
    pub fn new(storm: InfiniteStorm<'a>) -> Self {
        Chamber {
            chamber: Vec::new(),
            top: 0,
            storm,
        }
    }

    pub fn get_height(&self) -> usize {
        self.top
    }

    fn spawn_rock(&mut self, rock: &Rock) -> Vec<(usize, usize)> {
        let bottom_left = (2, self.top + 3);

        let rock_coords = rock
            .coords()
            .into_iter()
            .map(|(x, y)| (bottom_left.0 + x, bottom_left.1 + y))
            .collect();

        let new_top = bottom_left.1 + rock.height() - 1;
        if new_top >= self.chamber.len() {
            for _ in self.chamber.len()..=new_top {
                self.chamber.push([0; 7]);
            }
        }

        rock_coords
    }

    fn is_blocked(&self, rock_coords: &Vec<(usize, usize)>) -> bool {
        rock_coords.iter().any(|(x, y)| self.chamber[*y][*x] == 1)
    }

    fn settle(&mut self, rock_coords: Vec<(usize, usize)>) {
        rock_coords.into_iter().for_each(|(x, y)| {
            self.chamber[y][x] = 1;
            self.top = self.top.max(y + 1);
        });
    }

    pub fn get_top(&self) -> [u8; 7] {
        self.chamber.get(self.top - 1).unwrap().clone()
    }

    pub fn get_wind_index(&self) -> usize {
        self.storm.next_index
    }

    pub fn drop_rock(&mut self, rock: &Rock) {
        let mut last_rock_coords = self.spawn_rock(rock);
        let mut wind;

        loop {
            let mut new_rock_coords = last_rock_coords.clone();

            wind = self.storm.next().unwrap();
            wind.blow(&mut new_rock_coords);
            if self.is_blocked(&new_rock_coords) {
                new_rock_coords = last_rock_coords.clone();
            }

            if new_rock_coords.iter().any(|(_, y)| *y == 0) {
                self.settle(new_rock_coords);
                return;
            }

            last_rock_coords = new_rock_coords.clone();

            new_rock_coords.iter_mut().for_each(|(_, y)| *y -= 1);
            if self.is_blocked(&new_rock_coords) {
                self.settle(last_rock_coords);
                return;
            }

            last_rock_coords = new_rock_coords;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_spawn_rock() {
        let winds = Vec::new();
        let mut chamber = Chamber::new(InfiniteStorm::new(&winds));
        chamber.spawn_rock(&Rock::Cross);
        assert_eq!(chamber.chamber.len(), 6);
        assert_eq!(chamber.top, 0);
        chamber.spawn_rock(&Rock::Horizontal);
        assert_eq!(chamber.chamber.len(), 6);
        assert_eq!(chamber.top, 0);

        chamber.top = 8;
        chamber.spawn_rock(&Rock::Horizontal);
        assert_eq!(chamber.chamber.len(), 12);
        assert_eq!(chamber.top, 8);
    }

    #[test]
    fn can_iterate_rocks() {
        let mut rock = Rock::Horizontal;
        assert_eq!(rock, Rock::Horizontal);
        rock = rock.next().unwrap();
        assert_eq!(rock, Rock::Cross);
        rock = rock.next().unwrap();
        assert_eq!(rock, Rock::Corner);
        rock = rock.next().unwrap();
        assert_eq!(rock, Rock::Vertical);
        rock = rock.next().unwrap();
        assert_eq!(rock, Rock::Square);
        rock = rock.next().unwrap();
        assert_eq!(rock, Rock::Horizontal);
        rock = rock.next().unwrap();
        assert_eq!(rock, Rock::Cross);
    }

    #[test]
    fn can_iterate_winds() {
        let winds = vec!['>', '>', '<', '>', '<']
            .into_iter()
            .map(|c| Wind::new(c))
            .collect();
        let mut storm = InfiniteStorm::new(&winds);

        assert_eq!(storm.next(), Some(&Wind::Right));
        assert_eq!(storm.next(), Some(&Wind::Right));
        assert_eq!(storm.next(), Some(&Wind::Left));
        assert_eq!(storm.next(), Some(&Wind::Right));
        assert_eq!(storm.next(), Some(&Wind::Left));
        assert_eq!(storm.next(), Some(&Wind::Right));
        assert_eq!(storm.next(), Some(&Wind::Right));
        assert_eq!(storm.next(), Some(&Wind::Left));
    }
}
