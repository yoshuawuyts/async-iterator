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
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(async_fn_in_trait)]
#![forbid(unsafe_code, future_incompatible)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs)]

mod extend;
mod from_iterator;
mod into_iterator;
mod iter;
mod lending_iter;

pub use from_iterator::FromIterator;
pub use into_iterator::IntoIterator;
pub use lending_iter::LendingIterator;

pub use iter::{Iterator, Lend, LendMut, Map};

/// The `async-iterator` prelude
pub mod prelude {
    pub use crate::extend::Extend;
    pub use crate::from_iterator::FromIterator;
    pub use crate::into_iterator::IntoIterator;
}

#[cfg(feature = "alloc")]
extern crate alloc as std;

#[cfg(test)]
mod test {
    pub use super::*;

    #[test]
    fn smoke() {
        #[allow(dead_code)]
        async fn foo(iter: impl Iterator<Item = u32>) {
            let _v: Vec<_> = iter.collect().await;
        }
    }
}
