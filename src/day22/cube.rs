use super::quaternion::Quaternion;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Deref,
    rc::Rc,
};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Axis {
    X,
    Y,
    Z,
    Xi,
    Yi,
    Zi,
}

impl Axis {
    fn from_vec(vec: (f64, f64, f64)) -> Self {
        match vec {
            (x, y, z) if y == 0.0 && z == 0.0 && x > 0.0 => Self::X,
            (x, y, z) if y == 0.0 && z == 0.0 && x < 0.0 => Self::Xi,
            (x, y, z) if x == 0.0 && z == 0.0 && y > 0.0 => Self::Y,
            (x, y, z) if x == 0.0 && z == 0.0 && y < 0.0 => Self::Yi,
            (x, y, z) if x == 0.0 && y == 0.0 && z > 0.0 => Self::Z,
            (x, y, z) if x == 0.0 && y == 0.0 && z < 0.0 => Self::Zi,
            _ => unimplemented!("not an axis vector"),
        }
    }

    fn coords(&self) -> (f64, f64, f64) {
        match self {
            Self::X => (1.0, 0.0, 0.0),
            Self::Y => (0.0, 1.0, 0.0),
            Self::Z => (0.0, 0.0, 1.0),
            Self::Xi => (-1.0, 0.0, 0.0),
            Self::Yi => (0.0, -1.0, 0.0),
            Self::Zi => (0.0, 0.0, -1.0),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Point(f64, f64, f64);

impl Point {
    fn coords(&self) -> (f64, f64, f64) {
        (self.0, self.1, self.2)
    }

    fn sub(&mut self, (x, y, z): (f64, f64, f64)) {
        self.0 -= x;
        self.1 -= y;
        self.2 -= z;
    }

    fn diff(&self, (x, y, z): (f64, f64, f64)) -> (f64, f64, f64) {
        (self.0 - x, self.1 - y, self.2 - z)
    }

    fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    fn set(&mut self, (x, y, z): (f64, f64, f64)) {
        self.0 = x;
        self.1 = y;
        self.2 = z;
    }
}

#[derive(Debug)]
struct PointPointer(Rc<RefCell<Point>>); // lol

impl Deref for PointPointer {
    type Target = RefCell<Point>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl PartialEq for PointPointer {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for PointPointer {}

impl Hash for PointPointer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(Rc::as_ptr(&self.0) as usize);
    }
}

#[derive(Debug, PartialEq)]
struct Edge {
    start: PointPointer,
    end: PointPointer,
    face_id: u8,
}

impl Edge {
    fn new(start: PointPointer, end: PointPointer, face_id: u8) -> Self {
        Self {
            start,
            end,
            face_id,
        }
    }

    fn axis(&self) -> Axis {
        let vec = self.end.borrow().diff(self.start.borrow().coords());
        Axis::from_vec(vec)
    }

    #[cfg(test)]
    fn point_vals(&self) -> [Point; 2] {
        [self.start.borrow().clone(), self.end.borrow().clone()]
    }
}

#[derive(Debug, PartialEq)]
struct Face {
    id: u8,
    nw: PointPointer,
    ne: PointPointer,
    sw: PointPointer,
    se: PointPointer,
    plane: Axis,
    right: Axis,
}

impl Face {
    fn points(&self) -> [&PointPointer; 4] {
        [&self.nw, &self.ne, &self.sw, &self.se]
    }

    fn point_vals(&self) -> [Point; 4] {
        [
            self.nw.borrow().clone(),
            self.ne.borrow().clone(),
            self.sw.borrow().clone(),
            self.se.borrow().clone(),
        ]
    }

    fn count_turns(&self, target: Axis) -> u8 {
        let mut dir = self.right;
        for i in 0..4 {
            if dir == target {
                return i;
            }
            dir = Axis::from_vec(Quaternion::rotate_point(
                dir.coords(),
                self.plane.coords(),
                -90.0,
                0,
            ))
        }

        unreachable!()
    }

    fn edge_axis(&self, edge_index: u8) -> Axis {
        match edge_index {
            0 => Axis::from_vec(self.se.borrow().diff(self.ne.borrow().coords())),
            1 => Axis::from_vec(self.se.borrow().diff(self.sw.borrow().coords())),
            2 => Axis::from_vec(self.sw.borrow().diff(self.nw.borrow().coords())),
            3 => Axis::from_vec(self.ne.borrow().diff(self.nw.borrow().coords())),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct CubeNet {
    faces: HashMap<u8, Face>,
    foldable_edges: Vec<Edge>,
    folded: bool,
}

impl CubeNet {
    pub fn from_grid(grid: Vec<Vec<i8>>, side_len: f64) -> Self {
        let mut foldable_edges = Vec::new();
        let mut faces: HashMap<(usize, usize), Face> = HashMap::new();

        for (r, row) in grid.into_iter().enumerate() {
            let rf = r as f64;

            for (c, face_id) in row.into_iter().enumerate() {
                if face_id < 0 {
                    continue;
                }

                let cf = c as f64;

                let mut nw = Rc::new(RefCell::new(Point::new(side_len * cf, side_len * rf, 0.0)));
                let mut ne = Rc::new(RefCell::new(Point::new(
                    side_len * (cf + 1.0),
                    side_len * rf,
                    0.0,
                )));
                let mut sw = Rc::new(RefCell::new(Point::new(
                    side_len * cf,
                    side_len * (rf + 1.0),
                    0.0,
                )));
                let se = Rc::new(RefCell::new(Point::new(
                    side_len * (cf + 1.0),
                    side_len * (rf + 1.0),
                    0.0,
                )));

                if let Some(face) = faces.get(&(r, c + 2)) {
                    ne = Rc::clone(&face.sw.0);
                }

                let mut foldable = (false, false);
                if let Some(face) = faces.get(&(r, c + 1)) {
                    nw = Rc::clone(&face.sw.0);
                    ne = Rc::clone(&face.se.0);
                    foldable.0 = true;
                }

                if let Some(face) = faces.get(&(r + 1, c)) {
                    nw = Rc::clone(&face.ne.0);
                    sw = Rc::clone(&face.se.0);
                    foldable.1 = true;
                }

                if foldable.0 {
                    foldable_edges.push(Edge::new(
                        PointPointer(Rc::clone(&nw)),
                        PointPointer(Rc::clone(&ne)),
                        face_id as u8,
                    ));
                }

                if foldable.1 {
                    foldable_edges.push(Edge::new(
                        PointPointer(Rc::clone(&nw)),
                        PointPointer(Rc::clone(&sw)),
                        face_id as u8,
                    ));
                }

                faces.insert(
                    (r + 1, c + 1),
                    Face {
                        id: face_id as u8,
                        nw: PointPointer(nw),
                        ne: PointPointer(ne),
                        sw: PointPointer(sw),
                        se: PointPointer(se),
                        plane: Axis::Zi,
                        right: Axis::X,
                    },
                );
            }
        }

        Self {
            faces: faces.into_iter().map(|(_, face)| (face.id, face)).collect(),
            foldable_edges,
            folded: false,
        }
    }

    fn translate(&mut self, d: (f64, f64, f64)) {
        let mut moved = HashSet::new();
        self.faces.iter().for_each(|(_, face)| {
            face.points().into_iter().for_each(|point| {
                if !moved.contains(point) {
                    moved.insert(point);
                    point.borrow_mut().sub(d);
                }
            });
        });
    }

    fn rotate(&mut self, faces: Vec<u8>, axis: Axis) {
        let mut rotated = HashSet::new();
        faces.into_iter().for_each(|face_id| {
            self.faces[&face_id].points().into_iter().for_each(|point| {
                if !rotated.contains(point) {
                    rotated.insert(PointPointer(Rc::clone(&point.0)));

                    let p = point.borrow().clone();
                    point.borrow_mut().set(Quaternion::rotate_point(
                        p.coords(),
                        axis.coords(),
                        90.0,
                        0,
                    ));
                }
            });

            let face = self.faces.get_mut(&face_id).unwrap();
            face.plane = Axis::from_vec(Quaternion::rotate_point(
                face.plane.coords(),
                axis.coords(),
                90.0,
                0,
            ));
            face.right = Axis::from_vec(Quaternion::rotate_point(
                face.right.coords(),
                axis.coords(),
                90.0,
                0,
            ));
        });
    }

    fn find_origo(&self) -> (f64, f64, f64) {
        let find_lesser = |origo: Point, point: Point| {
            if point.0 <= origo.0 && point.1 <= origo.1 && point.2 <= origo.2 {
                return point;
            }

            origo
        };

        self.faces
            .iter()
            .map(|(_, face)| face.point_vals().into_iter().reduce(find_lesser).unwrap())
            .reduce(find_lesser)
            .unwrap()
            .coords()
    }

    fn rotation_filter(edge_axis: Axis, face_plane: Axis) -> fn(&&PointPointer) -> bool {
        match (edge_axis, face_plane) {
            (Axis::X, Axis::Y) | (Axis::Xi, Axis::Yi) => |p: &&PointPointer| p.borrow().2 >= 0.0,
            (Axis::Y, Axis::Xi) | (Axis::Yi, Axis::X) => |p: &&PointPointer| p.borrow().2 >= 0.0,
            (Axis::X, Axis::Z) | (Axis::Xi, Axis::Zi) => |p: &&PointPointer| p.borrow().1 <= 0.0,
            (Axis::Z, Axis::Xi) | (Axis::Zi, Axis::X) => |p: &&PointPointer| p.borrow().1 <= 0.0,
            (Axis::X, Axis::Yi) | (Axis::Xi, Axis::Y) => |p: &&PointPointer| p.borrow().2 <= 0.0,
            (Axis::Y, Axis::X) | (Axis::Yi, Axis::Xi) => |p: &&PointPointer| p.borrow().2 <= 0.0,
            (Axis::X, Axis::Zi) | (Axis::Xi, Axis::Z) => |p: &&PointPointer| p.borrow().1 >= 0.0,
            (Axis::Z, Axis::X) | (Axis::Zi, Axis::Xi) => |p: &&PointPointer| p.borrow().1 >= 0.0,
            (Axis::Y, Axis::Z) | (Axis::Yi, Axis::Zi) => |p: &&PointPointer| p.borrow().0 >= 0.0,
            (Axis::Z, Axis::Yi) | (Axis::Zi, Axis::Y) => |p: &&PointPointer| p.borrow().0 >= 0.0,
            (Axis::Y, Axis::Zi) | (Axis::Yi, Axis::Z) => |p: &&PointPointer| p.borrow().0 <= 0.0,
            (Axis::Z, Axis::Y) | (Axis::Zi, Axis::Yi) => |p: &&PointPointer| p.borrow().0 <= 0.0,
            _ => unimplemented!("shouldn't happen lmao"),
        }
    }

    fn find_face_by_plane(&self, plane: Axis) -> &Face {
        self.faces
            .iter()
            .find(|(_, face)| face.plane == plane)
            .unwrap()
            .1
    }

    pub fn fold(&mut self) {
        if self.folded {
            return;
        }

        for edge_idx in 0..self.foldable_edges.len() {
            let edge_axis = self.foldable_edges[edge_idx].axis();
            let face_plane = self
                .faces
                .get(&self.foldable_edges[edge_idx].face_id)
                .unwrap()
                .plane;
            let coords = self.foldable_edges[edge_idx].start.borrow().coords();

            self.translate(coords);

            let filter = Self::rotation_filter(edge_axis, face_plane);

            let turned_faces: Vec<u8> = self
                .faces
                .iter()
                .filter(|(_, face)| face.points().iter().all(filter))
                .map(|(id, _)| *id)
                .collect();

            self.rotate(turned_faces, edge_axis);
        }

        self.translate(self.find_origo());
        self.folded = true;
    }

    pub fn edge_map(&self) -> [[(usize, usize, bool); 4]; 6] {
        if !self.folded {
            unimplemented!("not folded");
        }

        let mut res = [[(0, 0, false); 4]; 6]; // [east, south, west, north]

        for face_id in 0..6 {
            let face = &self.faces[&(face_id as u8)];
            let plane = face.plane;
            let mut dir = face.right;
            let mut prev_dir = Axis::from_vec(Quaternion::rotate_point(
                dir.coords(),
                plane.coords(),
                90.0,
                0,
            ));

            for dir_id in 0..4 {
                let neighbor = self.find_face_by_plane(dir);

                let dir_on_neighbor = Axis::from_vec(Quaternion::rotate_point(
                    dir.coords(),
                    prev_dir.coords(),
                    90.0,
                    0,
                ));
                let new_dir_id = neighbor.count_turns(dir_on_neighbor);
                let new_dir_inverse = (new_dir_id + 2) % 4;

                res[face_id][dir_id] = (
                    neighbor.id as usize,
                    new_dir_id as usize,
                    face.edge_axis(dir_id as u8) == neighbor.edge_axis(new_dir_inverse),
                );
                prev_dir = dir;
                dir = Axis::from_vec(Quaternion::rotate_point(
                    dir.coords(),
                    plane.coords(),
                    -90.0,
                    0,
                ));
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_fold_cube_net() {
        let grid = vec![
            vec![-1, 0, -1],
            vec![1, 2, 3],
            vec![-1, 4, -1],
            vec![-1, 5, -1],
        ];

        let mut cube_net = CubeNet::from_grid(grid, 50.0);
        cube_net.fold();

        assert_eq!(
            (
                cube_net.faces[&0].point_vals(),
                cube_net.faces[&0].plane,
                cube_net.faces[&0].right
            ),
            (
                [
                    Point::new(50.0, 0.0, 0.0),
                    Point::new(50.0, 50.0, 0.0),
                    Point::new(0.0, 0.0, 0.0),
                    Point::new(0.0, 50.0, 0.0)
                ],
                Axis::Zi,
                Axis::Y
            )
        );
        assert_eq!(
            (
                cube_net.faces[&1].point_vals(),
                cube_net.faces[&1].plane,
                cube_net.faces[&1].right
            ),
            (
                [
                    Point::new(50.0, 0.0, 0.0),
                    Point::new(0.0, 0.0, 0.0),
                    Point::new(50.0, 0.0, 50.0),
                    Point::new(0.0, 0.0, 50.0)
                ],
                Axis::Yi,
                Axis::Xi
            )
        );
        assert_eq!(
            (
                cube_net.faces[&2].point_vals(),
                cube_net.faces[&2].plane,
                cube_net.faces[&2].right
            ),
            (
                [
                    Point::new(0.0, 0.0, 0.0),
                    Point::new(0.0, 50.0, 0.0),
                    Point::new(0.0, 0.0, 50.0),
                    Point::new(0.0, 50.0, 50.0)
                ],
                Axis::Xi,
                Axis::Y
            )
        );
        assert_eq!(
            (
                cube_net.faces[&3].point_vals(),
                cube_net.faces[&3].plane,
                cube_net.faces[&3].right
            ),
            (
                [
                    Point::new(0.0, 50.0, 0.0),
                    Point::new(50.0, 50.0, 0.0),
                    Point::new(0.0, 50.0, 50.0),
                    Point::new(50.0, 50.0, 50.0)
                ],
                Axis::Y,
                Axis::X
            )
        );
        assert_eq!(
            (
                cube_net.faces[&4].point_vals(),
                cube_net.faces[&4].plane,
                cube_net.faces[&4].right
            ),
            (
                [
                    Point::new(0.0, 0.0, 50.0),
                    Point::new(0.0, 50.0, 50.0),
                    Point::new(50.0, 0.0, 50.0),
                    Point::new(50.0, 50.0, 50.0)
                ],
                Axis::Z,
                Axis::Y
            )
        );
        assert_eq!(
            (
                cube_net.faces[&5].point_vals(),
                cube_net.faces[&5].plane,
                cube_net.faces[&5].right
            ),
            (
                [
                    Point::new(50.0, 0.0, 50.0),
                    Point::new(50.0, 50.0, 50.0),
                    Point::new(50.0, 0.0, 0.0),
                    Point::new(50.0, 50.0, 0.0)
                ],
                Axis::X,
                Axis::Y
            )
        );
    }

    #[test]
    fn can_hash_pointpointer() {
        let p1_0 = PointPointer(Rc::new(RefCell::new(Point::new(50.0, 0.0, 0.0))));
        let p1_1 = PointPointer(Rc::clone(&p1_0.0));
        let p1_2 = PointPointer(Rc::clone(&p1_1.0));

        let p2 = PointPointer(Rc::new(RefCell::new(Point::new(50.0, 0.0, 0.0))));

        let p3_0 = PointPointer(Rc::new(RefCell::new(Point::new(34.0, -20.0, -440.0))));
        let p3_1 = PointPointer(Rc::clone(&p3_0.0));

        let mut set = HashSet::new();

        set.insert(&p1_0);
        assert_eq!(set.len(), 1);
        assert!(set.contains(&p1_0));
        assert!(set.contains(&p1_1));
        assert!(set.contains(&p1_2));
        assert!(!set.contains(&p2));
        assert!(!set.contains(&p3_0));
        assert!(!set.contains(&p3_1));

        assert!(!set.insert(&p1_1));
        assert!(!set.insert(&p1_2));

        assert!(set.insert(&p2));

        assert!(set.insert(&p3_0));
        assert!(!set.insert(&p3_1));
    }

    #[test]
    fn can_build_cube_net_from_grid() {
        let grid = vec![
            vec![-1, 0, -1],
            vec![1, 2, 3],
            vec![-1, 4, -1],
            vec![-1, 5, -1],
        ];

        let cube_net = CubeNet::from_grid(grid, 50.0);
        let points = vec![
            Point::new(50.0, 0.0, 0.0),
            Point::new(100.0, 0.0, 0.0),
            Point::new(0.0, 50.0, 0.0),
            Point::new(50.0, 50.0, 0.0),
            Point::new(100.0, 50.0, 0.0),
            Point::new(150.0, 50.0, 0.0),
            Point::new(0.0, 100.0, 0.0),
            Point::new(50.0, 100.0, 0.0),
            Point::new(100.0, 100.0, 0.0),
            Point::new(150.0, 100.0, 0.0),
            Point::new(50.0, 150.0, 0.0),
            Point::new(100.0, 150.0, 0.0),
            Point::new(50.0, 200.0, 0.0),
            Point::new(100.0, 200.0, 0.0),
        ];

        assert_eq!(cube_net.faces.len(), 6);

        assert_eq!(
            cube_net.faces[&0].point_vals(),
            [
                points[0].clone(),
                points[1].clone(),
                points[3].clone(),
                points[4].clone()
            ]
        );
        assert_eq!(
            cube_net.faces[&1].point_vals(),
            [
                points[2].clone(),
                points[3].clone(),
                points[6].clone(),
                points[7].clone()
            ]
        );
        assert_eq!(
            cube_net.faces[&2].point_vals(),
            [
                points[3].clone(),
                points[4].clone(),
                points[7].clone(),
                points[8].clone()
            ]
        );
        assert_eq!(
            cube_net.faces[&3].point_vals(),
            [
                points[4].clone(),
                points[5].clone(),
                points[8].clone(),
                points[9].clone()
            ]
        );
        assert_eq!(
            cube_net.faces[&4].point_vals(),
            [
                points[7].clone(),
                points[8].clone(),
                points[10].clone(),
                points[11].clone()
            ]
        );
        assert_eq!(
            cube_net.faces[&5].point_vals(),
            [
                points[10].clone(),
                points[11].clone(),
                points[12].clone(),
                points[13].clone()
            ]
        );

        assert_eq!(cube_net.foldable_edges.len(), 5);

        assert_eq!(
            cube_net.foldable_edges[0].point_vals(),
            [points[3].clone(), points[4].clone()]
        );
        assert_eq!(
            cube_net.foldable_edges[1].point_vals(),
            [points[3].clone(), points[7].clone()]
        );
        assert_eq!(
            cube_net.foldable_edges[2].point_vals(),
            [points[4].clone(), points[8].clone()]
        );
        assert_eq!(
            cube_net.foldable_edges[3].point_vals(),
            [points[7].clone(), points[8].clone()]
        );
        assert_eq!(
            cube_net.foldable_edges[4].point_vals(),
            [points[10].clone(), points[11].clone()]
        );

        assert_eq!(cube_net.foldable_edges[0].face_id, 2);
        assert_eq!(cube_net.foldable_edges[1].face_id, 2);
        assert_eq!(cube_net.foldable_edges[2].face_id, 3);
        assert_eq!(cube_net.foldable_edges[3].face_id, 4);
        assert_eq!(cube_net.foldable_edges[4].face_id, 5);
    }
}
