use std::{marker::PhantomData, ptr::NonNull};

pub struct VecNomicon<T> {
    data: NonNull<T>,
    len: usize,
    cap: usize,
    _marker: PhantomData<T>,
}

unsafe impl<T> Send for VecNomicon<T> where T: Send {}
unsafe impl<T> Sync for VecNomicon<T> where T: Sync {}
