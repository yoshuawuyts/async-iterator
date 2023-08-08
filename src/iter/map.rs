use crate::Iterator;
use core::future::Future;

/// An iterator that maps value of another stream with a function.
#[derive(Debug)]
pub struct Map<I, F> {
    stream: I,
    f: F,
}

impl<I, F> Map<I, F> {
    pub(crate) fn new(stream: I, f: F) -> Self {
        Self { stream, f }
    }
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
