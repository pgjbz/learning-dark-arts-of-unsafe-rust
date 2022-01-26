struct Foo {
    _a: i16,
    _b: i32,
}

/*
 * The struct above has 8bytes, because Rust compiler align a field with b field
 * */

fn main() {
    let size: usize = std::mem::size_of::<Foo>();
    println!("Size of foo: {}", size)
}
