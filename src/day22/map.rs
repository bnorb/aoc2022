use super::cube::CubeNet;

#[derive(Clone)]
pub enum Tile {
    Void,
    Floor,
    Wall,
}

impl Tile {
    pub fn parse(c: char) -> Self {
        match c {
            ' ' => Self::Void,
            '.' => Self::Floor,
            '#' => Self::Wall,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn new(val: usize) -> Self {
        match val {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => unimplemented!(),
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Self::Right => (0, 1),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Up => (-1, 0),
        }
    }

    pub fn val(&self) -> usize {
        match self {
            Self::Right => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Up => 3,
        }
    }

    fn turn(&self, turn: &Turn) -> Self {
        match (self, turn) {
            (Self::Right, Turn::Left) => Self::Up,
            (Self::Down, Turn::Left) => Self::Right,
            (Self::Left, Turn::Left) => Self::Down,
            (Self::Up, Turn::Left) => Self::Left,

            (Self::Right, Turn::Right) => Self::Down,
            (Self::Down, Turn::Right) => Self::Left,
            (Self::Left, Turn::Right) => Self::Up,
            (Self::Up, Turn::Right) => Self::Right,

            _ => self.clone(),
        }
    }
}

pub enum Turn {
    Left,
    Right,
    Stay,
}

impl Turn {
    pub fn parse(c: &str) -> Option<Self> {
        match c {
            "L" => Some(Self::Left),
            "R" => Some(Self::Right),
            "S" => Some(Self::Stay),
            _ => None,
        }
    }
}

pub type Instruction = (u8, Turn);
type Position = (usize, usize, Direction);

pub struct Map(Vec<Vec<Tile>>);

impl Map {
    pub fn new(map: Vec<Vec<Tile>>) -> Self {
        Self(map)
    }

    fn row(&self, row: usize, col: usize, d: isize) -> usize {
        if d == 0 {
            return row;
        }

        let mut next_row = row as isize + d;
        if next_row >= self.0.len() as isize {
            next_row = 0;
        } else if next_row < 0 {
            next_row = self.0.len() as isize - 1;
        }

        if matches!(self.0[next_row as usize][col], Tile::Void) {
            return self.row(next_row as usize, col, d);
        }

        next_row as usize
    }

    fn col(&self, row: usize, col: usize, d: isize) -> usize {
        if d == 0 {
            return col;
        }

        let mut next_col = col as isize + d;
        if next_col >= self.0[row].len() as isize {
            next_col = 0;
        } else if next_col < 0 {
            next_col = self.0[row].len() as isize - 1;
        }

        if matches!(self.0[row][next_col as usize], Tile::Void) {
            return self.col(row, next_col as usize, d);
        }

        next_col as usize
    }

    pub fn make_move(&self, start: Position, inst: &Instruction) -> Position {
        let mut current = (start.0, start.1);
        let delta = start.2.delta();

        for _ in 0..inst.0 {
            let next = (
                self.row(current.0, current.1, delta.0),
                self.col(current.0, current.1, delta.1),
            );

            if let Tile::Wall = self.0[next.0][next.1] {
                break;
            }

            current = next;
        }

        (current.0, current.1, start.2.turn(&inst.1))
    }

    pub fn find_start(&self) -> Position {
        (0, self.col(0, 0, 1), Direction::Right)
    }

    fn faces(&self, side_len: usize) -> (Vec<Vec<i8>>, [(usize, usize); 6]) {
        let mut face_id = 0;
        let mut grid = Vec::new();
        let mut corners = [(0, 0); 6];

        for r in (0..self.0.len()).step_by(side_len) {
            let mut row = Vec::new();
            for c in (0..self.0[0].len()).step_by(side_len) {
                match self.0[r][c] {
                    Tile::Void => row.push(-1),
                    _ => {
                        row.push(face_id);
                        corners[face_id as usize] = (r, c);
                        face_id += 1
                    }
                }
            }
            grid.push(row);
        }

        (grid, corners)
    }
}

pub struct CubeMap {
    map: Vec<Vec<Tile>>,
    face_corners: [(usize, usize); 6],
    edge_map: [[(usize, usize, bool); 4]; 6],
    side_len: usize,
}

impl CubeMap {
    pub fn from_map(map: &Map, side_len: usize) -> Self {
        let (grid, face_corners) = map.faces(side_len);

        let mut cube_net = CubeNet::from_grid(grid, side_len as f64);
        cube_net.fold();
        let edge_map = cube_net.edge_map();

        Self {
            map: map.0.clone(),
            face_corners,
            edge_map,
            side_len,
        }
    }

    fn in_face(&self, row: usize, col: usize, face: usize) -> bool {
        let (fr, fc) = self.face_corners[face];
        row >= fr && col >= fc && row < fr + self.side_len && col < fc + self.side_len
    }

    fn find_face(&self, row: usize, col: usize) -> usize {
        self.face_corners
            .iter()
            .enumerate()
            .find(|(id, _)| self.in_face(row, col, *id))
            .unwrap()
            .0
    }

    fn edge_offset(
        &self,
        row: usize,
        col: usize,
        dir: Direction,
        face: usize,
        same_orientation: bool,
    ) -> usize {
        let (fr, fc) = self.face_corners[face];
        let (fr_max, fc_max) = (fr + self.side_len - 1, fc + self.side_len - 1);

        match (dir, same_orientation) {
            (Direction::Right | Direction::Left, true) => row - fr,
            (Direction::Right | Direction::Left, false) => fr_max - row,
            (Direction::Down | Direction::Up, true) => col - fc,
            (Direction::Down | Direction::Up, false) => fc_max - col,
        }
    }

    fn change_face(
        &self,
        row: usize,
        col: usize,
        dir: Direction,
        face: usize,
    ) -> ((usize, usize), Direction, usize) {
        let (new_face, new_dir, same_orientation) = self.edge_map[face][dir.val()];
        let new_dir = Direction::new(new_dir);
        let (fr, fc) = self.face_corners[new_face];
        let (fr_max, fc_max) = (fr + self.side_len - 1, fc + self.side_len - 1);

        let offset = self.edge_offset(row, col, dir, face, same_orientation);

        (
            match new_dir {
                Direction::Right => (fr + offset, fc),
                Direction::Down => (fr, fc + offset),
                Direction::Left => (fr + offset, fc_max),
                Direction::Up => (fr_max, fc + offset),
            },
            new_dir,
            new_face,
        )
    }

    pub fn make_move(&self, start: Position, inst: &Instruction) -> Position {
        let mut current = (start.0, start.1);
        let mut dir = start.2;
        let mut face = self.find_face(start.0, start.1);

        for _ in 0..inst.0 {
            let delta = dir.delta();

            let mut next = (
                (current.0 as isize + delta.0) as usize,
                (current.1 as isize + delta.1) as usize,
            );
            let mut next_dir = dir;
            let mut next_face = face;

            if !self.in_face(next.0, next.1, face) {
                (next, next_dir, next_face) = self.change_face(current.0, current.1, dir, face);
            }

            if let Tile::Wall = self.map[next.0][next.1] {
                break;
            }

            current = next;
            dir = next_dir;
            face = next_face;
        }

        (current.0, current.1, dir.turn(&inst.1))
    }
}
