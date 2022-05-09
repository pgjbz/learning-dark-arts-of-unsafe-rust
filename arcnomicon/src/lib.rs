use std::{
    marker::PhantomData,
    ops::Deref,
    ptr::NonNull,
    rc::Rc,
    sync::atomic::{self, AtomicUsize, Ordering},
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

impl<T> Clone for ArcNomicon<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.ptr.as_ref() };
        let old_rc = inner.rc.fetch_add(1, Ordering::Relaxed);

        if old_rc >= isize::MAX as usize {
            std::process::abort();
        }
        Self {
            ptr: self.ptr,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for ArcNomicon<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.ptr.as_ref() };

        if inner.rc.fetch_sub(1, Ordering::Release) != 1 {
            return;
        }
        atomic::fence(Ordering::Acquire);
        unsafe {
            Box::from_raw(self.ptr.as_ptr());
        }
    }
}
