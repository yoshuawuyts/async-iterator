/// An async iterator that filters the elements of `iter` with `predicate`.
///
/// This `struct` is created by the [`filter`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`filter`]: crate::Iterator::filter
/// [`Iterator`]: crate::Iterator
#[derive(Debug)]
pub struct Filter<I, P>
where
    I: crate::Iterator,
    P: async FnMut(&I::Item) -> bool,
{
    iter: I,
    predicate: P,
}

impl<I, P> crate::Iterator for Filter<I, P>
where
    I: crate::Iterator,
    P: async FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    async fn next(&mut self) -> Option<Self::Item> {
        loop {
            let item = self.iter.next().await?;
            if (self.predicate)(&item).await {
                return Some(item);
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }
}

impl<I, P> Filter<I, P>
where
    I: crate::Iterator,
    P: async FnMut(&I::Item) -> bool,
{
    pub(crate) fn new(stream: I, predicate: P) -> Self {
        Self {
            iter: stream,
            predicate,
        }
    }
}
