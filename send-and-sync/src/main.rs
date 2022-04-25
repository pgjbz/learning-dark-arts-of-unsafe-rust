use std::{
    mem::{self, align_of, size_of},
    ops::{Deref, DerefMut},
    ptr,
};

struct MyBox(*mut i8);

/*
Send and sync are automatic derived, but
raw pointers don't automatic derive this tratis
*/
unsafe impl Send for MyBox {}
unsafe impl Sync for MyBox {}

pub mod libc {
    pub use ::std::os::raw::{c_int, c_void};
    #[allow(non_camel_case_types)]
    pub type size_t = usize;
    extern "C" {
        pub fn posix_memalign(memptr: *mut *mut c_void, align: size_t, size: size_t) -> c_int;
        pub fn free(p: *mut c_void);
    }
}

struct Carton<T>(ptr::NonNull<T>);

impl<T> Carton<T> {
    pub fn new(data: T) -> Self {
        assert_ne!(
            mem::size_of::<T>(),
            0,
            "Zero sized types are not allowed for this"
        );
        let mut mem_ptr: *mut T = ptr::null_mut();
        unsafe {
            let ret = libc::posix_memalign((&mut mem_ptr).cast(), align_of::<T>(), size_of::<T>());
            assert_eq!(ret, 0, "Fail to alloc memory or invalid align");
        }
        let ptr = {
            ptr::NonNull::new(mem_ptr).expect("Guaranteed non-null if posix_memalign returns 0")
        };
        unsafe {
            ptr.as_ptr().write(data);
        }
        Self(ptr)
    }
}

unsafe impl<T> Send for Carton<T> where Box<T>: Send {}
unsafe impl<T> Sync for Carton<T> where T: Sync {}

impl<T> Deref for Carton<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<T> Drop for Carton<T> {
    fn drop(&mut self) {
        unsafe { libc::free(self.0.as_ptr().cast()) }
    }
}

impl<T> DerefMut for Carton<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

struct Foo {
    bar: i32,
}

fn main() {
    Carton::new(Foo { bar: 10 });
}

