use std::fmt::Display;

use gensynciter::{IntoStream, Stream};

pub fn print_all<T: Display>(i: impl IntoStream<Item = T>) {
    i.into_stream().for_each(|i| {
        println!("{i}");
    });
}

fn main() {
    print_all(gensynciter::gen_range(0, 5));
}
