//! An example using async "poll traits" which use manual "poll" functions
//! to create async functions.
//!
//! The Rust stdlib doesn't provide a way to run futures to completion,
//! so we have to define our own. To make the output easier to compare,
//! we create a waker which doesn't use any atomics.

#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![feature(const_waker)]

// use async_iterator::prelude::*;
use futures::stream::StreamExt;
use futures_core::Stream as PollingIterator;
use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::{pin, Pin};
use std::ptr;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/////////////////////////////// Testing:

fn main() {
    call_once();
}

pub fn call_once() -> Poll<Option<usize>> {
    let mut iter = pin!(once(12usize));
    let output = poll_once(iter.next());
    output
}

/////////////////////// Implementation:
#[derive(Clone, Debug)]
pub struct PollingOnce<T> {
    value: Option<T>,
    _phantom: PhantomPinned,
}

impl<T> PollingIterator for PollingOnce<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = unsafe { Pin::into_inner_unchecked(self) };
        Poll::Ready((&mut this.value).take())
    }
}

////////////////////////////////////////// Wrappers:
#[derive(Clone, Debug)]
pub struct Once<T: Unpin> {
    inner: PollingOnce<T>,
}

impl<T: Unpin> Iterator for Once<T> {
    type Item = T;

    async fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().await
    }
}

/// An async iterator which yields an item once
pub fn once<T>(t: T) -> PollingOnce<T> {
    PollingOnce {
        value: Some(t),
        _phantom: Default::default(),
    }
}

////////////////////////////////////////// Utilities:

/// Poll a non-Send future exactly once.
fn poll_once<T>(f: impl Future<Output = T>) -> Poll<T> {
    const WAKER: &Waker = {
        const RAW: RawWaker = {
            RawWaker::new(
                ptr::null(),
                &RawWakerVTable::new(no_clone, no_wake, no_wake, no_drop),
            )
        };
        fn no_clone(_: *const ()) -> RawWaker {
            RAW
        }
        fn no_wake(_: *const ()) {}
        fn no_drop(_: *const ()) {}
        &unsafe { Waker::from_raw(RAW) }
    };
    let mut ctx = Context::from_waker(&WAKER);
    pin!(f).poll(&mut ctx)
}
