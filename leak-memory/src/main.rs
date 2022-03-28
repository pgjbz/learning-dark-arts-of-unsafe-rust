use std::mem;

#[derive(Clone, Debug)]
struct Foo {
    bar: u8,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("the value is {val}", val = self.bar);
    }
}

fn main() {
    let mut vec = vec![Box::new(Foo { bar: 1 }); 4];
    {
        let mut drain = vec.drain(..);
        drain.next();
        drain.next();
        mem::forget(drain);
    }
    println!("value = {:?}", vec[0]);
}

/*
    in this example only 2 items of vector execute drop method
    mem::forget can leak memory in Rc and other ways
*/
