struct MySuperSlice<T: ?Sized> { //use ?Sized permits to use non-sizeble structs
    info: u32,
    data: T
}

fn main() {
    let super_slice = MySuperSlice {
        info: 16,
        data: [1,2,3,4,5]
    };
    println!("{} {:?}", super_slice.info, super_slice.data);
}


/*struct MySuperSlice {
    info: u32,
    data: [u8]
}

fn main() {
    let arr = [1,2,3,4,5];
    let super_slice = MySuperSlice {
        info: 16,
        data: arr[..]
    };
    println!("{} {:?}", super_slice.info, super_slice.data);
}
  --> data-layout/exotically_sized_types.rs:8:23
   |
8  |       let super_slice = MySuperSlice {
   |  _______________________^
9  | |         info: 16,
10 | |         data: arr[..]
11 | |     };
   | |_____^ doesn't have a size known at compile-time

*/
