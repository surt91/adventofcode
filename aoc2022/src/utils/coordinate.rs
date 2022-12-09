use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Point {x, y}
    }

    pub fn distance_l0(&self, other: &Point) -> isize{
        let diff = self - other;
        std::cmp::max(diff.x.abs(), diff.y.abs())
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