use crate::lib::ArcNomicon;

mod lib;

#[derive(Debug)]
struct Foo {
    bar: i32,
}

fn main() {
    let foo = ArcNomicon::new(Foo { bar: 1 });
    ops(ArcNomicon::clone(&foo));
    println!("bar = {bar}", bar = foo.bar)
}

fn ops(foo: ArcNomicon<Foo>) {
    println!("bar = {}", foo.bar);
}
