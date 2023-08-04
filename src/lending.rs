/// An interface for dealing with iterators which borrow from `Self`

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub trait LendingIterator {
    /// The type of the elements being iterated over.
    type Item<'a>
    where
        Self: 'a;

    /// Advances the iterator and returns the next value.
    async fn next(&mut self) -> Option<Self::Item<'_>>;
}
