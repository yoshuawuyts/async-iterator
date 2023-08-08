use crate::Iterator;

/// Conversion into an [`Iterator`].
pub trait IntoIterator {
    /// The type of the elements being iterated over.
    type Item;

    /// Which kind of iterator are we turning this into?
    type IntoIter: Iterator<Item = Self::Item>;

    /// Creates an iterator from a value.
    async fn into_iter(self) -> Self::IntoIter;
}

impl<I: Iterator> IntoIterator for I {
    type Item = I::Item;
    type IntoIter = I;

    async fn into_iter(self) -> I {
        self
    }
}
