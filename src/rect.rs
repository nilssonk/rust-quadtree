mod comparison;
mod hashing;

#[derive(Clone, Copy)]
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}