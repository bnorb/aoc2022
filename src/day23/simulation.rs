use std::{
    collections::{HashMap, HashSet, LinkedList},
    ops::Add,
};

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    S,
    W,
    E,
    NE,
    NW,
    SE,
    SW,
}

impl Direction {
    fn val(&self) -> (isize, isize) {
        let r = match self {
            Self::N | Self::NW | Self::NE => -1,
            Self::S | Self::SE | Self::SW => 1,
            _ => 0,
        };
        let c = match self {
            Self::W | Self::NW | Self::SW => -1,
            Self::E | Self::NE | Self::SE => 1,
            _ => 0,
        };

        (r, c)
    }

    fn adj(&self) -> (Self, Self) {
        match self {
            Self::N => (Self::NW, Self::NE),
            Self::S => (Self::SW, Self::SE),
            Self::W => (Self::SW, Self::NW),
            Self::E => (Self::NE, Self::SE),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Elf(isize, isize);

impl Elf {
    fn propose(&self, other_elves: &HashSet<Elf>, order: &LinkedList<Direction>) -> Option<Elf> {
        let valid: Vec<&Direction> = order
            .into_iter()
            .filter(|dir| {
                let (left, right) = dir.adj();
                let dirs = [dir, &left, &right];
                dirs.iter().all(|d| {
                    let (dr, dc) = d.val();
                    !other_elves.contains(&(self + &Elf(dr, dc)))
                })
            })
            .collect();

        if valid.len() == 4 || valid.len() == 0 {
            return None;
        }

        let (dr, dc) = valid[0].val();
        Some(self + &Elf(dr, dc))
    }
}

impl Add for &Elf {
    type Output = Elf;

    fn add(self, rhs: Self) -> Self::Output {
        Elf(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub struct Simulation {
    elves: HashSet<Elf>,
    order: LinkedList<Direction>,
}

impl Simulation {
    pub fn parse(input: &str) -> Self {
        let elves = input
            .lines()
            .enumerate()
            .flat_map(|(r, row)| {
                row.chars()
                    .enumerate()
                    .filter_map(move |(c, char)| match char {
                        '#' => Some(Elf(r as isize, c as isize)),
                        _ => None,
                    })
            })
            .collect();
        Self {
            elves,
            order: LinkedList::from([Direction::N, Direction::S, Direction::W, Direction::E]),
        }
    }

    pub fn sim_round(&mut self) -> bool {
        let mut proposals: HashMap<Elf, (Elf, bool)> = HashMap::new();

        self.elves.iter().for_each(|elf| {
            if let Some(proposal) = elf.propose(&self.elves, &self.order) {
                if let Some(curr) = proposals.get_mut(&proposal) {
                    curr.1 = false;
                } else {
                    proposals.insert(proposal, (*elf, true));
                }
            }
        });

        let mut moved = false;

        proposals.into_iter().for_each(|(proposal, (elf, ok))| {
            if ok {
                moved = true;
                self.elves.remove(&elf);
                self.elves.insert(proposal);
            }
        });

        let f = self.order.pop_front().unwrap();
        self.order.push_back(f);

        moved
    }

    pub fn bounds(&self) -> (isize, isize, isize, isize) {
        self.elves.iter().fold(
            (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
            |bounds, elf| {
                (
                    bounds.0.min(elf.0),
                    bounds.1.max(elf.0),
                    bounds.2.min(elf.1),
                    bounds.3.max(elf.1),
                )
            },
        )
    }

    pub fn elf_count(&self) -> usize {
        self.elves.len()
    }
}
