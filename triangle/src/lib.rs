use num::Num;
use std::ops::{MulAssign, SubAssign};

pub struct Triangle<T>(T, T, T);

impl<T> Triangle<T>
where
    T: Num + Copy + MulAssign + SubAssign + PartialOrd,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let t = Triangle(sides[0], sides[1], sides[2]);
        let has_invalid_sides = t.0 + t.1 <= t.2 || t.0 + t.2 <= t.1 || t.1 + t.2 <= t.0;
        if sides.iter().any(|&n| n.is_zero()) || has_invalid_sides {
            return None;
        }

        Some(t)
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == self.1 && self.0 == self.2
    }

    pub fn is_scalene(&self) -> bool {
        self.0 != self.1 && self.0 != self.2 && self.1 != self.2
    }

    pub fn is_isosceles(&self) -> bool {
        self.0 == self.1 || self.1 == self.2 || self.0 == self.2
    }
}
