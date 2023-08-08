use crate::{Iterator, LendingIterator};

/// The iterator returned from `AsyncIterator::lend`.
#[derive(Debug)]
pub struct Lend<I: Iterator>(I);

impl<I: Iterator> Lend<I> {
    pub(crate) fn new(i: I) -> Self {
        Self(i)
    }
}

impl<I: Iterator> LendingIterator for Lend<I> {
    type Item<'a> = (&'a I, I::Item)
    where
        Self: 'a;

    async fn next(&mut self) -> Option<Self::Item<'_>> {
        let item = self.0.next().await;
        item.map(move |item| (&self.0, item))
    }
}
