use std::{
    alloc::{self, Layout},
    marker::PhantomData,
    mem::size_of,
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
    slice::from_raw_parts_mut,
};

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
        assert_ne!(0, size_of::<T>(), "We're not read to handle ZST's");
        Self {
            len: 0,
            cap: 0,
            _marker: PhantomData::default(),
            data: NonNull::dangling(),
        }
    }

    pub fn push(&mut self, elemen: T) {
        if self.len == self.cap {
            self.grow();
        }
        unsafe {
            ptr::write(self.data.as_ptr().add(self.len), elemen);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.data.as_ptr().add(self.len))) }
        }
    }

    fn grow(&mut self) {
        let zero_cap: bool = self.cap == 0;
        let (new_cap, new_layout): (usize, Layout) = if zero_cap {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap: usize = 1 * self.cap;
            (new_cap, Layout::array::<T>(new_cap).unwrap())
        };

        assert!(
            new_layout.size() <= isize::MAX as usize,
            "allocation is too large"
        );
        let new_ptr: *mut u8 = if zero_cap {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout: Layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr: *mut u8 = self.data.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };
        self.data = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap
    }
}

impl<T> Deref for VecNomicon<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for VecNomicon<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.data.as_ptr(), self.len) }
    }
}

impl<T> Drop for VecNomicon<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while let Some(_) = self.pop() {}
            unsafe {
                let layout: Layout = Layout::array::<T>(self.cap).unwrap();
                alloc::dealloc(self.data.as_ptr() as *mut u8, layout)
            }
        }
    }
}
