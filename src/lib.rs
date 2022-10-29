//! An async version of iterator.
//!
//! This crate provides the following capabilities:
//!
//! - The base async `Iterator` trait implemented with `async fn next`
//! - The ability to `collect` into a `vec`
//! - The ability to asynchronously `map` over values in the iterator
//! - The ability to extend `vec` with an async iterator
//! 
//! # Minimum Supported Rust Version
//! 
//! This code should be considered _unstable_ and only works on recent versions
//! of nightly.
//!
//! # Trait definitions
//!
//! All traits make use of the `async_trait` annotation. In order to implement
//! the traits, use `async_trait`.
//!
#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]
#![forbid(unsafe_code, future_incompatible)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, unreachable_pub)]

/// `async-trait` re-export
use std::future::Future;

/// The `async-iterator` prelude
pub mod prelude {
    pub use super::{Extend, FromIterator, IntoIterator, Iterator};
}

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
    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        Map { stream: self, f }
    }

    /// Transforms an iterator into a collection.
    // #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
    async fn collect<B: FromIterator<Self::Item>>(self) -> B
    where
        Self: Sized,
    {
        let fut = <B as FromIterator<_>>::from_iter(self);
        fut.await
    }
}

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

/// Conversion from an [`Iterator`].

pub trait FromIterator<A>: Sized {
    /// Creates a value from an iterator.
    async fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
}

impl<T> FromIterator<T> for Vec<T> {
    async fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Vec<T> {
        let mut iter = iter.into_iter().await;
        let mut output = Vec::with_capacity(iter.size_hint().1.unwrap_or_default());
        while let Some(item) = iter.next().await {
            output.push(item);
        }
        output
    }
}

/// Extend a collection with the contents of an iterator.

pub trait Extend<A> {
    /// Extends a collection with the contents of an iterator.
    async fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T);
}

impl<T> Extend<T> for Vec<T> {
    async fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let mut iter = iter.into_iter().await;
        self.reserve(iter.size_hint().1.unwrap_or_default());
        while let Some(item) = iter.next().await {
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

impl<I, F, B, Fut> Iterator for Map<I, F>
where
    I: Iterator,
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
