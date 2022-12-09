use std::ops::{Add, AddAssign, Sub, Neg};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Point {x, y}
    }

    pub fn distance_l0(&self, other: &Point) -> isize {
        let diff = self - other;
        std::cmp::max(diff.x.abs(), diff.y.abs())
    }

    pub fn octant(&self, other: &Point) -> Point {
        let diff = other - self;
        Point {
            x: diff.x.signum(),
            y: diff.y.signum(),
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Neg for &Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}