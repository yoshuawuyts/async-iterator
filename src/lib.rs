//! An async version of iterator.
//!
//! This crate provides the following capabilities:
//!
//! - The base async `Iterator` trait implemented with `async fn next`
//! - The ability to `collect` into a `vec`
//! - The ability to asynchronously `map` over values in the iterator
//! - The ability to extend `vec` with an async iterator
//!
//! # Flavors
//!
//! There are four flavors of iterator checked into this repository:
//!
//! |               | not pinned      | pinned (self-referential) |
//! | ------------- | ----------      | ---------------           |
//! | **not async** | `Iterator`      | `PinnedIterator`          |
//! | **async**     | `AsyncIterator` | `AsyncPinnedIterator`     |
//!
//! In order for generator functions to hold references across `yield` points,
//! we need access to self-referential iterators. This means introducing pinned variants
//! of `Iterator` for both async and non-async Rust.
//!
//! Once we including "lending" as an effect too, this table will duplicate once
//! again based on whether the returned item can borrow from `self` or not.
//!
//! # Minimum Supported Rust Version
//!
//! This code should be considered _unstable_ and only works on recent versions
//! of nightly.
//!
//!
#![allow(incomplete_features)]
#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]
#![forbid(unsafe_code, future_incompatible)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, unreachable_pub)]

// /// The `async-iterator` prelude
// pub mod prelude {
//     pub use super::async_iter::{AsyncExtend, AsyncIterator, FromAsyncIterator, IntoAsyncIterator};
// }

pub mod async_iter;
pub mod async_pinned_iter;
pub mod iter;
pub mod pinned_iter;
