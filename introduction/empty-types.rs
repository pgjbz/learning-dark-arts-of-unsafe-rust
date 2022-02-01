
//This is a empty type
enum Void {}


fn main() {
    let res: Result<u32, Void> = Ok(0);

    // Err doesn't exist anymore, so Ok is actually irrefutable.
    if let Ok(num) = res {
        println!("{}", num);
    }
}
