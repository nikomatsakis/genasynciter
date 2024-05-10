pub trait Stream {
    type Item;

    fn exec(self, op: impl FnMut(Self::Item) -> std::ops::ControlFlow<()>);
}

pub trait IntoStream {
    type Item;

    fn into_stream(self) -> impl Stream<Item = Self::Item>;
}
