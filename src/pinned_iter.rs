//! Self-referential (pinned) iterator support.

use std::pin::{pin, Pin};

use pin_project::pin_project;

/// An interface for dealing with iterators.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub trait PinnedIterator {
    /// The type of the elements being iterated over.
    type Item;

    /// Advances the iterator and returns the next value.
    fn next(self: Pin<&mut Self>) -> Option<Self::Item>;

    /// Returns the bounds on the remaining length of the iterator.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Takes a closure and creates an iterator which calls that closure on each element.
    fn map<B, F>(self, f: F) -> PinnedMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        PinnedMap { stream: self, f }
    }

    /// Transforms an iterator into a collection.
    // #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
    fn collect<B: FromPinnedIterator<Self::Item>>(self) -> B
    where
        Self: Sized,
    {
        let fut = <B as FromPinnedIterator<_>>::from_iter(self);
        fut
    }
}

/// Conversion into an [`Iterator`].

pub trait IntoPinnedIterator {
    /// The type of the elements being iterated over.
    type Item;

    /// Which kind of iterator are we turning this into?
    type IntoIter: PinnedIterator<Item = Self::Item>;

    /// Creates an iterator from a value.
    fn into_iter(self) -> Self::IntoIter;
}

impl<I: PinnedIterator> IntoPinnedIterator for I {
    type Item = I::Item;
    type IntoIter = I;

    fn into_iter(self) -> I {
        self
    }
}

/// Conversion from an [`Iterator`].

pub trait FromPinnedIterator<A>: Sized {
    /// Creates a value from an iterator.
    fn from_iter<T: IntoPinnedIterator<Item = A>>(iter: T) -> Self;
}

impl<T> FromPinnedIterator<T> for Vec<T> {
    fn from_iter<I: IntoPinnedIterator<Item = T>>(iter: I) -> Vec<T> {
        let mut iter = pin!(iter.into_iter());
        let mut output = Vec::with_capacity(iter.size_hint().1.unwrap_or_default());
        while let Some(item) = iter.as_mut().next() {
            output.push(item);
        }
        output
    }
}

/// Extend a collection with the contents of an iterator.

pub trait PinnedExtend<A> {
    /// Extends a collection with the contents of an iterator.
    fn extend<T: IntoPinnedIterator<Item = A>>(&mut self, iter: T);
}

impl<T> PinnedExtend<T> for Vec<T> {
    fn extend<I: IntoPinnedIterator<Item = T>>(&mut self, iter: I) {
        let mut iter = pin!(iter.into_iter());
        self.reserve(iter.size_hint().1.unwrap_or_default());
        while let Some(item) = iter.as_mut().next() {
            self.push(item);
        }
    }
}

/// An iterator that maps value of another stream with a function.
#[derive(Debug)]
#[pin_project]
pub struct PinnedMap<I, F> {
    #[pin]
    stream: I,
    f: F,
}

impl<I, F, B> PinnedIterator for PinnedMap<I, F>
where
    I: PinnedIterator,
    F: FnMut(I::Item) -> B,
{
    type Item = B;

    fn next(self: Pin<&mut Self>) -> Option<Self::Item> {
        let this = self.project();
        let item = this.stream.next()?;
        let out = (this.f)(item);
        Some(out)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
        #[allow(unused)]
        fn foo(iter: impl PinnedIterator<Item = u32>) {
            let v: Vec<_> = iter.collect();
        }
    }
}
