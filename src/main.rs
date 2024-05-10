use genasaynciter::sync::IntoStream;

pub fn print_all(i: IntoStream) {
    let i = i.into_stream();
}

fn main() {
    println!("Hello, world!");
}
