use crate::bounding::BoundingNumber;
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq, Copy, Hash)]
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

impl<T> PartialOrd for Rect<T>
where
    T: BoundingNumber,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Ordering::*;

        if self == other {
            return Some(Equal);
        }

        // Fits inside
        if self.x >= other.x
            && self.x + self.w <= other.x + other.w
            && self.y >= other.y
            && self.y + self.h <= other.y + other.h
        {
            return Some(Less);
        }

        // Does not fit inside
        Some(Greater)
    }
}
