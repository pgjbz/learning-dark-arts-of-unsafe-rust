use std::fmt::Display;

fn index<T>(idx: usize, arr: &[T]) -> Option<T>
where
    T: Display + Copy,
{
         unsafe { Some(*arr.get_unchecked(idx)) }//unsafe block, thats possible to call null memory, or something else
}

fn main() {
    let numbers: Vec<i8> = vec![0,1,2,3,4,5,6];
    println!("{}", if let Some(x) = index(10, &numbers) { 
        x 
    } else {
        0
    });
}
