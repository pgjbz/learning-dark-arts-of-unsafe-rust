use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

pub fn main() {
    let data = vec![1, 2, 3, 4];
    let idx = Arc::new(AtomicUsize::new(0));
    let other_idx = idx.clone();
    thread::spawn(move || {
        other_idx.fetch_add(10, Ordering::SeqCst);
    }); //if join this thread, the program will panic, because idx turn value equals 10

    println!("{}", data[idx.load(Ordering::SeqCst)]);
}
