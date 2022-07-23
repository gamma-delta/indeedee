#![doc = include_str!("../README.md")]

use std::time::{Duration, Instant};

pub struct ProgressiveWaiter<L, I>
where
    L: Loader,
    I: Iterator<Item = L::Data>,
{
    data: I,
    loader: Option<L>,
    count: usize,
}

impl<L, I> ProgressiveWaiter<L, I>
where
    L: Loader,
    I: Iterator<Item = L::Data>,
{
    pub fn new(loader: L, data: I) -> Self {
        Self {
            data,
            loader: Some(loader),
            count: 0,
        }
    }

    pub fn query(&mut self, allowed_time: Duration, context: &L::Context) -> LoadResult<L> {
        let loader = match self.loader {
            Some(ref mut loader) => loader,
            None => panic!("tried to query after the loader was done."),
        };

        let start_time = Instant::now();

        while let Some(next) = self.data.next() {
            self.count += 1;
            let update = loader.operate(next, &context);

            if start_time.elapsed() >= allowed_time {
                return LoadResult::Loading(update);
            }
        }

        let output = self.loader.take().unwrap().finish(&context);
        LoadResult::Done(output)
    }

    /// How many elements we've processed.
    pub fn finished_count(&self) -> usize {
        self.count
    }

    pub fn loader(&self) -> &L {
        self.loader.as_ref().unwrap()
    }

    pub fn loader_mut(&mut self) -> &mut L {
        self.loader.as_mut().unwrap()
    }
}

impl<L, I> ProgressiveWaiter<L, I>
where
    L: Loader,
    I: ExactSizeIterator<Item = L::Data>,
{
    /// How many total elements there are to process (including ones we already did).
    pub fn total_elements(&self) -> usize {
        self.count + self.data.len()
    }

    /// How many elements there are left to process.
    pub fn elements_left(&self) -> usize {
        self.data.len()
    }

    /// How far we are through, as a proportion from 0 to 1.
    pub fn progress(&self) -> f64 {
        self.count as f64 / self.total_elements() as f64
    }
}

/// Something that handles a stream of data that might take a long time.
pub trait Loader {
    /// Type this operates over.
    type Data;

    /// An update on what the loader is currently working. This is just for informative purposes, if you care about that kind of thing...
    /// but you can also just use `()`.
    type ProgressUpdate;

    /// What this produces when it's done.
    type Output;

    /// Any context this might need to do its work.
    type Context;

    /// Perform an update on one piece of data.
    fn operate(&mut self, data: Self::Data, ctx: &Self::Context) -> Self::ProgressUpdate;

    /// Once this is all through, consume itself and return the thing it's been building towards.
    fn finish(self, ctx: &Self::Context) -> Self::Output;
}

/// Whether we've finished with this loader.
#[derive(Debug)]
pub enum LoadResult<L: Loader> {
    /// We have not finished loading, and here's a progress update
    Loading(L::ProgressUpdate),
    /// We're done, and here's the output!
    Done(L::Output),
}
