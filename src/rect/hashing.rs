use crate::rect::Rect;

impl<T> std::hash::Hash for Rect<T>
where
    T: std::hash::Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.w.hash(state);
        self.h.hash(state);
    }
}
