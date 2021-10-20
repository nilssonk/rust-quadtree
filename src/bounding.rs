use crate::rect::Rect;
use num_traits::NumOps;

/// # Summary
/// A type T which implements BoundingNumber<T> is something from which a BoundingBox can be constructed.
pub trait BoundingNumber<T>: NumOps<T> + Ord + From<u8> + Copy {}

impl<T> BoundingNumber<T> for T where T: NumOps<T> + Ord + From<u8> + Copy {}

/// # Summary
/// A type U which implements the BoxBounded<T> trait (for some T that also satisfies
/// BoundingNumber<T>) provides a function that returns a Rect<T> to be used in bounding box
/// calculations.
pub trait BoxBounded<T>
where
    T: BoundingNumber<T>,
{
    fn get_bounding_box(&self) -> Rect<T>;
}
