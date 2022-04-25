use std::{marker::PhantomData, mem::size_of, ptr::NonNull};

pub struct VecNomicon<T> {
    data: NonNull<T>,
    len: usize,
    cap: usize,
    _marker: PhantomData<T>,
}

unsafe impl<T> Send for VecNomicon<T> where T: Send {}
unsafe impl<T> Sync for VecNomicon<T> where T: Sync {}

impl<T> VecNomicon<T> {
    pub fn new() -> Self {
        assert_eq!(0, size_of::<T>(), "We're not read to handle ZST's");
        Self {
            len: 0,
            cap: 0,
            _marker: PhantomData::default(),
            data: NonNull::dangling(),
        }
    }
}
