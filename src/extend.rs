use crate::IntoIterator;
use crate::Iterator;

/// Extend a collection with the contents of an iterator.

pub trait Extend<A> {
    /// Extends a collection with the contents of an iterator.
    async fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = A>;
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T> Extend<T> for std::vec::Vec<T> {
    async fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let mut iter = iter.into_iter().await;
        self.reserve(iter.size_hint().1.unwrap_or_default());
        while let Some(item) = iter.next().await {
            self.push(item);
        }
    }
}
