use std::{
    marker::PhantomData,
    ops::Deref,
    ptr::NonNull,
    sync::atomic::{self, AtomicUsize},
};

pub struct ArcNomicon<T> {
    ptr: NonNull<ArcNomiconIner<T>>,
    _marker: PhantomData<ArcNomiconIner<T>>,
}

pub struct ArcNomiconIner<T> {
    rc: atomic::AtomicUsize,
    data: T,
}

unsafe impl<T> Send for ArcNomicon<T> where T: Send + Sync {}
unsafe impl<T> Sync for ArcNomicon<T> where T: Send + Sync {}

impl<T> ArcNomicon<T> {
    pub fn new(data: T) -> Self {
        let boxed = Box::new(ArcNomiconIner {
            rc: AtomicUsize::new(1),
            data,
        });

        Self {
            ptr: NonNull::new(Box::into_raw(boxed)).unwrap(),
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for ArcNomicon<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let inner = unsafe { self.ptr.as_ref() };
        &inner.data
    }
}
