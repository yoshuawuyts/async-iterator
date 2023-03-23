//! Async self-referential (pinned) iterator support.

use std::future::Future;
use std::pin::{pin, Pin};

use pin_project::pin_project;

/// An interface for dealing with iterators.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub trait AsyncPinnedIterator {
    /// The type of the elements being iterated over.
    type Item;

    /// Advances the iterator and returns the next value.
    async fn next(self: Pin<&mut Self>) -> Option<Self::Item>;

    /// Returns the bounds on the remaining length of the iterator.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Takes a closure and creates an iterator which calls that closure on each element.
    fn map<B, F>(self, f: F) -> AsyncPinnedMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        AsyncPinnedMap { stream: self, f }
    }

    /// Transforms an iterator into a collection.
    // #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
    async fn collect<B: FromAsyncPinnedIterator<Self::Item>>(self) -> B
    where
        Self: Sized,
    {
        let fut = <B as FromAsyncPinnedIterator<_>>::from_iter(self);
        fut.await
    }
}

/// Conversion into an [`Iterator`].

pub trait IntoAsyncPinnedIterator {
    /// The type of the elements being iterated over.
    type Item;

    /// Which kind of iterator are we turning this into?
    type IntoIter: AsyncPinnedIterator<Item = Self::Item>;

    /// Creates an iterator from a value.
    async fn into_iter(self) -> Self::IntoIter;
}

impl<I: AsyncPinnedIterator> IntoAsyncPinnedIterator for I {
    type Item = I::Item;
    type IntoIter = I;

    async fn into_iter(self) -> I {
        self
    }
}

/// Conversion from an [`Iterator`].

pub trait FromAsyncPinnedIterator<A>: Sized {
    /// Creates a value from an iterator.
    async fn from_iter<T: IntoAsyncPinnedIterator<Item = A>>(iter: T) -> Self;
}

impl<T> FromAsyncPinnedIterator<T> for Vec<T> {
    async fn from_iter<I: IntoAsyncPinnedIterator<Item = T>>(iter: I) -> Vec<T> {
        let mut iter = pin!(iter.into_iter().await);
        let mut output = Vec::with_capacity(iter.size_hint().1.unwrap_or_default());
        while let Some(item) = iter.as_mut().next().await {
            output.push(item);
        }
        output
    }
}

/// Extend a collection with the contents of an iterator.

pub trait AsyncPinnedExtend<A> {
    /// Extends a collection with the contents of an iterator.
    async fn extend<T: IntoAsyncPinnedIterator<Item = A>>(&mut self, iter: T);
}

impl<T> AsyncPinnedExtend<T> for Vec<T> {
    async fn extend<I: IntoAsyncPinnedIterator<Item = T>>(&mut self, iter: I) {
        let mut iter = pin!(iter.into_iter().await);
        self.reserve(iter.size_hint().1.unwrap_or_default());
        while let Some(item) = iter.as_mut().next().await {
            self.push(item);
        }
    }
}

/// An iterator that maps value of another stream with a function.
#[derive(Debug)]
#[pin_project]
pub struct AsyncPinnedMap<I, F> {
    #[pin]
    stream: I,
    f: F,
}

impl<I, F, B, Fut> AsyncPinnedIterator for AsyncPinnedMap<I, F>
where
    I: AsyncPinnedIterator,
    F: FnMut(I::Item) -> Fut,
    Fut: Future<Output = B>,
{
    type Item = B;

    async fn next(self: Pin<&mut Self>) -> Option<Self::Item> {
        let this = self.project();
        let item = this.stream.next().await?;
        let out = (this.f)(item).await;
        Some(out)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
        #[allow(unused)]
        async fn foo(iter: impl AsyncPinnedIterator<Item = u32>) {
            let v: Vec<_> = iter.collect().await;
        }
    }
}
