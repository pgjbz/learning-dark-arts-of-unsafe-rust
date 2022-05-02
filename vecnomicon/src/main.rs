use crate::vecnomicon::VecNomicon;

mod vecnomicon;

fn main() {
    let mut vecnom: VecNomicon<i32> = VecNomicon::new();
    vecnom.push(10);
    vecnom.push(20);
    vecnom.push(30);
    vecnom.push(40);
    vecnom.insert(2, 25);
    vecnom.remove(3);

    println!("drain");

    for aa in vecnom.drain() {
        println!("{aa}");
    }

    vecnom.push(10);
    vecnom.push(20);

    println!("iter");

    for a in vecnom.iter() {
        println!("{}", a)
    }
    println!("into iter");
    for b in vecnom.into_iter() {
        println!("{}", b);
    }
}
