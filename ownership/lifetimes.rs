#[derive(Debug)]
struct Xi<'a>(&'a i32);


impl Drop for Xi<'_> {
    fn drop(&mut self) {}
}

fn main() {
    let mut vector = vec![1,2,3,4];
    //let x = &vector[0];
    //vector.push(5); //not ok
    //println!("{}", x);
    //vector.push(5); //it's ok

    let x = Xi(&vector[0]);
    println!("{:?}", x);
    /*
     * only vector.push(5) only compile if use drop(x); 
     * Error is because x call drop method
     * */
    drop(x);
    vector.push(5); //ok if call drop method

    let mut data = vec![1,2,3,4];
    let mut x = &data[0]; //this is mut because we change x reference
    println!("x = {}", x);
    data.push(5);  //mutabble reference, this ref, make compile fails if try to print x before "re-pointer" x
    x = &data[0];  //change x reference, and after this, has success to print x
    println!("x = {}", x);
}
