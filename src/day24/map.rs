use std::collections::{HashMap, HashSet, LinkedList};

#[derive(Debug, PartialEq)]
enum Tile {
    Ground,
    BlizPortal(usize, usize),
}

#[derive(Debug, Clone)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    fn parse(c: char) -> Self {
        match c {
            '>' => Direction::E,
            '<' => Direction::W,
            'v' => Direction::S,
            '^' => Direction::N,
            _ => unimplemented!(),
        }
    }

    fn val(&self) -> (isize, isize) {
        match self {
            Self::E => (0, 1),
            Self::W => (0, -1),
            Self::S => (1, 0),
            Self::N => (-1, 0),
        }
    }
}

type BlizzardMap = HashMap<(usize, usize), Vec<Direction>>;

#[derive(PartialEq, Eq, Debug)]
struct Node {
    pos: (usize, usize),
    f: usize,
    g: usize,
    h: usize,
    steps: usize,
    blizz_id: usize,
}

#[derive(Debug)]
pub struct Map {
    map: Vec<Vec<Tile>>,
    blizzard_memo: HashMap<usize, BlizzardMap>,
    blizzard_id: usize,
    blizzard_loop: usize,
    steps_taken: usize,
}

impl Map {
    pub fn parse(input: &str) -> Self {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().chars().count();

        let map: Vec<Vec<Tile>> = input
            .lines()
            .enumerate()
            .map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .map(|(c, char)| match char {
                        '#' if r == 0 => Tile::BlizPortal(height - 2, c),
                        '#' if r == height - 1 => Tile::BlizPortal(1, c),
                        '#' if c == 0 => Tile::BlizPortal(r, width - 2),
                        '#' if c == width - 1 => Tile::BlizPortal(r, 1),
                        '#' => unimplemented!(),
                        _ => Tile::Ground,
                    })
                    .collect()
            })
            .collect();

        let blizzards: BlizzardMap = input
            .lines()
            .enumerate()
            .flat_map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(c, char)| match char {
                        '#' | '.' => None,
                        char => Some(((r, c), vec![Direction::parse(char)])),
                    })
                    .collect::<Vec<((usize, usize), Vec<Direction>)>>()
            })
            .collect();

        let blizzard_loop = lcm(map.len() - 2, map[0].len() - 2);

        Self {
            map,
            blizzard_memo: HashMap::from([(0, blizzards)]),
            blizzard_id: 0,
            blizzard_loop,
            steps_taken: 0,
        }
    }

    pub fn width(&self) -> usize {
        self.map[0].len()
    }

    pub fn height(&self) -> usize {
        self.map.len()
    }

    pub fn steps_taken(&self) -> usize {
        self.steps_taken
    }

    fn calc_blizzard(&mut self, id: usize) {
        let prev = self.blizzard_memo.get(&(id - 1)).unwrap();
        let mut new_bliz: BlizzardMap = HashMap::new();

        prev.into_iter().for_each(|((r, c), dirs)| {
            dirs.into_iter().for_each(|dir| {
                let d = dir.val();
                let nr = (*r as isize + d.0) as usize;
                let nc = (*c as isize + d.1) as usize;

                let (nr, nc) = match self.map[nr][nc] {
                    Tile::Ground => (nr, nc),
                    Tile::BlizPortal(nr, nc) => (nr, nc),
                };

                if let Some(v) = new_bliz.get_mut(&(nr, nc)) {
                    v.push(dir.clone());
                } else {
                    new_bliz.insert((nr, nc), vec![dir.clone()]);
                }
            });
        });

        self.blizzard_memo.insert(id, new_bliz);
    }

    fn in_blizzard(&mut self, coords: &(usize, usize), id: usize) -> bool {
        if !self.blizzard_memo.contains_key(&id) {
            self.calc_blizzard(id);
        }

        self.blizzard_memo.get(&id).unwrap().contains_key(coords)
    }

    fn next_tiles(&mut self, (r, c): (usize, usize), blizz_id: usize) -> Vec<(usize, usize)> {
        [(r + 1, c), (r, c + 1), (r - 1, c), (r, c - 1), (r, c)]
            .into_iter()
            .filter(|coords| {
                coords.0 < self.map.len()
                    && coords.1 < self.map[0].len()
                    && self.map[coords.0][coords.1] == Tile::Ground
                    && !self.in_blizzard(coords, blizz_id)
            })
            .collect()
    }

    pub fn traverse(&mut self, from: (usize, usize), to: (usize, usize)) {
        let mut states = HashSet::from([(from, self.blizzard_id)]);
        let mut queue = LinkedList::from([(from, self.steps_taken, self.blizzard_id)]);

        while let Some((current, steps, blizz_id)) = queue.pop_front() {
            if current == to {
                self.steps_taken = steps;
                self.blizzard_id = blizz_id;

                return;
            }

            let blizz_id = (blizz_id + 1) % self.blizzard_loop;
            let steps = steps + 1;

            let next = self.next_tiles(current, blizz_id);
            for coords in next {
                if !states.contains(&(coords, blizz_id)) {
                    states.insert((coords, blizz_id));
                    queue.push_back((coords, steps, blizz_id));
                }
            }
        }

        unreachable!()
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if a == b {
        return a;
    }

    if a > b {
        return gcd(a - b, b);
    }

    gcd(a, b - a)
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
