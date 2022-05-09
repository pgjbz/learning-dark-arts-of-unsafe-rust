#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("Hello Rust!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
