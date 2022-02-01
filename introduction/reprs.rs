use std::mem::size_of;


#[repr(packed)] //this remove memory align
struct Maybe8Bytes {
    a: u16,
    b: u32,
}

enum MyOption<T> {
    Some(T),
    None
}

#[repr(C)]
enum MyReprOption<T> {
    Some(T),
    None
}

fn main() {
    let size = size_of::<Maybe8Bytes>();
    println!("8 bytes? NO {}", size);
    let size = size_of::<MyOption<&u16>>();
    println!("Size? {}", size);
    let size = size_of::<MyReprOption<&u16>>();
    println!("Repr size? {}", size);
}
