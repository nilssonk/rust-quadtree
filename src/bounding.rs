use crate::rect::Rect;

use num_traits::{FromPrimitive, Num};

/// # Summary
/// A type that implements `BoundingNumber` can be used for the elements of a `BoundingBox`.
pub trait BoundingNumber: Num + Ord + FromPrimitive + Copy {}

/// Default implementation
impl<T> BoundingNumber for T where T: Num + Ord + FromPrimitive + Copy {}

/// # Summary
/// A type that implements the `BoxBounded<T>` trait (for some type `T` that satisfies
/// `BoundingNumber`) provides a function `get_bounding_box()` that returns a `Rect<T>` for use in bounding box
/// calculations.
pub trait BoxBounded<T: BoundingNumber> {
    fn get_bounding_box(&self) -> Rect<T>;
}
