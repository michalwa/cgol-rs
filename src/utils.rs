use std::ops::RangeInclusive;

pub trait RangeExt<T> {
    /// Constrains the value to be contained within the range
    fn clamp(&self, t: T) -> T
    where
        T: Clone + Ord;
}

impl<T> RangeExt<T> for RangeInclusive<T> {
    fn clamp(&self, t: T) -> T
    where
        T: Clone + Ord,
    {
        if &t < self.start() {
            self.start().clone()
        } else if &t > self.end() {
            self.end().clone()
        } else {
            t
        }
    }
}
