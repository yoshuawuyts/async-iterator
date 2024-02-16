mod lend;
mod lend_mut;
mod map;

pub use lend::Lend;
pub use lend_mut::LendMut;
pub use map::Map;

use crate::FromIterator;

/// An interface for dealing with iterators.

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub trait Iterator {
    /// The type of the elements being iterated over.
    type Item;

    /// Advances the iterator and returns the next value.
    async fn next(&mut self) -> Option<Self::Item>;

    /// Returns the bounds on the remaining length of the iterator.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Takes a closure and creates an iterator which calls that closure on each element.
    #[must_use = "iterators do nothing unless iterated over"]
    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        Map::new(self, f)
    }

    /// Creates an iterator which uses a closure to determine if an element should be yielded.
    #[must_use = "iterators do nothing unless iterated over"]
    fn filter<P>(self, predicate: P) -> Filter<Self, P>
    where
        P: async FnMut(&Self::Item) -> bool,
        // P: FnMut(&Self::Item) -> bool,
        Self: Sized,
    {
        Filter::new(self, predicate)
    }

    /// Transforms an iterator into a collection.
    #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
    async fn collect<B: FromIterator<Self::Item>>(self) -> B
    where
        Self: Sized,
    {
        let fut = <B as crate::FromIterator<_>>::from_iter(self);
        fut.await
    }

    /// Creates an iterator which yields a reference to `self` as well as
    /// the next value.
    #[must_use = "iterators do nothing unless iterated over"]
    fn lend(self) -> Lend<Self>
    where
        Self: Sized,
    {
        Lend::new(self)
    }

    /// Creates an iterator which yields a mutable reference to `self` as well
    /// as the next value.
    #[must_use = "iterators do nothing unless iterated over"]
    fn lend_mut(self) -> LendMut<Self>
    where
        Self: Sized,
    {
        LendMut::new(self)
    }
}

/// An async iterator that filters the elements of `iter` with `predicate`.
///
/// This `struct` is created by the [`filter`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`filter`]: crate::Iterator::filter
/// [`Iterator`]: crate::Iterator
#[derive(Debug)]
pub struct Filter<I, P>
where
    I: crate::Iterator,
    P: async FnMut(&I::Item) -> bool,
{
    iter: I,
    predicate: P,
}

impl<I, P> crate::Iterator for Filter<I, P>
where
    I: crate::Iterator,
    P: async FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    async fn next(&mut self) -> Option<Self::Item> {
        loop {
            let item = self.iter.next().await?;
            if (self.predicate)(&item).await {
                return Some(item);
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }
}

impl<I, P> Filter<I, P>
where
    I: crate::Iterator,
    P: async FnMut(&I::Item) -> bool,
{
    pub(crate) fn new(stream: I, predicate: P) -> Self {
        Self {
            iter: stream,
            predicate,
        }
    }
}
