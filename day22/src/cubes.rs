use std::cmp::{self, max, min};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Point3d {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Cube {
    pub corner1: Point3d,
    pub corner2: Point3d,
}

impl Cube {
    pub fn new(x1: i32, y1: i32, z1: i32, x2: i32, y2: i32, z2: i32) -> Self {
        Self {
            corner1: Point3d {
                x: min(x1, x2),
                y: min(y1, y2),
                z: min(z1, z2),
            },
            corner2: Point3d {
                x: max(x1, x2),
                y: max(y1, y2),
                z: max(z1, z2),
            },
        }
    }

    pub fn volume(&self) -> i64 {
        (self.corner2.x - self.corner1.x + 1) as i64 * (self.corner2.y - self.corner1.y + 1) as i64 * (self.corner2.z - self.corner1.z + 1) as i64
    }

    pub fn is_point_inside(&self, p: &Point3d) -> bool {
        self.corner1.x < p.x && p.x < self.corner2.x && self.corner1.y < p.y && p.y < self.corner2.y && self.corner1.z < p.z && p.z < self.corner2.z
    }

    pub fn points(&self) -> impl Iterator<Item = Point3d> {
        vec![
            Point3d {
                x: self.corner1.x,
                y: self.corner1.y,
                z: self.corner1.z,
            },
            Point3d {
                x: self.corner2.x,
                y: self.corner1.y,
                z: self.corner1.z,
            },
            Point3d {
                x: self.corner2.x,
                y: self.corner2.y,
                z: self.corner1.z,
            },
            Point3d {
                x: self.corner1.x,
                y: self.corner2.y,
                z: self.corner1.z,
            },
            Point3d {
                x: self.corner1.x,
                y: self.corner1.y,
                z: self.corner2.z,
            },
            Point3d {
                x: self.corner2.x,
                y: self.corner1.y,
                z: self.corner2.z,
            },
            Point3d {
                x: self.corner2.x,
                y: self.corner2.y,
                z: self.corner2.z,
            },
            Point3d {
                x: self.corner1.x,
                y: self.corner2.y,
                z: self.corner2.z,
            },
        ]
        .into_iter()
    }

    pub fn is_inside(&self, other: &Cube) -> bool {
        other.points().all(|p| self.is_point_inside(&p))
    }

    pub fn intersection(&self, other: &Cube) -> Option<Cube> {
        if (self.corner1.x < other.corner1.x && self.corner2.x < other.corner1.x)
            || (self.corner1.x > other.corner2.x && self.corner2.x > other.corner2.x)
            || (self.corner1.y < other.corner1.y && self.corner2.y < other.corner1.y)
            || (self.corner1.y > other.corner2.y && self.corner2.y > other.corner2.y)
            || (self.corner1.z < other.corner1.z && self.corner2.z < other.corner1.z)
            || (self.corner1.z > other.corner2.z && self.corner2.z > other.corner2.z)
        {
            return None;
        }

        Some(Cube::new(
            max(self.corner1.x, other.corner1.x),
            max(self.corner1.y, other.corner1.y),
            max(self.corner1.z, other.corner1.z),
            min(self.corner2.x, other.corner2.x),
            min(self.corner2.y, other.corner2.y),
            min(self.corner2.z, other.corner2.z),
        ))
    }

    pub fn encloses(&self, other: &Cube) -> bool {
        self.corner1.x <= other.corner1.x
            && self.corner1.y <= other.corner1.y
            && self.corner1.z <= other.corner1.z
            && self.corner2.x >= other.corner2.x
            && self.corner2.y >= other.corner2.y
            && self.corner2.z >= other.corner2.z
    }

    pub fn cut(&self, other: &Cube) -> Vec<Cube> {
        let mut result = Vec::new();

        if other.encloses(self) {
            // if cut region encloses, the result is empty
            return result;
        }

        if self.intersection(other).is_none() {
            // if cut region does not intersect, original cuboid is result
            result.push(*self);
            return result;
        }

        // top
        if let Some(i) = self.intersection(&Cube::new(i32::MIN, other.corner2.y + 1, i32::MIN, i32::MAX, i32::MAX, i32::MAX)) {
            result.push(i);
        }

        // bottom
        if let Some(i) = self.intersection(&Cube::new(i32::MIN, i32::MIN, i32::MIN, i32::MAX, other.corner1.y - 1, i32::MAX)) {
            result.push(i);
        }

        // side 1
        if let Some(i) = self.intersection(&Cube::new(i32::MIN, other.corner1.y, i32::MIN, other.corner1.x - 1, other.corner2.y, i32::MAX)) {
            result.push(i);
        }

        // side 2
        if let Some(i) = self.intersection(&Cube::new(other.corner2.x + 1, other.corner1.y, i32::MIN, i32::MAX, other.corner2.y, i32::MAX)) {
            result.push(i);
        }

        // side 3
        if let Some(i) = self.intersection(&Cube::new(other.corner1.x, other.corner1.y, i32::MIN, other.corner2.x, other.corner2.y, other.corner1.z - 1)) {
            result.push(i);
        }

        // side 4
        if let Some(i) = self.intersection(&Cube::new(other.corner1.x, other.corner1.y, other.corner2.z + 1, other.corner2.x, other.corner2.y, i32::MAX)) {
            result.push(i);
        }

        result
    }
}

#[cfg(test)]
mod tests_cubes {
    use super::*;

    #[test]
    fn test_is_point_inside() {
        let cube = Cube::new(0, 0, 0, 10, 10, 10);
        assert!(cube.is_point_inside(&Point3d { x: 5, y: 5, z: 5 }));
        assert!(!cube.is_point_inside(&Point3d { x: 15, y: 5, z: 5 }));
        assert!(!cube.is_point_inside(&Point3d { x: 5, y: 15, z: 5 }));
        assert!(!cube.is_point_inside(&Point3d { x: 5, y: 5, z: 15 }));
    }

    #[test]
    fn test_is_point_inside_neg() {
        let cube = Cube::new(-5, 5, -5, 5, -5, 5);
        assert!(cube.is_point_inside(&Point3d { x: 0, y: 0, z: 0 }));
        assert!(!cube.is_point_inside(&Point3d { x: -6, y: 0, z: 0 }));
    }

    #[test]
    fn test_new_cube() {
        let cube = Cube::new(-5, -5, -5, 5, 5, 5);
        assert_eq!(Cube::new(-5, -5, -5, 5, 5, 5), cube);

        let cube = Cube::new(5, 5, 5, -5, -5, -5);
        assert_eq!(Cube::new(-5, -5, -5, 5, 5, 5), cube);

        let cube = Cube::new(5, -5, 5, -5, 5, -5);
        assert_eq!(Cube::new(-5, -5, -5, 5, 5, 5), cube);
    }

    #[test]
    fn test_is_inside() {
        let cube = Cube::new(-5, -5, -5, 5, 5, 5);
        let cube_inside = Cube::new(-2, -2, -2, 2, 2, 2);
        assert!(cube.is_inside(&cube_inside));
    }

    #[test]
    fn test_is_not_inside() {
        let cube = Cube::new(-5, -5, -5, 5, 5, 5);
        let cube_inside = Cube::new(-2, -2, -2, 2, 2, 6);
        assert!(!cube.is_inside(&cube_inside));
    }

    #[test]
    fn test_intersection_partly() {
        let cube = Cube::new(0, 0, 0, 10, 10, 10);
        let cube1 = Cube::new(5, 5, 5, 15, 15, 15);
        assert_eq!(Some(Cube::new(5, 5, 5, 10, 10, 10)), cube.intersection(&cube1));

        let cube = Cube::new(-5, -5, -5, 5, 5, 5);
        let cube1 = Cube::new(-10, 0, -5, 0, 10, 5);
        assert_eq!(Some(Cube::new(-5, 0, -5, 0, 5, 5)), cube.intersection(&cube1));

        let cube = Cube::new(-5, -5, -5, 5, 5, 5);
        let cube1 = Cube::new(-5, -5, -5, 5, 5, 5);
        assert_eq!(Some(Cube::new(-5, -5, -5, 5, 5, 5)), cube.intersection(&cube1));

        let cube = Cube::new(1, 0, 1, 4, 4, 4);
        let cube1 = Cube::new(0, 1, 0, 2, 2, 5);
        assert_eq!(Some(Cube::new(1, 1, 1, 2, 2, 4)), cube.intersection(&cube1));
    }

    #[test]
    fn test_intersection_fully() {
        let cube = Cube::new(-10, -10, -10, 10, 10, 10);
        let cube1 = Cube::new(-5, -5, -5, 5, 5, 5);
        assert_eq!(Some(Cube::new(-5, -5, -5, 5, 5, 5)), cube.intersection(&cube1));

        let cube = Cube::new(-5, -5, -5, 5, 5, 5);
        let cube1 = Cube::new(-10, -10, -10, 10, 10, 10);
        assert_eq!(Some(Cube::new(-5, -5, -5, 5, 5, 5)), cube.intersection(&cube1));
    }

    #[test]
    fn test_no_intersection() {
        let cube = Cube::new(0, 0, 0, 5, 5, 5);
        let cube1 = Cube::new(6, 6, 6, 15, 15, 15);
        assert_eq!(None, cube.intersection(&cube1));
    }

    #[test]
    fn test_encloses() {
        let cube = Cube::new(0, 0, 0, 5, 5, 5);
        let cube1 = Cube::new(1, 1, 1, 4, 4, 4);
        assert!(cube.encloses(&cube1));
    }

    #[test]
    fn test_volume() {
        assert_eq!(27, Cube::new(0, 0, 0, 2, 2, 2).volume());
        assert_eq!(64, Cube::new(-1, -1, -1, 2, 2, 2).volume());
        assert_eq!(64, Cube::new(2, 2, 2, -1, -1, -1).volume());
    }

    #[test]
    fn test_cut_no_intersection() {
        let cube = Cube::new(0, 0, 0, 5, 5, 5);
        let cube1 = Cube::new(6, 6, 6, 8, 8, 8);
        let result = cube.cut(&cube1);
        assert_eq!(1, result.len());
        assert_eq!(cube, result.into_iter().nth(0).unwrap());
    }

    #[test]
    fn test_cut_inside() {
        let cube = Cube::new(0, 0, 0, 10, 10, 10);
        let cube1 = Cube::new(1, 1, 1, 9, 9, 9);
        let result = cube.cut(&cube1);
        assert_eq!(6, result.len());
        assert_eq!(cube.volume() - cube1.volume(), result.iter().map(|c| c.volume()).sum());
    }

    #[test]
    fn test_cut_left() {
        let cube = Cube::new(0, 0, 0, 10, 5, 5);
        let cube1 = Cube::new(0, 0, 0, 5, 5, 5);
        let result = cube.cut(&cube1);
        assert_eq!(1, result.len());
        assert_eq!(cube.volume() - cube1.volume(), result.iter().map(|c| c.volume()).sum());
    }

    #[test]
    fn test_cut_right() {
        let cube = Cube::new(0, 0, 0, 10, 5, 5);
        let cube1 = Cube::new(5, 0, 0, 10, 5, 5);
        let result = cube.cut(&cube1);
        assert_eq!(1, result.len());
        assert_eq!(cube.volume() - cube1.volume(), result.iter().map(|c| c.volume()).sum());
    }

    #[test]
    fn test_cut_piece() {
        let cube = Cube::new(0, 0, 0, 10, 10, 10);
        let cube1 = Cube::new(5, 5, 5, 10, 10, 10);
        let result = cube.cut(&cube1);
        assert_eq!(3, result.len());
        assert_eq!(cube.volume() - cube1.volume(), result.iter().map(|c| c.volume()).sum());
    }

    #[test]
    fn test_cut_everything() {
        let cube = Cube::new(1,1,1,1,1,1);
        let cube1 = Cube::new(1,1,1,1,1,1);
        let result = cube.cut(&cube1);
        assert_eq!(0, result.len());
    }
}
