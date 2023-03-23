//! Async iterator support.

use std::future::Future;

/// An interface for dealing with iterators.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub trait AsyncIterator {
    /// The type of the elements being iterated over.
    type Item;

    /// Advances the iterator and returns the next value.
    async fn next(&mut self) -> Option<Self::Item>;

    /// Returns the bounds on the remaining length of the iterator.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Takes a closure and creates an iterator which calls that closure on each element.
    fn map<B, F>(self, f: F) -> AsyncMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        AsyncMap { stream: self, f }
    }

    /// Transforms an iterator into a collection.
    // #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
    async fn collect<B: FromAsyncIterator<Self::Item>>(self) -> B
    where
        Self: Sized,
    {
        let fut = <B as FromAsyncIterator<_>>::from_iter(self);
        fut.await
    }
}

/// Conversion into an [`Iterator`].

pub trait IntoAsyncIterator {
    /// The type of the elements being iterated over.
    type Item;

    /// Which kind of iterator are we turning this into?
    type IntoIter: AsyncIterator<Item = Self::Item>;

    /// Creates an iterator from a value.
    async fn into_iter(self) -> Self::IntoIter;
}

impl<I: AsyncIterator> IntoAsyncIterator for I {
    type Item = I::Item;
    type IntoIter = I;

    async fn into_iter(self) -> I {
        self
    }
}

/// Conversion from an [`Iterator`].

pub trait FromAsyncIterator<A>: Sized {
    /// Creates a value from an iterator.
    async fn from_iter<T: IntoAsyncIterator<Item = A>>(iter: T) -> Self;
}

impl<T> FromAsyncIterator<T> for Vec<T> {
    async fn from_iter<I: IntoAsyncIterator<Item = T>>(iter: I) -> Vec<T> {
        let mut iter = iter.into_iter().await;
        let mut output = Vec::with_capacity(iter.size_hint().1.unwrap_or_default());
        while let Some(item) = iter.next().await {
            output.push(item);
        }
        output
    }
}

/// Extend a collection with the contents of an iterator.

pub trait AsyncExtend<A> {
    /// Extends a collection with the contents of an iterator.
    async fn extend<T: IntoAsyncIterator<Item = A>>(&mut self, iter: T);
}

impl<T> AsyncExtend<T> for Vec<T> {
    async fn extend<I: IntoAsyncIterator<Item = T>>(&mut self, iter: I) {
        let mut iter = iter.into_iter().await;
        self.reserve(iter.size_hint().1.unwrap_or_default());
        while let Some(item) = iter.next().await {
            self.push(item);
        }
    }
}

/// An iterator that maps value of another stream with a function.
#[derive(Debug)]
pub struct AsyncMap<I, F> {
    stream: I,
    f: F,
}

impl<I, F, B, Fut> AsyncIterator for AsyncMap<I, F>
where
    I: AsyncIterator,
    F: FnMut(I::Item) -> Fut,
    Fut: Future<Output = B>,
{
    type Item = B;

    async fn next(&mut self) -> Option<Self::Item> {
        let item = self.stream.next().await?;
        let out = (self.f)(item).await;
        Some(out)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
        #[allow(unused)]
        async fn foo(iter: impl AsyncIterator<Item = u32>) {
            let v: Vec<_> = iter.collect().await;
        }
    }
}
