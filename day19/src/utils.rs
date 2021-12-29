use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

#[macro_export]
macro_rules! v3 {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::Vector3d { x: $x, y: $y, z: $z }
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub struct Vector3d {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl From<&str> for Vector3d {
    fn from(vs: &str) -> Self {
        let mut vsi = vs.split(',');
        v3![vsi.next().unwrap().parse().unwrap(), vsi.next().unwrap().parse().unwrap(), vsi.next().unwrap().parse().unwrap()]
    }
}

impl Sub for Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Vector3d {
    pub fn length(&self) -> u64 {
        (((self.x * self.x + self.y * self.y + self.z * self.z) as f64).sqrt() * 10f64.powi(5)).trunc() as u64
    }

    pub fn manhattan_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

pub type Rotator = fn(Vector3d) -> Vector3d;

pub const ROTATORS: [Rotator; 24] = [
    // View from 0/0/1
    |v| v3![v.x, v.y, v.z],
    |v| v3![v.y, -v.x, v.z],
    |v| v3![-v.x, -v.y, v.z],
    |v| v3![-v.y, v.x, v.z],
    // View from 0/0/-1
    |v| v3![-v.x, v.y, -v.z],
    |v| v3![v.y, v.x, -v.z],
    |v| v3![v.x, -v.y, -v.z],
    |v| v3![-v.y, -v.x, -v.z],
    // View from 1/0/0
    |v| v3![-v.z, v.y, v.x],
    |v| v3![v.y, v.z, v.x],
    |v| v3![v.z, -v.y, v.x],
    |v| v3![-v.y, -v.z, v.x],
    // View from -1/0/0
    |v| v3![v.z, v.y, -v.x],
    |v| v3![v.y, -v.z, -v.x],
    |v| v3![-v.z, -v.y, -v.x],
    |v| v3![-v.y, v.z, -v.x],
    // View from 0/1/0
    |v| v3![-v.z, -v.x, v.y],
    |v| v3![-v.x, v.z, v.y],
    |v| v3![v.z, v.x, v.y],
    |v| v3![v.x, -v.z, v.y],
    // View from 0/-1/0
    |v| v3![v.z, -v.x, -v.y],
    |v| v3![-v.x, -v.z, -v.y],
    |v| v3![-v.z, v.x, -v.y],
    |v| v3![v.x, v.z, -v.y],
];

pub struct Translation {
    pub movement: Vector3d,
    pub rotator_fn: Rotator,
    pub rotator_ix: usize,
}

impl Translation {
    pub fn new(movement: Vector3d, rotator_ix: usize) -> Self {
        Self {
            movement,
            rotator_ix,
            rotator_fn: ROTATORS[rotator_ix],
        }
    }

    pub fn translate_set(&self, s: &HashSet<Vector3d>) -> HashSet<Vector3d> {
        let mut result = HashSet::<Vector3d>::with_capacity(s.len());
        for sb in s.iter().cloned() {
            result.insert(self.translate(sb));
        }

        result
    }

    pub fn translate(&self, v: Vector3d) -> Vector3d {
        self.rotate(v) - self.movement
    }

    pub fn rotate(&self, v: Vector3d) -> Vector3d {
        (self.rotator_fn)(v)
    }
}

impl Clone for Translation {
    fn clone(&self) -> Self {
        Self {
            movement: self.movement,
            rotator_fn: ROTATORS[self.rotator_ix],
            rotator_ix: self.rotator_ix,
        }
    }
}

impl Default for Translation {
    fn default() -> Self {
        Self {
            movement: Default::default(),
            rotator_fn: ROTATORS[0],
            rotator_ix: Default::default(),
        }
    }
}

pub struct ScannerData {
    pub beacons: HashSet<Vector3d>,
    pub distances: HashSet<u64>,
    pub movement_to_zero: Vector3d,
    pub translations_to_zero: Vec<Translation>,
}

impl ScannerData {
    pub fn new(beacons: Vec<Vector3d>) -> Self {
        ScannerData {
            beacons: HashSet::from_iter(beacons.iter().cloned()),
            distances: Self::get_distances(&beacons),
            movement_to_zero: Default::default(),
            translations_to_zero: Vec::new(),
        }
    }

    fn get_distances(beacons: &[Vector3d]) -> HashSet<u64> {
        let len = beacons.len();
        let mut distances = HashSet::<_>::with_capacity(len * (len - 1) / 2);
        for i in 0..len - 1 {
            for j in i + 1..len {
                distances.insert((beacons[j] - beacons[i]).length());
            }
        }

        distances
    }
}

#[cfg(test)]
mod tests_utils {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_from_str() {
        assert_eq!(v3![1, 2, 3], "1,2,3".into());
        assert_eq!(v3![10, 20, 30], "10,20,30".into());
        assert_eq!(v3![10, 20, 30], Vector3d::from("10,20,30"));
    }

    #[test]
    fn test_sub() {
        assert_eq!(v3![1, 1, 1], v3![3, 3, 3] - v3![2, 2, 2]);
    }

    #[test]
    fn test_v3d() {
        assert_eq!(v3![1, 1, 1], v3![1, 1, 1]);
    }

    #[test]
    fn test_rotation() {
        let originals = vec![v3![-1, -1, 1], v3![-2, -2, 2], v3![-3, -3, 3], v3![-2, -3, 1], v3![5, 6, -4], v3![8, 0, 7]];

        let mut rotated = Vec::<HashSet<Vector3d>>::with_capacity(originals.len());
        for o in originals {
            let mut hs = HashSet::<Vector3d>::with_capacity(24);
            for r in ROTATORS {
                hs.insert(r(o));
            }

            rotated.push(hs);
        }

        assert!(rotated[0].contains(&v3!(-1, -1, 1)));
        assert!(rotated[0].contains(&v3!(1, -1, 1)));
        assert!(rotated[0].contains(&v3!(-1, -1, -1)));
        assert!(rotated[0].contains(&v3!(1, 1, -1)));
        assert!(rotated[0].contains(&v3!(1, 1, 1)));

        assert!(rotated[1].contains(&v3!(-2, -2, 2)));
        assert!(rotated[1].contains(&v3!(2, -2, 2)));
        assert!(rotated[1].contains(&v3!(-2, -2, -2)));
        assert!(rotated[1].contains(&v3!(2, 2, -2)));
        assert!(rotated[1].contains(&v3!(2, 2, 2)));

        assert!(rotated[2].contains(&v3!(-3, -3, 3)));
        assert!(rotated[2].contains(&v3!(3, -3, 3)));
        assert!(rotated[2].contains(&v3!(-3, -3, -3)));
        assert!(rotated[2].contains(&v3!(3, 3, -3)));
        assert!(rotated[2].contains(&v3!(3, 3, 3)));

        assert!(rotated[3].contains(&v3!(-2, -3, 1)));
        assert!(rotated[3].contains(&v3!(2, -1, 3)));
        assert!(rotated[3].contains(&v3!(-1, -3, -2)));
        assert!(rotated[3].contains(&v3!(1, 3, -2)));
        assert!(rotated[3].contains(&v3!(3, 1, 2)));

        assert!(rotated[4].contains(&v3!(5, 6, -4)));
        assert!(rotated[4].contains(&v3!(-5, 4, -6)));
        assert!(rotated[4].contains(&v3!(4, 6, 5)));
        assert!(rotated[4].contains(&v3!(-4, -6, 5)));
        assert!(rotated[4].contains(&v3!(-6, -4, -5)));

        assert!(rotated[5].contains(&v3!(8, 0, 7)));
        assert!(rotated[5].contains(&v3!(-8, -7, 0)));
        assert!(rotated[5].contains(&v3!(-7, 0, 8)));
        assert!(rotated[5].contains(&v3!(7, 0, 8)));
        assert!(rotated[5].contains(&v3!(0, 7, -8)));
    }

    #[test]
    fn test_length() {
        assert_eq!(538516, v3![2, 3, 4].length())
    }

    #[test]
    fn test_get_distances() {
        let result = ScannerData::get_distances(&vec![v3![404, -588, -901], v3![528, -643, 409]]);
        assert_eq!(result.len(), 2 * 1 / 2);
        assert!(result.contains(&131700455));
    }

    #[test]
    fn test_default_translation() {
        let translation: Translation = Default::default();
        assert_eq!(v3![1, 2, 3], translation.translate(v3![1, 2, 3]));
    }

    #[test]
    fn test_manhattan_distaince() {
        assert_eq!(3621, v3![-1105, 1205, -1229].manhattan_distance(&v3![92, 2380, 20]));
    }
}
