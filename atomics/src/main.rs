use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

fn main() {
    let lock = Arc::new(AtomicBool::new(false));

    while lock
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire)
        .unwrap()
    {}
    lock.store(false, Ordering::Release);
    println!("{:?}", lock);
}
