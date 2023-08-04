//! Concurrent async iterator support

use std::{future::Future, marker::PhantomData};

/// A concurrent iterator
pub trait ConcurrentIterator {
    /// What kind of item are we yielding?
    type Item;

    /// Internal function to drive the progress of the concurrent iterator
    fn drive<C>(&mut self, consumer: C)
    where
        C: Consumer<Self::Item>;

    /// Calls a closure on each element of an iterator.
    async fn for_each<F>(self)
    where
        Self: Sized,
        F: FnMut(Self::Item),
    {
    }

    // /// Takes a closure and creates an iterator which calls that closure on each element.
    // fn map<B, F>(self, f: F) -> Map<Self, F>
    // where
    //     Self: Sized,
    //     F: FnMut(Self::Item) -> B,
    // {
    //     Map { iter: self, f }
    // }
}

struct FoldConsumer<T> {}

/// Consume the items
pub trait Consumer<T> {
    /// What kind of item are we yielding?
    type Output;
    /// Consume an input and produce an output.
    async fn next(&mut self, input: T) -> Self::Output;
}

// /// An iterator that maps value of another stream with a function.
// #[derive(Debug)]
// pub struct Map<I, F> {
//     iter: I,
//     f: F,
// }

// impl<I, F, B, Fut> ConcurrentIterator for Map<I, F>
// where
//     I: ConcurrentIterator,
//     F: FnMut(I::Item) -> Fut,
//     Fut: Future<Output = B>,
// {
//     type Item = B;

//     fn drive<C>(&mut self, consumer: C)
//     where
//         C: Consumer<Self::Item>,
//     {
//         let map_consumer = MapConsumer(self, PhantomData);
//         self.iter.drive(consumer);
//         todo!()
//     }
// }

// struct MapConsumer<'a, I, F, B>(&'a mut Map<I, F>, PhantomData<B>);
// impl<'a, I, F, B> Consumer<B> for MapConsumer<'a, I, F, B> {
//     type Output = B;

//     async fn next(&mut self, input: I::Item) -> Self::Output {
//         todo!()
//     }
// }
