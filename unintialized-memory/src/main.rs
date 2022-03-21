fn main() {
    let mut x = Box::new(1); //x uninit, overwrite
    let mut y = x; //y unitnit, drop x, overwrite y
    y = Box::new(0); //drop y, rewrite y
    x = Box::new(4); //x uninit, overwrite x
    println!("x = {x}, y = {y}");
}
