use crate::bounding::BoundingNumber;
use crate::rect::Rect;

impl<T> std::cmp::PartialEq for Rect<T>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y && self.w == other.w && self.h == other.h;
    }
}

impl<T> std::cmp::Eq for Rect<T> where T: Eq {}

impl<T> Rect<T>
where
    T: BoundingNumber<T>,
{
    pub fn fits_inside(&self, other: &Rect<T>) -> bool {
        self.x >= other.x
            && self.x + self.w <= other.x + other.w
            && self.y >= other.y
            && self.y + self.h <= other.y + other.h
    }
}
