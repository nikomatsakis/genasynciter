pub trait Stream {
    type Item;

    fn exec(self, op: impl FnMut(Self::Item) -> std::ops::ControlFlow<()>);

    fn for_each(self, mut op: impl FnMut(Self::Item))
    where
        Self: Sized,
    {
        self.exec(|item| {
            op(item);
            std::ops::ControlFlow::Continue(())
        })
    }
}

pub trait IntoStream {
    type Item;

    fn into_stream(self) -> impl Stream<Item = Self::Item>;
}

impl<S: Stream> IntoStream for S {
    type Item = S::Item;

    fn into_stream(self) -> impl Stream<Item = Self::Item> {
        self
    }
}
