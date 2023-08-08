use crate::Iterator;
use crate::LendingIterator;

/// The iterator returned from `AsyncIterator::lend`.
#[derive(Debug)]
pub struct LendMut<I: Iterator>(I);

impl<I: Iterator> LendMut<I> {
    pub(crate) fn new(i: I) -> Self {
        Self(i)
    }
}

impl<I: Iterator> LendingIterator for LendMut<I> {
    type Item<'a> = (&'a mut I, I::Item)
    where
        Self: 'a;

    async fn next(&mut self) -> Option<Self::Item<'_>> {
        let item = self.0.next().await;
        item.map(move |item| (&mut self.0, item))
    }
}
