use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
};

fn main() {
    let data = vec![0, 1, 2, 3];
    let idx = Arc::new(AtomicUsize::new(0));
    let other_idx = idx.clone();
    thread::spawn(move || {
        other_idx.fetch_add(10, Ordering::SeqCst);
    });
    if idx.load(Ordering::SeqCst) < data.len() {
        unsafe { println!("{}", data[idx.load(Ordering::SeqCst)]) } //gwt_unchecked "only wortks" in unsafe block
    }
}
