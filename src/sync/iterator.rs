use std::ops::ControlFlow;

use super::stream::Stream;

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

pub trait IntoIterator {
    type Item;

    fn into_iter(self) -> impl Iterator<Item = Self::Item>;
}

impl<G> Stream for G
where
    G: Iterator,
{
    type Item = G::Item;

    fn exec(mut self, mut op: impl FnMut(Self::Item) -> ControlFlow<()>) {
        while let Some(elem) = self.next() {
            match op(elem) {
                ControlFlow::Break(()) => return,
                ControlFlow::Continue(()) => (),
            }
        }
    }
}
