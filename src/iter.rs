//! Iterator support.

/// An interface for dealing with iterators.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub trait Iterator {
    /// The type of the elements being iterated over.
    type Item;

    /// Advances the iterator and returns the next value.
    fn next(&mut self) -> Option<Self::Item>;

    /// Returns the bounds on the remaining length of the iterator.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Takes a closure and creates an iterator which calls that closure on each element.
    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        Map { stream: self, f }
    }

    /// Transforms an iterator into a collection.
    // #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
    fn collect<B: FromIterator<Self::Item>>(self) -> B
    where
        Self: Sized,
    {
        let fut = <B as FromIterator<_>>::from_iter(self);
        fut
    }
}

/// Conversion into an [`Iterator`].

pub trait IntoIterator {
    /// The type of the elements being iterated over.
    type Item;

    /// Which kind of iterator are we turning this into?
    type IntoIter: Iterator<Item = Self::Item>;

    /// Creates an iterator from a value.
    fn into_iter(self) -> Self::IntoIter;
}

impl<I: Iterator> IntoIterator for I {
    type Item = I::Item;
    type IntoIter = I;

    fn into_iter(self) -> I {
        self
    }
}

/// Conversion from an [`Iterator`].

pub trait FromIterator<A>: Sized {
    /// Creates a value from an iterator.
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
}

impl<T> FromIterator<T> for Vec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Vec<T> {
        let mut iter = iter.into_iter();
        let mut output = Vec::with_capacity(iter.size_hint().1.unwrap_or_default());
        while let Some(item) = iter.next() {
            output.push(item);
        }
        output
    }
}

/// Extend a collection with the contents of an iterator.

pub trait Extend<A> {
    /// Extends a collection with the contents of an iterator.
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T);
}

impl<T> Extend<T> for Vec<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let mut iter = iter.into_iter();
        self.reserve(iter.size_hint().1.unwrap_or_default());
        while let Some(item) = iter.next() {
            self.push(item);
        }
    }
}

/// An iterator that maps value of another stream with a function.
#[derive(Debug)]
pub struct Map<I, F> {
    stream: I,
    f: F,
}

impl<I, F, B> Iterator for Map<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.stream.next()?;
        let out = (self.f)(item);
        Some(out)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
        #[allow(unused)]
        fn foo(iter: impl Iterator<Item = u32>) {
            let v: Vec<_> = iter.collect();
        }
    }
}
